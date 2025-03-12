use crate::utility::ptrarc::PtrArc;

/// MultiVectorVia::Construct can be destructured into Vecs and Floats for multi-line simplification.
/// VecNExpr::GatherN can be destructured into Floats for the same purpose.
/// However, trait invocation arguments (including arithmetic) demand that keep the variable instead.
/// A FloatExpr cannot be destructured, despite appearing in places where VecNExpr might be destructured.
pub(crate) struct DestructurableVariables {

    // These values shouldn't be destructured/inlined, because they are used in whole
    // (for example as an argument to a function or arithmetic)
    whole_variable_is_used: HashSet<PtrArc<RawVariableDeclaration>>,

    // These values might should be destructured, as long as they don't
    // also appear in whole_variable_is_used. For example maybe we have a Point, but only
    // use e4, and destructuring the point and inlining the e4 accesses will simplify the code.
    // The other thing to be aware of is that the variable can/will only be destructured
    // if the variable declaration is a construction itself (not a trait invocation or arithmetic expression).
    partial_variable_use: HashSet<PtrArc<RawVariableDeclaration>>,
}
impl DestructurableVariables {
    pub(crate) fn new() -> Self {
        DestructurableVariables {
            whole_variable_is_used: HashSet::new(),
            partial_variable_use: HashSet::new()
        }
    }

    fn note_whole_variable_use(&mut self, rvi: &RawVariableInvocation) {
        self.whole_variable_is_used.insert(rvi.decl.clone().into());
    }

