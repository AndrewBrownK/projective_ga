use crate::algebra::basis::BasisElement;
use crate::ast::impls::Elaborated;
use crate::ast::traits::{NameTrait};
use impls::*;

// TODO carrier, cocarrier

pub const fn conformal_conjugate(infinity: BasisElement) -> Elaborated<ConformalConjugateImpl> {
    ConformalConjugateImpl { infinity }
        .new_trait_named("ConformalConjugate")
        .blurb("The conformal conjugate negates the flat elements of an object, and is \
        useful in calculating the center norm of the object.")
}

pub const fn round_bulk(origin: BasisElement, infinity: BasisElement) -> Elaborated<RoundBulkImpl> {
    RoundBulkImpl { origin, infinity }
        .new_trait_named("RoundBulk")
        .blurb("This is the aspect of a round object that characterizes the carrier's \
        relationship with the origin.")
}

pub const fn round_weight(origin: BasisElement, infinity: BasisElement) -> Elaborated<RoundWeightImpl> {
    RoundWeightImpl { origin, infinity }
        .new_trait_named("RoundWeight")
        .blurb("This is the aspect of a round object that characterizes the carrier's \
        relationship with the horizon.")
}

pub const fn flat_bulk(origin: BasisElement, infinity: Option<BasisElement>) -> Elaborated<FlatBulkImpl> {
    FlatBulkImpl { origin, infinity }
        .new_trait_named("FlatBulk")
        .blurb("This characterizes the flat aspect's relationship with the origin.")
}

pub const fn flat_weight(origin: BasisElement, infinity: Option<BasisElement>) -> Elaborated<FlatWeightImpl> {
    FlatWeightImpl { origin, infinity }
        .new_trait_named("FlatWeight")
        .blurb("This characterizes the flat aspect's relationship with the horizon.")
}

pub const fn center_norm(origin: BasisElement, infinity: BasisElement) -> Elaborated<CenterNormImpl> {
    CenterNormImpl { origin, infinity }
        .new_trait_named("CenterNorm")
        .blurb("Distance between origin and center (not yet unitized, still \
        requires division by round weight).")
}

pub const fn center_norm_squared(origin: BasisElement, infinity: BasisElement) -> Elaborated<CenterNormSquaredImpl> {
    CenterNormSquaredImpl { origin, infinity }
        .new_trait_named("CenterNormSquared")
        .blurb("Intermediate result to CenterNorm.")
}

pub const fn flat_bulk_norm(origin: BasisElement, infinity: Option<BasisElement>) -> Elaborated<FlatBulkNormImpl> {
    FlatBulkNormImpl { origin, infinity }
        .new_trait_named("FlatBulkNorm")
        .blurb("BulkNorm for flat aspect.")
}

pub const fn flat_bulk_norm_squared(origin: BasisElement, infinity: Option<BasisElement>) -> Elaborated<FlatBulkNormSquaredImpl> {
    FlatBulkNormSquaredImpl { origin, infinity }
        .new_trait_named("FlatBulkNormSquared")
        .blurb("Intermediate result for FlatBulkNorm.")
}

pub const fn flat_norm(origin: BasisElement, infinity: Option<BasisElement>) -> Elaborated<FlatNormImpl> {
    FlatNormImpl { origin, infinity }
        .new_trait_named("FlatNorm")
        .blurb("Norm for flat aspect.")
}

pub const fn flat_norm_squared(origin: BasisElement, infinity: Option<BasisElement>) -> Elaborated<FlatNormSquaredImpl> {
    FlatNormSquaredImpl { origin, infinity }
        .new_trait_named("FlatNormSquared")
        .blurb("Intermediate result to FlatNorm.")
}

pub const fn flat_weight_norm(origin: BasisElement, infinity: Option<BasisElement>) -> Elaborated<FlatWeightNormImpl> {
    FlatWeightNormImpl { origin, infinity }
        .new_trait_named("FlatWeightNorm")
        .blurb("Weight Norm for flat aspect.")
}

