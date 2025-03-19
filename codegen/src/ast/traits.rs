// TODO clean up imports manually. The built in detection doesn't work well across include!()s
#![allow(unused_imports)]
#![allow(non_upper_case_globals)]

use async_trait::async_trait;
use indicatif::{MultiProgress, ProgressFinish};
use lazy_static::lazy_static;
use parking_lot::{Mutex, RwLock};
use regex::Regex;
use std::borrow::Cow;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::sync::{Arc, Weak};
use tokio::task::JoinSet;
use tracing::Level;
use tracing::level_filters::LevelFilter;
use crate::algebra::basis::{BasisElement, BasisSignature};
use crate::algebra::multivector::{DynamicMultiVector, MultiVecRepository};
use crate::algebra::GeometricAlgebra;
use crate::ast::datatype::{ClassesFromRegistry, ExpressionType, Float, Integer, MultiVector};
use crate::ast::expressions::{extract_float_expr, extract_integer_expr, extract_multivector_expr, AnyExpression, DestructurableVariables, Expression, FloatExpr, IntExpr, MultiVectorExpr, MultiVectorGroupExpr, MultiVectorVia, TraitResultType, Vec2Expr, Vec3Expr, Vec4Expr};
use crate::ast::impls::{Elaborated, InlineOnly, OvertDelegate};
use crate::ast::operations_tracker::{TrackOperations, TraitOperationsLookup, VectoredOperationsTracker};
use crate::ast::{RawVariableDeclaration, RawVariableInvocation, Variable};
use crate::utility::AsyncMap;
use crate::utility::tracing::DebuggableCopyPasta;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TraitTypeConsensus {
    NoVotes,
    AllAgree(ExpressionType, bool),
    AlwaysSelf,
    Disagreement,
}
impl TraitTypeConsensus {
    pub fn add_vote(slf: &Arc<RwLock<TraitTypeConsensus>>, expr_type: ExpressionType, is_self: bool) {
        let output = slf.read();
        match output.deref() {
            TraitTypeConsensus::Disagreement => return,
            TraitTypeConsensus::AlwaysSelf if is_self => return,
            TraitTypeConsensus::AllAgree(agreed, was_self) if *agreed == expr_type && is_self == *was_self => return,
            _ => {}
        }
        drop(output);
        let mut owners = slf.write();
        *owners = match owners.deref() {
            TraitTypeConsensus::Disagreement => return,
            TraitTypeConsensus::AlwaysSelf if is_self => return,
            TraitTypeConsensus::AlwaysSelf => TraitTypeConsensus::Disagreement,
            TraitTypeConsensus::NoVotes => TraitTypeConsensus::AllAgree(expr_type, is_self),
            TraitTypeConsensus::AllAgree(agreed_type, was_self) => match (*agreed_type == expr_type, *was_self && is_self) {
                (true, true) => return,
                (true, false) => TraitTypeConsensus::AllAgree(expr_type, false),
                (false, true) => TraitTypeConsensus::AlwaysSelf,
                (false, false) => TraitTypeConsensus::Disagreement,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Param {
    TypeParam,
    DataParam,
    TypeAndDataParam,
}
impl Param {
    fn is_type_param(&self) -> bool {
        match self {
            Param::TypeParam => true,
            Param::DataParam => false,
            Param::TypeAndDataParam => true,
        }
    }
    fn is_data_param(&self) -> bool {
        match self {
            Param::TypeParam => false,
            Param::DataParam => true,
            Param::TypeAndDataParam => true,
        }
    }
}

pub(crate) type TraitParam = (ExpressionType, Param);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TraitArity {
    Zero,
    One,
    Two,
}
impl TraitArity {
    pub fn as_str(&self) -> &'static str {
        match self {
            TraitArity::Zero => "arity_0",
            TraitArity::One => "arity_1",
            TraitArity::Two => "arity_2",
        }
    }
}

pub struct RawTraitDefinition {
    pub(crate) documentation: String,
    pub(crate) names: TraitNames,
    pub(crate) owner: Arc<RwLock<TraitTypeConsensus>>,
    pub(crate) arity: TraitArity,
    pub(crate) output: Arc<RwLock<TraitTypeConsensus>>,
    pub(crate) op: Arc<Mutex<Option<Ops>>>,
    pub(crate) dependencies: Arc<Mutex<HashSet<TraitKey>>>,
}
pub(crate) struct RawTraitImplementation {
    pub(crate) definition: Arc<RawTraitDefinition>,
    pub(crate) owner: TraitParam,
    pub(crate) other_params: Vec<TraitParam>,
    pub(crate) lines: Vec<CommentOrVariableDeclaration>,
    pub(crate) return_comment: Option<String>,
    pub(crate) return_expr: AnyExpression,
    pub(crate) statistics: VectoredOperationsTracker,
}


include!("traits/definitions.rs");
include!("traits/names.rs");
include!("traits/ops.rs");
include!("traits/register.rs");
include!("traits/debug.rs");
include!("traits/builder.rs");




#[macro_export]
macro_rules! variants {
    (
        $declarations:ident;
        $(
        $(#docs($docs:expr))?
        $($prefix:ident)? {type} $($suffix:ident)? => ($el_filter:expr) where $mv_in:expr => $mv_out:expr
        );+
        $(;)?
    ) => {
        {
            $(
            let mut doc: std::option::Option<&'static str> = None;
            $(doc = Some($docs);)?
            $declarations.variants(
                stringify!($($prefix)?),
                stringify!($($suffix)?),
                $mv_in, $el_filter, $mv_out,
                doc,
            );
            )+
        }
    };
    (
        $declarations:ident;
        $(
        $(#docs($docs:expr))?
        $($prefix:ident)? {Vector} $($suffix:ident)? => ($el_filter:expr) where $mv_in:expr => $mv_out:expr
        );+
        $(;)?
    ) => {
        {
            $(
            let mut doc: std::option::Option<&'static str> = None;
            $(doc = Some($docs);)?
            $declarations.variants(
                stringify!($($prefix)?),
                stringify!($($suffix)?),
                $mv_in, $el_filter, $mv_out,
                doc,
            );
            )+
        }
    };
}


#[macro_export]
macro_rules! register_all {
    ( $anti_scalar:ident $mv_repo:expr; $($t:ident)+ $(| $($t2:ident)+)*) => {
        {
            // TODO handle toggling of progress bars
            let use_progress_bars = true;
            use $crate::build_scripts::common_traits::*;
            let tir = $crate::ast::traits::TraitImplRegistry::new();
            use $crate::ast::traits::{Register10, Register11, Register21, Register22, Register12f, Register12i};
            let rt = $crate::ast::traits::tokio_rt();

            let multi_progress = $crate::ast::traits::indicatif_multi_progress();
            let _: () = rt.block_on(async {
                let mut overall_pb = None;
                if use_progress_bars {
                    let opb = std::sync::Arc::new(multi_progress.add($crate::ast::traits::indicatif_progress_bar(0).with_finish($crate::ast::traits::indicatif_and_leave())));
                    opb.set_style($crate::ast::traits::progress_style());
                    opb.set_message("AST: Trait Implementations");
                    overall_pb = Some(opb);
                }
                let mut js = $crate::ast::traits::tokio_joinset();
                $(
                let tir_c = tir.clone();
                let mv_repo_c = $mv_repo.clone();
                let mp = multi_progress.clone();
                let overall_pb_2 = overall_pb.clone();
                js.spawn(async move {
                    $crate::ast::traits::RegisterTrait($t).register::<$anti_scalar>(tir_c, mv_repo_c, mp, overall_pb_2).await;
                });
                )+
                while let Some(_) = js.join_next().await {}

                $(
                let mut js = $crate::ast::traits::tokio_joinset();
                $(
                let tir_c = tir.clone();
                let mv_repo_c = $mv_repo.clone();
                let mp = multi_progress.clone();
                let overall_pb_2 = overall_pb.clone();
                js.spawn(async move {
                    $crate::ast::traits::RegisterTrait($t2).register::<$anti_scalar>(tir_c, mv_repo_c, mp, overall_pb_2).await;
                });
                )+
                while let Some(_) = js.join_next().await {}
                )*
                if let Some(opb) = overall_pb {
                    opb.finish();
                }
            });
            tir
        }
    };
}