    fn note_partial_variable_use(&mut self, rvi: &RawVariableInvocation) {
        let rvd = &rvi.decl;
        let Some(vd) = &rvd.expr else { return };
        if self.partial_variable_use.contains(&rvd.clone().into()) { return }
        if self.whole_variable_is_used.contains(&rvd.clone().into()) { return }

        // TODO maybe we should allow destructuring Products if they are just one term with coefficients
        //  impl AntiConstraintViolation for AntiMotor
        let ae = vd.read();
        let result = match &*ae {
            AnyExpression::Vec2(Vec2Expr::Gather1(_)) => true,
            AnyExpression::Vec2(Vec2Expr::Gather2(_, _)) => true,
            AnyExpression::Vec2(Vec2Expr::AccessMultiVecGroup(MultiVectorExpr { expr: box MultiVectorVia::Variable(_), .. }, _)) => true,
            // AnyExpression::Vec2(Vec2Expr::Truncate3to2(box Vec3Expr::Variable(..))) => true,
            // AnyExpression::Vec2(Vec2Expr::Truncate3to2(box Vec3Expr::SwizzleVec3(box Vec3Expr::Variable(_), ..))) => true,
            // AnyExpression::Vec2(Vec2Expr::Truncate4to2(box Vec4Expr::Variable(..))) => true,
            // AnyExpression::Vec2(Vec2Expr::Truncate4to2(box Vec4Expr::SwizzleVec4(box Vec4Expr::Variable(_), ..))) => true,
            AnyExpression::Vec2(Vec2Expr::Product(v, last_factor)) if v.len() == 1 => match &v[0].0 {
                Vec2Expr::Gather1(_) if last_factor[0] == last_factor[1] => true,
                Vec2Expr::Gather2(_, _) => true,
                Vec2Expr::AccessMultiVecGroup(MultiVectorExpr { expr: box MultiVectorVia::Variable(_), .. }, _) => true,
                // Vec2Expr::Truncate3to2(box Vec3Expr::Variable(..)) => true,
                // Vec2Expr::Truncate3to2(box Vec3Expr::SwizzleVec3(box Vec3Expr::Variable(_), ..)) => true,
                // Vec2Expr::Truncate4to2(box Vec4Expr::Variable(..)) => true,
                // Vec2Expr::Truncate4to2(box Vec4Expr::SwizzleVec4(box Vec4Expr::Variable(_), ..)) => true,
                _ => false
            }
            AnyExpression::Vec3(Vec3Expr::Gather1(_)) => true,
            AnyExpression::Vec3(Vec3Expr::Gather3(_, _, _)) => true,
            AnyExpression::Vec3(Vec3Expr::Extend2to3(_, _)) => true,
            AnyExpression::Vec3(Vec3Expr::AccessMultiVecGroup(MultiVectorExpr { expr: box MultiVectorVia::Variable(_), .. }, _)) => true,
            // AnyExpression::Vec3(Vec3Expr::Truncate4to3(box Vec4Expr::Variable(..))) => true,
            // AnyExpression::Vec3(Vec3Expr::Truncate4to3(box Vec4Expr::SwizzleVec4(box Vec4Expr::Variable(..), ..))) => true,
            AnyExpression::Vec3(Vec3Expr::Product(v, last_factor)) if v.len() == 1 => match &v[0].0 {
                Vec3Expr::Gather1(_) if last_factor[0] == last_factor[1] && last_factor[0] == last_factor[2] => true,
                Vec3Expr::Gather3(_, _, _) => true,
                Vec3Expr::Extend2to3(_, _) => true,
                Vec3Expr::AccessMultiVecGroup(MultiVectorExpr { expr: box MultiVectorVia::Variable(_), .. }, _) => true,
                // Vec3Expr::Truncate4to3(box Vec4Expr::Variable(..)) => true,
                // Vec3Expr::Truncate4to3(box Vec4Expr::SwizzleVec4(box Vec4Expr::Variable(..), ..)) => true,
                _ => false
            }
            AnyExpression::Vec4(Vec4Expr::Gather1(_)) => true,
            AnyExpression::Vec4(Vec4Expr::Gather4(_, _, _, _)) => true,
            AnyExpression::Vec4(Vec4Expr::Extend2to4(_, _, _)) => true,
            AnyExpression::Vec4(Vec4Expr::Extend3to4(_, _)) => true,
            AnyExpression::Vec4(Vec4Expr::AccessMultiVecGroup(MultiVectorExpr { expr: box MultiVectorVia::Variable(_), .. }, _)) => true,
            AnyExpression::Vec4(Vec4Expr::Product(v, last_factor)) if v.len() == 1 => match &v[0].0 {
                Vec4Expr::Gather1(_) if last_factor[0] == last_factor[1] && last_factor[0] == last_factor[2] && last_factor[0] == last_factor[3] => true,
                Vec4Expr::Gather4(_, _, _, _) => true,
                Vec4Expr::Extend2to4(_, _, _) => true,
                Vec4Expr::Extend3to4(_, _) => true,
                Vec4Expr::AccessMultiVecGroup(MultiVectorExpr { expr: box MultiVectorVia::Variable(_), .. }, _) => true,
                _ => false
            }
            AnyExpression::Class(MultiVectorExpr { expr: box MultiVectorVia::Construct(_), .. }) => true,
            _ => false
        };
        if result {
            self.partial_variable_use.insert(rvd.clone().into());
        }
    }

    pub(crate) fn needs_destructuring(&self) -> Vec<Arc<RawVariableDeclaration>> {
        let mut result = Vec::new();
        for it in self.partial_variable_use.iter() {
            if !self.whole_variable_is_used.contains(it) {
                result.push(it.clone().into());
            }
        }
        result
    }
}



impl AnyExpression {
    pub(crate) fn scan_for_destructurable_variables(&self, tracker: &mut DestructurableVariables) {
        match self {
            AnyExpression::Int(e) => e.scan_for_destructurable_variables(tracker),
            AnyExpression::Float(e) => e.scan_for_destructurable_variables(tracker),
            AnyExpression::Vec2(e) => e.scan_for_destructurable_variables(tracker),
            AnyExpression::Vec3(e) => e.scan_for_destructurable_variables(tracker),
            AnyExpression::Vec4(e) => e.scan_for_destructurable_variables(tracker),
            AnyExpression::Class(e) => e.scan_for_destructurable_variables(tracker),
        }
    }
}


impl IntExpr {
    fn scan_for_destructurable_variables(&self, _tracker: &mut DestructurableVariables) {
        match self {
            IntExpr::Variable(_) => {}
            IntExpr::Literal(_) => {}
            IntExpr::TraitInvoke10ToInt(_t, _mv) => {}
        }
    }
}