pub const fn flat_weight_norm_squared(origin: BasisElement, infinity: Option<BasisElement>) -> Elaborated<FlatWeightNormSquaredImpl> {
    FlatWeightNormSquaredImpl { origin, infinity }
        .new_trait_named("FlatWeightNormSquared")
        .blurb("Intermediate result to FlatWeightNorm.")
}

pub static RadiusNorm: Elaborated<RadiusNormImpl> = RadiusNormImpl
    .new_trait_named("RadiusNorm")
    .blurb("Distance radius of a round object (not yet unitized, still \
        requires division by round weight).");

pub static RadiusNormSquared: Elaborated<RadiusNormSquaredImpl> = RadiusNormSquaredImpl
    .new_trait_named("RadiusNormSquared")
    .blurb("Intermediate result to RadiusNorm.");

pub const fn round_bulk_norm(origin: BasisElement, infinity: BasisElement) -> Elaborated<RoundBulkNormImpl> {
    RoundBulkNormImpl { origin, infinity }
        .new_trait_named("RoundBulkNorm")
        .blurb("Bulk Norm for round aspect.")
}

pub const fn round_bulk_norm_squared(origin: BasisElement, infinity: BasisElement) -> Elaborated<RoundBulkNormSquaredImpl> {
    RoundBulkNormSquaredImpl { origin, infinity }
        .new_trait_named("RoundBulkNormSquared")
        .blurb("Intermediate result to RoundBulkNorm.")
}

pub const fn round_norm(origin: BasisElement, infinity: BasisElement) -> Elaborated<RoundNormImpl> {
    RoundNormImpl { origin, infinity }
        .new_trait_named("RoundNorm")
        .blurb("Norm for Round aspect.")
}

pub const fn round_norm_squared(origin: BasisElement, infinity: BasisElement) -> Elaborated<RoundNormSquaredImpl> {
    RoundNormSquaredImpl { origin, infinity }
        .new_trait_named("RoundNormSquared")
        .blurb("Intermediate result for RoundNorm.")
}

pub const fn round_weight_norm(origin: BasisElement, infinity: BasisElement) -> Elaborated<RoundWeightNormImpl> {
    RoundWeightNormImpl { origin, infinity }
        .new_trait_named("RoundWeightNorm")
        .blurb("Weight Norm for round aspect.")
}

pub const fn round_weight_norm_squared(origin: BasisElement, infinity: BasisElement) -> Elaborated<RoundWeightNormSquaredImpl> {
    RoundWeightNormSquaredImpl { origin, infinity }
        .new_trait_named("RoundWeightNormSquared")
        .blurb("Intermediate result for RoundWeight.")
}

pub const fn unitized_center_norm(origin: BasisElement, infinity: BasisElement) -> Elaborated<UnitizedCenterNormImpl> {
    UnitizedCenterNormImpl { origin, infinity }
        .new_trait_named("UnitizedCenterNorm")
        .blurb("Unitized distance from origin to center of round object.")
}

pub const fn unitized_center_norm_squared(origin: BasisElement, infinity: BasisElement) -> Elaborated<UnitizedCenterNormSquaredImpl> {
    UnitizedCenterNormSquaredImpl { origin, infinity }
        .new_trait_named("UnitizedCenterNormSquared")
        .blurb("Intermediate result to UnitizedCenterNorm.")
}

pub const fn unitized_flat_norm(origin: BasisElement, infinity: Option<BasisElement>) -> Elaborated<UnitizedFlatNormImpl> {
    UnitizedFlatNormImpl { origin, infinity }
        .new_trait_named("UnitizedFlatNorm")
        .blurb("Unitized FlatNorm.")
}

pub const fn unitized_flat_norm_squared(origin: BasisElement, infinity: Option<BasisElement>) -> Elaborated<UnitizedFlatNormSquaredImpl> {
    UnitizedFlatNormSquaredImpl { origin, infinity }
        .new_trait_named("UnitizedFlatNormSquared")
        .blurb("Intermediate result to UnitizedFlatNorm.")
}