impl FloatExpr {
    fn scan_for_destructurable_variables(&self, tracker: &mut DestructurableVariables) {
        match self {
            FloatExpr::Variable(_) => {}
            FloatExpr::Literal(_) => {}
            FloatExpr::FromInt(e) => e.scan_for_destructurable_variables(tracker),
            FloatExpr::AccessVec2(box v, _i) => {
                if let Vec2Expr::Variable(v) = &v {
                    tracker.note_partial_variable_use(&v);
                } else {
                    v.scan_for_destructurable_variables(tracker);
                }
            }
            FloatExpr::AccessVec3(box v, _i) => {
                if let Vec3Expr::Variable(v) = &v {
                    tracker.note_partial_variable_use(&v);
                } else {
                    v.scan_for_destructurable_variables(tracker);
                }
            }
            FloatExpr::AccessVec4(box v, _i) => {
                if let Vec4Expr::Variable(v) = &v {
                    tracker.note_partial_variable_use(&v);
                } else {
                    v.scan_for_destructurable_variables(tracker);
                }
            }
            FloatExpr::AccessMultiVecGroup(mv, _i) => {
                if let box MultiVectorVia::Variable(v) = &mv.expr {
                    tracker.note_partial_variable_use(&v);
                } else {
                    mv.scan_for_destructurable_variables(tracker);
                }
            }
            FloatExpr::AccessMultiVecFlat(mv, _i) => {
                if let box MultiVectorVia::Variable(v) = &mv.expr {
                    tracker.note_partial_variable_use(&v);
                } else {
                    mv.scan_for_destructurable_variables(tracker);
                }
            }
            FloatExpr::TraitInvoke11ToFloat(_t, mv) => {
                if let box MultiVectorVia::Variable(v) = &mv.expr {
                    tracker.note_whole_variable_use(&v);
                } else {
                    mv.scan_for_destructurable_variables(tracker);
                }
            }
            FloatExpr::Product(v, _l) => {
                for (f, _l) in v {
                    f.scan_for_destructurable_variables(tracker);
                }
            }
            FloatExpr::Sum(v, _l) => {
                for (f, _l) in v {
                    f.scan_for_destructurable_variables(tracker);
                }
            }
            FloatExpr::Exp(v, p, _l) => {
                v.scan_for_destructurable_variables(tracker);
                if let Some(p) = &p {
                    p.scan_for_destructurable_variables(tracker);
                }
            }
        }
    }
}

impl Vec2Expr {
    fn scan_for_destructurable_variables(&self, tracker: &mut DestructurableVariables) {
        match self {
            Vec2Expr::Variable(_) => {}
            Vec2Expr::Gather1(x) => {
                x.scan_for_destructurable_variables(tracker);
            }
            Vec2Expr::Gather2(x, y) => {
                x.scan_for_destructurable_variables(tracker);
                y.scan_for_destructurable_variables(tracker);
            }
            Vec2Expr::AccessMultiVecGroup(mv, _i) => {
                if let box MultiVectorVia::Variable(v) = &mv.expr {
                    tracker.note_partial_variable_use(&v);
                } else {
                    mv.scan_for_destructurable_variables(tracker);
                }
            }
            Vec2Expr::Product(v, _l) => {
                for (f, _l) in v {
                    if let Vec2Expr::Variable(v) = f {
                        tracker.note_whole_variable_use(&v);
                    } else {
                        f.scan_for_destructurable_variables(tracker);
                    }
                }
            }
            Vec2Expr::Sum(v, _l) => {
                for (f, _l) in v {
                    if let Vec2Expr::Variable(v) = f {
                        tracker.note_whole_variable_use(&v);
                    } else {
                        f.scan_for_destructurable_variables(tracker);
                    }
                }
            }
            Vec2Expr::SwizzleVec2(box v, _a, _b) => {
                v.scan_for_destructurable_variables(tracker);
            }
            // Vec2Expr::Truncate3to2(box Vec3Expr::Variable(v)) => tracker.note_partial_variable_use(v),
            // Vec2Expr::Truncate3to2(box Vec3Expr::SwizzleVec3(box Vec3Expr::Variable(v), _, _, _)) => tracker.note_partial_variable_use(v),
            Vec2Expr::Truncate3to2(box v) => v.scan_for_destructurable_variables(tracker),
            // Vec2Expr::Truncate4to2(box Vec4Expr::Variable(v)) => tracker.note_partial_variable_use(v),
            // Vec2Expr::Truncate4to2(box Vec4Expr::SwizzleVec4(box Vec4Expr::Variable(v), _, _, _, _)) => tracker.note_partial_variable_use(v),
            Vec2Expr::Truncate4to2(box v) => v.scan_for_destructurable_variables(tracker),
        }
    }
}