pub const fn unitized_radius_norm(origin: BasisElement, infinity: BasisElement) -> Elaborated<UnitizedRadiusNormImpl> {
    UnitizedRadiusNormImpl { origin, infinity }
        .new_trait_named("UnitizedRadiusNorm")
        .blurb("Unitized radius of an object.")
}

pub const fn unitized_radius_norm_squared(origin: BasisElement, infinity: BasisElement) -> Elaborated<UnitizedRadiusNormSquaredImpl> {
    UnitizedRadiusNormSquaredImpl { origin, infinity }
        .new_trait_named("UnitizedRadiusNormSquared")
        .blurb("Intermediate result to UnitizedRadiusNorm.")
}

pub const fn unitized_round_norm(origin: BasisElement, infinity: BasisElement) -> Elaborated<UnitizedRoundNormImpl> {
    UnitizedRoundNormImpl { origin, infinity }
        .new_trait_named("UnitizedRoundNorm")
        .blurb("Unitized Norm for round aspect.")
}

pub const fn unitized_round_norm_squared(origin: BasisElement, infinity: BasisElement) -> Elaborated<UnitizedRoundNormSquaredImpl> {
    UnitizedRoundNormSquaredImpl { origin, infinity }
        .new_trait_named("UnitizedRoundNormSquared")
        .blurb("Intermediate result to UnitizedRoundNorm.")
}

pub mod impls {
    use async_trait::async_trait;

    use crate::algebra::basis::BasisElement;
    use crate::algebra::basis::filter::{allow_all_signatures, signatures_containing, signatures_not_containing};
    use crate::algebra::multivector::DynamicMultiVector;
    use crate::ast::datatype::{Float, MultiVector};
    use crate::ast::expressions::FloatExpr;
    use crate::ast::traits::{HasNotReturned, TraitDef_1_Type_1_Arg, TraitDef_2_Types_2_Args, TraitImpl_11, TraitImplBuilder};
    use crate::ast::Variable;
    use crate::build_scripts::common_traits::{Addition, AntiDotProduct, AntiSquareRoot, DotProduct, RightAntiDual, SquareRoot, SubType, Wedge};
    use crate::build_scripts::common_traits::conformal::{center_norm_squared, flat_bulk, flat_bulk_norm, flat_bulk_norm_squared, flat_weight, flat_weight_norm, flat_weight_norm_squared, RadiusNormSquared, round_bulk, round_bulk_norm, round_bulk_norm_squared, round_weight, round_weight_norm, round_weight_norm_squared, unitized_center_norm_squared, unitized_flat_norm_squared, unitized_radius_norm_squared, unitized_round_norm_squared};
    use crate::trait_impl_1_type_1_arg;

    #[derive(Clone, Copy)]
    pub struct ConformalConjugateImpl {
        pub infinity: BasisElement,
    }
    #[async_trait]
    impl TraitImpl_11 for ConformalConjugateImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let infinity_sig = self.infinity.signature();
            let mut result = DynamicMultiVector::zero();
            for (mut fe, el) in slf.elements() {
                if el.signature().contains(infinity_sig) {
                    fe = fe * -1.0;
                }
                result += (fe, el);
            }
            let result = result.construct(&builder)?;
            builder.return_expr(result)
        }
    }

    #[derive(Clone, Copy)]
    pub struct RoundBulkImpl {
        pub origin: BasisElement,
        pub infinity: BasisElement,
    }
    #[async_trait]
    impl TraitImpl_11 for RoundBulkImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let all = allow_all_signatures();
            let not_origin = signatures_not_containing(self.origin);
            let not_infinity = signatures_not_containing(self.infinity);
            let result = SubType(all, not_origin & not_infinity, all)
                .inline(&mut builder, slf).await?;
            builder.return_expr(result)
        }
    }

    #[derive(Clone, Copy)]
    pub struct RoundWeightImpl {
        pub origin: BasisElement,
        pub infinity: BasisElement,
    }
    #[async_trait]
    impl TraitImpl_11 for RoundWeightImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let all = allow_all_signatures();
            let origin = signatures_containing(self.origin);
            let not_infinity = signatures_not_containing(self.infinity);
            let result = SubType(all, origin & not_infinity, all)
                .inline(&mut builder, slf).await?;
            builder.return_expr(result)
        }
    }

    #[derive(Clone, Copy)]
    pub struct FlatBulkImpl {
        pub origin: BasisElement,
        pub infinity: Option<BasisElement>,
    }
    #[async_trait]
    impl TraitImpl_11 for FlatBulkImpl {
        type Output = MultiVector;
        //noinspection DuplicatedCode
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let all = allow_all_signatures();
            let not_origin = signatures_not_containing(self.origin);
            let result = match self.infinity {
                None => SubType(all, not_origin, all).inline(&mut builder, slf).await?,
                Some(i) => {
                    let infinity = signatures_containing(i);
                    SubType(all, not_origin & infinity, all).inline(&mut builder, slf).await?
                }
            };
            builder.return_expr(result)
        }
    }

    #[derive(Clone, Copy)]
    pub struct FlatWeightImpl {
        pub origin: BasisElement,
        pub infinity: Option<BasisElement>,
    }
    #[async_trait]
    impl TraitImpl_11 for FlatWeightImpl {
        type Output = MultiVector;
        //noinspection DuplicatedCode
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let all = allow_all_signatures();
            let origin = signatures_containing(self.origin);
            let result = match self.infinity {
                None => SubType(all, origin, all).inline(&mut builder, slf).await?,
                Some(i) => {
                    let infinity = signatures_containing(i);
                    SubType(all, origin & infinity, all).inline(&mut builder, slf).await?
                }
            };
            builder.return_expr(result)
        }
    }

    #[derive(Clone, Copy)]
    pub struct CenterNormImpl { pub origin: BasisElement, pub infinity: BasisElement }
    #[async_trait]
    impl TraitImpl_11 for CenterNormImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let cns = center_norm_squared(self.origin, self.infinity).inline(&mut builder, slf).await?;
            let result = SquareRoot.inline(&mut builder, cns).await?;
            builder.return_expr(result)
        }
    }

    #[derive(Clone, Copy)]
    pub struct CenterNormSquaredImpl { pub origin: BasisElement, pub infinity: BasisElement }
    #[async_trait]
    impl TraitImpl_11 for CenterNormSquaredImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let rbns = round_bulk_norm_squared(self.origin, self.infinity).inline(&mut builder, slf.clone()).await?;
            let fwns = flat_weight_norm_squared(self.origin, Some(self.infinity)).inline(&mut builder, slf).await?;
            let ad = RightAntiDual.inline(&mut builder, fwns).await?;
            let result = Addition.inline(&mut builder, rbns, ad).await?;
            builder.return_expr(result)
        }
    }

    #[derive(Clone, Copy)]
    pub struct FlatBulkNormImpl { pub origin: BasisElement, pub infinity: Option<BasisElement> }
    #[async_trait]
    impl TraitImpl_11 for FlatBulkNormImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let fbns = flat_bulk_norm_squared(self.origin, self.infinity).inline(&mut builder, slf).await?;
            let result = SquareRoot.inline(&mut builder, fbns).await?;
            builder.return_expr(result)
        }
    }

    #[derive(Clone, Copy)]
    pub struct FlatBulkNormSquaredImpl { pub origin: BasisElement, pub infinity: Option<BasisElement> }
    #[async_trait]
    impl TraitImpl_11 for FlatBulkNormSquaredImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let mut dyn_origin = DynamicMultiVector::zero();
            dyn_origin += (FloatExpr::Literal(1.0), self.origin);
            let origin = dyn_origin.construct(&builder)?;

            let flat_bulk = flat_bulk(self.origin, self.infinity).inline(&mut builder, slf).await?;
            let wedge = Wedge.inline(&mut builder, flat_bulk, origin).await?;
            let fbt = builder.variable("flat_bulk_thing", wedge);
            let dot = DotProduct.inline(&mut builder, fbt.clone(), fbt).await?;
            builder.return_expr(dot)
        }
    }

    #[derive(Clone, Copy)]
    pub struct FlatNormImpl { pub origin: BasisElement, pub infinity: Option<BasisElement> }
    #[async_trait]
    impl TraitImpl_11 for FlatNormImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let fbn = flat_bulk_norm(self.origin, self.infinity).inline(&mut builder, slf.clone()).await?;
            let fwn = flat_weight_norm(self.origin, self.infinity).inline(&mut builder, slf).await?;
            let result = Addition.inline(&mut builder, fbn, fwn).await?;
            builder.return_expr(result)
        }
    }

    #[derive(Clone, Copy)]
    pub struct FlatNormSquaredImpl { pub origin: BasisElement, pub infinity: Option<BasisElement> }
    #[async_trait]
    impl TraitImpl_11 for FlatNormSquaredImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let fbn = flat_bulk_norm_squared(self.origin, self.infinity).inline(&mut builder, slf.clone()).await?;
            let fwn = flat_weight_norm_squared(self.origin, self.infinity).inline(&mut builder, slf).await?;
            let result = Addition.inline(&mut builder, fbn, fwn).await?;
            builder.return_expr(result)
        }
    }

    #[derive(Clone, Copy)]
    pub struct FlatWeightNormImpl { pub origin: BasisElement, pub infinity: Option<BasisElement> }
    #[async_trait]
    impl TraitImpl_11 for FlatWeightNormImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let fwns = flat_weight_norm_squared(self.origin, self.infinity).inline(&mut builder, slf).await?;
            let result = AntiSquareRoot.inline(&mut builder, fwns).await?;
            builder.return_expr(result)
        }
    }

    #[derive(Clone, Copy)]
    pub struct FlatWeightNormSquaredImpl { pub origin: BasisElement, pub infinity: Option<BasisElement> }
    #[async_trait]
    impl TraitImpl_11 for FlatWeightNormSquaredImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let fw = flat_weight(self.origin, self.infinity).inline(&mut builder, slf).await?;
            let fw = builder.variable("flat_weight", fw);
            let anti_dot = AntiDotProduct.inline(&mut builder, fw.clone(), fw).await?;
            builder.return_expr(anti_dot)
        }
    }

    trait_impl_1_type_1_arg!(RadiusNormImpl(builder, slf) -> MultiVector {
        let rns = RadiusNormSquared.inline(&mut builder, slf.clone()).await?;
        let result = SquareRoot.inline(&mut builder, rns).await?;
        builder.return_expr(result)
    });
    trait_impl_1_type_1_arg!(RadiusNormSquaredImpl(builder, slf) -> MultiVector {
        let anti_dot = AntiDotProduct.inline(&mut builder, slf.clone(), slf.clone()).await?;
        let result = RightAntiDual.inline(&mut builder, anti_dot).await?;
        builder.return_expr(result)
    });

    #[derive(Clone, Copy)]
    pub struct RoundBulkNormImpl { pub origin: BasisElement, pub infinity: BasisElement }
    #[async_trait]
    impl TraitImpl_11 for RoundBulkNormImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let rbns = round_bulk_norm_squared(self.origin, self.infinity).inline(&mut builder, slf).await?;
            let result = SquareRoot.inline(&mut builder, rbns).await?;
            builder.return_expr(result)
        }
    }

    #[derive(Clone, Copy)]
    pub struct RoundBulkNormSquaredImpl { pub origin: BasisElement, pub infinity: BasisElement }
    #[async_trait]
    impl TraitImpl_11 for RoundBulkNormSquaredImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let rb = round_bulk(self.origin, self.infinity).inline(&mut builder, slf).await?;
            let rb = builder.variable("round_bulk", rb);
            let dot = DotProduct.inline(&mut builder, rb.clone(), rb).await?;
            builder.return_expr(dot)
        }
    }

    #[derive(Clone, Copy)]
    pub struct RoundNormImpl { pub origin: BasisElement, pub infinity: BasisElement }
    #[async_trait]
    impl TraitImpl_11 for RoundNormImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let rbn = round_bulk_norm(self.origin, self.infinity).inline(&mut builder, slf.clone()).await?;
            let rwn = round_weight_norm(self.origin, self.infinity).inline(&mut builder, slf).await?;
            let result = Addition.inline(&mut builder, rbn, rwn).await?;
            builder.return_expr(result)
        }
    }

    #[derive(Clone, Copy)]
    pub struct RoundNormSquaredImpl { pub origin: BasisElement, pub infinity: BasisElement }
    #[async_trait]
    impl TraitImpl_11 for RoundNormSquaredImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let rbn = round_bulk_norm_squared(self.origin, self.infinity).inline(&mut builder, slf.clone()).await?;
            let rwn = round_weight_norm_squared(self.origin, self.infinity).inline(&mut builder, slf).await?;
            let result = Addition.inline(&mut builder, rbn, rwn).await?;
            builder.return_expr(result)
        }
    }

    #[derive(Clone, Copy)]
    pub struct RoundWeightNormImpl { pub origin: BasisElement, pub infinity: BasisElement }
    #[async_trait]
    impl TraitImpl_11 for RoundWeightNormImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let rwns = round_weight_norm_squared(self.origin, self.infinity).inline(&mut builder, slf).await?;
            let result = AntiSquareRoot.inline(&mut builder, rwns).await?;
            builder.return_expr(result)
        }
    }

    #[derive(Clone, Copy)]
    pub struct RoundWeightNormSquaredImpl { pub origin: BasisElement, pub infinity: BasisElement }
    #[async_trait]
    impl TraitImpl_11 for RoundWeightNormSquaredImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let mut dyn_infinity = DynamicMultiVector::zero();
            dyn_infinity += (FloatExpr::Literal(1.0), self.infinity);
            let infinity = dyn_infinity.construct(&builder)?;

            let rw = round_weight(self.origin, self.infinity).inline(&mut builder, slf).await?;
            let wedge = Wedge.inline(&mut builder, rw, infinity).await?;
            let v = builder.variable("round_weight_carrier", wedge);
            let anti_dot = AntiDotProduct.inline(&mut builder, v.clone(), v).await?;
            builder.return_expr(anti_dot)
        }
    }

    #[derive(Clone, Copy)]
    pub struct UnitizedCenterNormImpl { pub origin: BasisElement, pub infinity: BasisElement }
    #[async_trait]
    impl TraitImpl_11 for UnitizedCenterNormImpl {
        type Output = Float;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let ucns = unitized_center_norm_squared(self.origin, self.infinity).inline(&mut builder, slf).await?;
            let sqrt = FloatExpr::Product(vec![(ucns.into(), 0.5)], 1.0);
            builder.return_expr(sqrt)
        }
    }

    #[derive(Clone, Copy)]
    pub struct UnitizedCenterNormSquaredImpl { pub origin: BasisElement, pub infinity: BasisElement }
    #[async_trait]
    impl TraitImpl_11 for UnitizedCenterNormSquaredImpl {
        type Output = Float;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let numerator = center_norm_squared(self.origin, self.infinity).inline(&mut builder, slf.clone()).await?;
            let denominator = round_weight_norm_squared(self.origin, self.infinity).inline(&mut builder, slf.clone()).await?;
            let numerator = FloatExpr::AccessMultiVecFlat(numerator.into(), 0);
            let denominator = FloatExpr::AccessMultiVecFlat(denominator.into(), 0);
            let divide = FloatExpr::Product(vec![(numerator, 1.0), (denominator, -1.0)], 1.0);
            builder.return_expr(divide)
        }
    }

    #[derive(Clone, Copy)]
    pub struct UnitizedFlatNormImpl { pub origin: BasisElement, pub infinity: Option<BasisElement> }
    #[async_trait]
    impl TraitImpl_11 for UnitizedFlatNormImpl {
        type Output = Float;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let ufns = unitized_flat_norm_squared(self.origin, self.infinity).inline(&mut builder, slf).await?;
            let sqrt = FloatExpr::Product(vec![(ufns.into(), 0.5)], 1.0);
            builder.return_expr(sqrt)
        }
    }

    #[derive(Clone, Copy)]
    pub struct UnitizedFlatNormSquaredImpl { pub origin: BasisElement, pub infinity: Option<BasisElement> }
    #[async_trait]
    impl TraitImpl_11 for UnitizedFlatNormSquaredImpl {
        type Output = Float;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let numerator = flat_bulk_norm_squared(self.origin, self.infinity).inline(&mut builder, slf.clone()).await?;
            let denominator = flat_weight_norm_squared(self.origin, self.infinity).inline(&mut builder, slf.clone()).await?;
            let numerator = FloatExpr::AccessMultiVecFlat(numerator.into(), 0);
            let denominator = FloatExpr::AccessMultiVecFlat(denominator.into(), 0);
            let divide = FloatExpr::Product(vec![(numerator, 1.0), (denominator, -1.0)], 1.0);
            builder.return_expr(divide)
        }
    }

    #[derive(Clone, Copy)]
    pub struct UnitizedRadiusNormImpl { pub origin: BasisElement, pub infinity: BasisElement }
    #[async_trait]
    impl TraitImpl_11 for UnitizedRadiusNormImpl {
        type Output = Float;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let urns = unitized_radius_norm_squared(self.origin, self.infinity).inline(&mut builder, slf).await?;
            let sqrt = FloatExpr::Product(vec![(urns.into(), 0.5)], 1.0);
            builder.return_expr(sqrt)
        }
    }

    #[derive(Clone, Copy)]
    pub struct UnitizedRadiusNormSquaredImpl { pub origin: BasisElement, pub infinity: BasisElement }
    #[async_trait]
    impl TraitImpl_11 for UnitizedRadiusNormSquaredImpl {
        type Output = Float;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let numerator = RadiusNormSquared.inline(&mut builder, slf.clone()).await?;
            let denominator = round_weight_norm_squared(self.origin, self.infinity).inline(&mut builder, slf.clone()).await?;
            let numerator = FloatExpr::AccessMultiVecFlat(numerator.into(), 0);
            let denominator = FloatExpr::AccessMultiVecFlat(denominator.into(), 0);
            let divide = FloatExpr::Product(vec![(numerator, 1.0), (denominator, -1.0)], 1.0);
            builder.return_expr(divide)
        }
    }

    #[derive(Clone, Copy)]
    pub struct UnitizedRoundNormImpl { pub origin: BasisElement, pub infinity: BasisElement }
    #[async_trait]
    impl TraitImpl_11 for UnitizedRoundNormImpl {
        type Output = Float;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let urns = unitized_round_norm_squared(self.origin, self.infinity).inline(&mut builder, slf).await?;
            let sqrt = FloatExpr::Product(vec![(urns.into(), 0.5)], 1.0);
            builder.return_expr(sqrt)
        }
    }

    #[derive(Clone, Copy)]
    pub struct UnitizedRoundNormSquaredImpl { pub origin: BasisElement, pub infinity: BasisElement }
    #[async_trait]
    impl TraitImpl_11 for UnitizedRoundNormSquaredImpl {
        type Output = Float;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let numerator = round_bulk_norm_squared(self.origin, self.infinity).inline(&mut builder, slf.clone()).await?;
            let denominator = round_weight_norm_squared(self.origin, self.infinity).inline(&mut builder, slf.clone()).await?;
            let numerator = FloatExpr::AccessMultiVecFlat(numerator.into(), 0);
            let denominator = FloatExpr::AccessMultiVecFlat(denominator.into(), 0);
            let divide = FloatExpr::Product(vec![(numerator, 1.0), (denominator, -1.0)], 1.0);
            builder.return_expr(divide)
        }
    }
}