impl Vec3Expr {
    fn scan_for_destructurable_variables(&self, tracker: &mut DestructurableVariables) {
        match self {
            Vec3Expr::Variable(_) => {}
            Vec3Expr::Gather1(x) => {
                x.scan_for_destructurable_variables(tracker);
            }
            Vec3Expr::Gather3(x, y, z) => {
                x.scan_for_destructurable_variables(tracker);
                y.scan_for_destructurable_variables(tracker);
                z.scan_for_destructurable_variables(tracker);
            }
            Vec3Expr::AccessMultiVecGroup(mv, _i) => {
                if let box MultiVectorVia::Variable(v) = &mv.expr {
                    tracker.note_partial_variable_use(&v);
                } else {
                    mv.scan_for_destructurable_variables(tracker);
                }
            }
            Vec3Expr::Product(v, _l) => {
                for (f, _l) in v {
                    if let Vec3Expr::Variable(v) = f {
                        tracker.note_whole_variable_use(&v);
                    } else {
                        f.scan_for_destructurable_variables(tracker);
                    }
                }
            }
            Vec3Expr::Sum(v, _l) => {
                for (f, _l) in v {
                    if let Vec3Expr::Variable(v) = f {
                        tracker.note_whole_variable_use(&v);
                    } else {
                        f.scan_for_destructurable_variables(tracker);
                    }
                }
            }
            Vec3Expr::SwizzleVec3(box v, _a, _b, _c) => {
                v.scan_for_destructurable_variables(tracker);
            }
            // Vec3Expr::Truncate4to3(box Vec4Expr::Variable(v)) => tracker.note_partial_variable_use(v),
            // Vec3Expr::Truncate4to3(box Vec4Expr::SwizzleVec4(box Vec4Expr::Variable(v), _, _, _, _)) => tracker.note_partial_variable_use(v),
            Vec3Expr::Truncate4to3(box v) => v.scan_for_destructurable_variables(tracker),
            Vec3Expr::Extend2to3(v, d) => {
                // It is tempting to note_whole_variable_use here, but actually,
                // not doing so will work and inline more stuff.
                // (Because the variable will be replaced with a raw construction of variables,
                //  and then simplification will sort it out)
                v.scan_for_destructurable_variables(tracker);
                d.scan_for_destructurable_variables(tracker);
            }
        }
    }
}

impl Vec4Expr {
    fn scan_for_destructurable_variables(&self, tracker: &mut DestructurableVariables) {
        match self {
            Vec4Expr::Variable(_) => {}
            Vec4Expr::Gather1(x) => {
                x.scan_for_destructurable_variables(tracker);
            }
            Vec4Expr::Gather4(x, y, z, w) => {
                x.scan_for_destructurable_variables(tracker);
                y.scan_for_destructurable_variables(tracker);
                z.scan_for_destructurable_variables(tracker);
                w.scan_for_destructurable_variables(tracker);
            }
            Vec4Expr::AccessMultiVecGroup(mv, _i) => {
                if let box MultiVectorVia::Variable(v) = &mv.expr {
                    tracker.note_partial_variable_use(&v);
                } else {
                    mv.scan_for_destructurable_variables(tracker);
                }
            }
            Vec4Expr::Product(v, _l) => {
                for (f, _l) in v {
                    if let Vec4Expr::Variable(v) = f {
                        tracker.note_whole_variable_use(&v);
                    } else {
                        f.scan_for_destructurable_variables(tracker);
                    }
                }
            }
            Vec4Expr::Sum(v, _l) => {
                for (f, _l) in v {
                    if let Vec4Expr::Variable(v) = f {
                        tracker.note_whole_variable_use(&v);
                    } else {
                        f.scan_for_destructurable_variables(tracker);
                    }
                }
            }
            Vec4Expr::SwizzleVec4(box v, _a, _b, _c, _d) => {
                v.scan_for_destructurable_variables(tracker);
            }
            Vec4Expr::Extend2to4(v, c, d) => {
                // It is tempting to note_whole_variable_use here, but actually,
                // not doing so will work and inline more stuff.
                // (Because the variable will be replaced with a raw construction of variables,
                //  and then simplification will sort it out)
                v.scan_for_destructurable_variables(tracker);
                c.scan_for_destructurable_variables(tracker);
                d.scan_for_destructurable_variables(tracker);
            }
            Vec4Expr::Extend3to4(v, d) => {
                // It is tempting to note_whole_variable_use here, but actually,
                // not doing so will work and inline more stuff.
                // (Because the variable will be replaced with a raw construction of variables,
                //  and then simplification will sort it out)
                v.scan_for_destructurable_variables(tracker);
                d.scan_for_destructurable_variables(tracker);
            }
        }
    }
}

impl MultiVectorGroupExpr {
    fn scan_for_destructurable_variables(&self, tracker: &mut DestructurableVariables) {
        match self {
            MultiVectorGroupExpr::JustFloat(f) => f.scan_for_destructurable_variables(tracker),
            MultiVectorGroupExpr::Vec2(v) => v.scan_for_destructurable_variables(tracker),
            MultiVectorGroupExpr::Vec3(v) => v.scan_for_destructurable_variables(tracker),
            MultiVectorGroupExpr::Vec4(v) => v.scan_for_destructurable_variables(tracker),
        }
    }
}

impl MultiVectorExpr {
    fn scan_for_destructurable_variables(&self, tracker: &mut DestructurableVariables) {
        match &*self.expr {
            MultiVectorVia::Variable(_) => {}
            MultiVectorVia::Construct(v) => {
                for v in v {
                    v.scan_for_destructurable_variables(tracker);
                }
            }
            MultiVectorVia::TraitInvoke11ToClass(_t, a) => {
                if let box MultiVectorVia::Variable(v) = &a.expr {
                    tracker.note_whole_variable_use(&v);
                } else {
                    a.scan_for_destructurable_variables(tracker);
                }
            }
            MultiVectorVia::TraitInvoke12iToClass(_t, a, b) => {
                if let box MultiVectorVia::Variable(v) = &a.expr {
                    tracker.note_whole_variable_use(&v);
                } else {
                    a.scan_for_destructurable_variables(tracker);
                }
                if let IntExpr::Variable(v) = &b {
                    tracker.note_whole_variable_use(&v);
                } else {
                    b.scan_for_destructurable_variables(tracker);
                }
            }
            MultiVectorVia::TraitInvoke12fToClass(_t, a, b) => {
                if let box MultiVectorVia::Variable(v) = &a.expr {
                    tracker.note_whole_variable_use(&v);
                } else {
                    a.scan_for_destructurable_variables(tracker);
                }
                if let FloatExpr::Variable(v) = &b {
                    tracker.note_whole_variable_use(&v);
                } else {
                    b.scan_for_destructurable_variables(tracker);
                }
            }
            MultiVectorVia::TraitInvoke21ToClass(_t, a, _b) => {
                if let box MultiVectorVia::Variable(v) = &a.expr {
                    tracker.note_whole_variable_use(&v);
                } else {
                    a.scan_for_destructurable_variables(tracker);
                }
            }
            MultiVectorVia::TraitInvoke22ToClass(_t, a, b) => {
                if let box MultiVectorVia::Variable(v) = &a.expr {
                    tracker.note_whole_variable_use(&v);
                } else {
                    a.scan_for_destructurable_variables(tracker);
                }
                if let box MultiVectorVia::Variable(v) = &b.expr {
                    tracker.note_whole_variable_use(&v);
                } else {
                    b.scan_for_destructurable_variables(tracker);
                }
            }
        }
    }
}