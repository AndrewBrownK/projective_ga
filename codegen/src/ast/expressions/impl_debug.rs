
// TODO make the Debug content valid rust copy-pasta


impl Debug for AnyExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "/* AnyExpression */ ")?;
        match self {
            AnyExpression::Int(e) => write!(f, "{e:?}")?,
            AnyExpression::Float(e) => write!(f, "{e:?}")?,
            AnyExpression::Vec2(e) => write!(f, "{e:?}")?,
            AnyExpression::Vec3(e) => write!(f, "{e:?}")?,
            AnyExpression::Vec4(e) => write!(f, "{e:?}")?,
            AnyExpression::Class(e) => write!(f, "{e:?}")?,
        }
        Ok(())
    }
}

impl Debug for IntExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let IntExpr::Variable(v) = &self {
            let (n, i) = &v.decl.name;
            match (n.as_str(), i) {
                ("self", 0) => write!(f, "slf")?,
                (n, 0) => write!(f, "{n}")?,
                (n, i) => write!(f, "{n}_{}", i + 1)?,
            }
            return write!(f, ".clone().into()");
        }
        write!(f, "IntExpr::")?;
        match self {
            IntExpr::Variable(_) => {}
            IntExpr::Literal(l) => write!(f, "Literal({l})")?,
            IntExpr::TraitInvoke10ToInt(t, m) => write!(f, "TraitInvoke10ToInt({t:?}, {m})")?,
        }
        Ok(())
    }
}
impl Debug for FloatExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let FloatExpr::Variable(v) = &self {
            let (n, i) = &v.decl.name;
            match (n.as_str(), i) {
                ("self", 0) => write!(f, "slf")?,
                (n, 0) => write!(f, "{n}")?,
                (n, i) => write!(f, "{n}_{}", i + 1)?,
            }
            return write!(f, ".clone().into()");
        }
        write!(f, "FloatExpr::")?;
        match self {
            FloatExpr::Variable(_) => {}
            FloatExpr::Literal(l) => write!(f, "Literal({l})")?,
            FloatExpr::FromInt(i) => write!(f, "FromInt({i:?})")?,
            FloatExpr::AccessVec2(v, i) => write!(f, "access_vec_2({}, {i})", *v)?,
            FloatExpr::AccessVec3(v, i) => write!(f, "access_vec_3({}, {i})", *v)?,
            FloatExpr::AccessVec4(v, i) => write!(f, "access_vec_4({}, {i})", *v)?,
            FloatExpr::AccessMultiVecGroup(mve, i) => write!(f, "AccessMultiVecGroup({mve:?}, {i:?})")?,
            FloatExpr::AccessMultiVecFlat(mve, i) => write!(f, "AccessMultiVecFlat({mve:?}, {i:?})")?,
            FloatExpr::TraitInvoke11ToFloat(t, m) => write!(f, "TraitInvoke11ToFloat({t:?}, {m})")?,
            FloatExpr::Product(v, l) => {
                write!(f, "Product(vec![")?;
                for (e, exp) in v.iter() {
                    write!(f, "({e:?}, {exp}), ")?;
                }
                write!(f, "], {l:?})")?;
            }
            FloatExpr::Sum(v, l) => {
                write!(f, "Sum(vec![")?;
                for (e, coe) in v.iter() {
                    write!(f, "({e:?}, {coe}), ")?;
                }
                write!(f, "], {l:?})")?;
            }
            FloatExpr::Exp(box a, b, c) => {
                write!(f, "Exp(Box::new({a:?}), ")?;
                match b {
                    None => write!(f, "None")?,
                    Some(box b) => write!(f, "Some(Box::new({b:?}))")?,
                }
                write!(f, ", {c:?})")?;
            }
        }
        Ok(())
    }
}
impl Debug for Vec2Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Vec2Expr::Variable(v) = &self {
            let (n, i) = &v.decl.name;
            match (n.as_str(), i) {
                ("self", 0) => write!(f, "slf")?,
                (n, 0) => write!(f, "{n}")?,
                (n, i) => write!(f, "{n}_{}", i + 1)?,
            }
            return write!(f, ".clone().into()");
        }
        write!(f, "Vec2Expr::")?;
        match self {
            Vec2Expr::Variable(_) => {}
            Vec2Expr::Gather1(x) => write!(f, "Gather1({x:?})")?,
            Vec2Expr::Gather2(x, y) => write!(f, "Gather2({x:?}, {y:?})")?,
            Vec2Expr::AccessMultiVecGroup(mve, i) => write!(f, "AccessMultiVecGroup({mve:?}, {i:?})")?,
            Vec2Expr::Product(v, l) => {
                write!(f, "Product(vec![")?;
                for (e, exp) in v.iter() {
                    write!(f, "({e:?}, {exp}), ")?;
                }
                write!(f, "], {l:?})")?;
            }
            Vec2Expr::Sum(v, l) => {
                write!(f, "Sum(vec![")?;
                for (e, coe) in v.iter() {
                    write!(f, "({e:?}, {coe}), ")?;
                }
                write!(f, "], {l:?})")?;
            }
            Vec2Expr::SwizzleVec2(box v, i0, i1) => write!(f, "swizzle_vec_2({v:?}, {i0}, {i1})")?,
            Vec2Expr::Truncate3to2(box v) => write!(f, "Truncate3to2(Box::new({v:?}))")?,
            Vec2Expr::Truncate4to2(box v) => write!(f, "Truncate4to2(Box::new({v:?}))")?,
        }
        Ok(())
    }
}
impl Debug for Vec3Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Vec3Expr::Variable(v) = &self {
            let (n, i) = &v.decl.name;
            match (n.as_str(), i) {
                ("self", 0) => write!(f, "slf")?,
                (n, 0) => write!(f, "{n}")?,
                (n, i) => write!(f, "{n}_{}", i + 1)?,
            }
            return write!(f, ".clone().into()");
        }
        write!(f, "Vec3Expr::")?;
        match self {
            Vec3Expr::Variable(_) => {}
            Vec3Expr::Gather1(x) => write!(f, "Gather1({x:?})")?,
            Vec3Expr::Gather3(x, y, z) => write!(f, "Gather3({x:?}, {y:?}, {z:?})")?,
            Vec3Expr::AccessMultiVecGroup(mve, i) => write!(f, "AccessMultiVecGroup({mve:?}, {i:?})")?,
            Vec3Expr::Product(v, l) => {
                write!(f, "Product(vec![")?;
                for (e, exp) in v.iter() {
                    write!(f, "({e:?}, {exp}), ")?;
                }
                write!(f, "], {l:?})")?;
            }
            Vec3Expr::Sum(v, l) => {
                write!(f, "Sum(vec![")?;
                for (e, coe) in v.iter() {
                    write!(f, "({e:?}, {coe}), ")?;
                }
                write!(f, "], {l:?})")?;
            }
            Vec3Expr::SwizzleVec3(box v, i0, i1, i2) => write!(f, "swizzle_vec_3({v:?}, {i0}, {i1}, {i2})")?,
            Vec3Expr::Truncate4to3(box v) => write!(f, "Truncate4to3(Box::new({v:?}))")?,
            Vec3Expr::Extend2to3(v, z) => write!(f, "Extend2to3(Box::new({v:?}), {z:?})")?,
        }
        Ok(())
    }
}
impl Debug for Vec4Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Vec4Expr::Variable(v) = &self {
            let (n, i) = &v.decl.name;
            match (n.as_str(), i) {
                ("self", 0) => write!(f, "slf")?,
                (n, 0) => write!(f, "{n}")?,
                (n, i) => write!(f, "{n}_{}", i + 1)?,
            }
            return write!(f, ".clone().into()");
        }
        write!(f, "Vec4Expr::")?;
        match self {
            Vec4Expr::Variable(_) => {}
            Vec4Expr::Gather1(x) => write!(f, "Gather1({x:?})")?,
            Vec4Expr::Gather4(x, y, z, w) => write!(f, "Gather4({x:?}, {y:?}, {z:?}, {w:?})")?,
            Vec4Expr::AccessMultiVecGroup(mve, i) => write!(f, "AccessMultiVecGroup({mve:?}, {i:?})")?,
            Vec4Expr::Product(v, l) => {
                write!(f, "Product(vec![")?;
                for (e, exp) in v.iter() {
                    write!(f, "({e:?}, {exp}), ")?;
                }
                write!(f, "], {l:?})")?;
            }
            Vec4Expr::Sum(v, l) => {
                write!(f, "Sum(vec![")?;
                for (e, coe) in v.iter() {
                    write!(f, "({e:?}, {coe}), ")?;
                }
                write!(f, "], {l:?})")?;
            }
            Vec4Expr::SwizzleVec4(box v, i0, i1, i2, i3) => write!(f, "swizzle_vec_4({v:?}, {i0}, {i1}, {i2}, {i3})")?,
            Vec4Expr::Extend2to4(v, z, w) => write!(f, "Extend2to4({v:?}, {z:?}, {w:?})")?,
            Vec4Expr::Extend3to4(v, z) => write!(f, "Extend3to4({v:?}, {z:?})")?,
        }
        Ok(())
    }
}

impl Debug for MultiVectorGroupExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MultiVectorGroupExpr::")?;
        match self {
            MultiVectorGroupExpr::JustFloat(v) => write!(f, "JustFloat({v:?})")?,
            MultiVectorGroupExpr::Vec2(v) => write!(f, "Vec2({v:?})")?,
            MultiVectorGroupExpr::Vec3(v) => write!(f, "Vec3({v:?})")?,
            MultiVectorGroupExpr::Vec4(v) => write!(f, "Vec4({v:?})")?,
        }
        Ok(())
    }
}


impl Debug for MultiVectorExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let n = self.mv_class.name();
        write!(f, "{n}(")?;
        let via = self.expr.as_ref();
        if let MultiVectorVia::Variable(v) = &via {
            let (n, i) = &v.decl.name;
            match (n.as_str(), i) {
                ("self", 0) => write!(f, "slf")?,
                (n, 0) => write!(f, "{n}")?,
                (n, i) => write!(f, "{n}_{}", i + 1)?,
            }
            return write!(f, ".clone().into()");
        }
        match via {
            MultiVectorVia::Variable(_) => {}
            MultiVectorVia::Construct(v) => {
                let mut gs = self.mv_class.groups().into_iter();
                write!(f, "vec![")?;
                for (i, expr) in v.iter().enumerate() {
                    let group = gs.next().expect("zipping");
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    use BasisElementGroup::*;
                    use MultiVectorGroupExpr::*;
                    match group {
                        G1(be0) => {
                            write!(f, "/* {be0} */ {expr:?}")?;
                        }
                        G2(be0, be1) => {
                            write!(f, "/* {be0}, {be1} */ {expr:?}")?;
                        }
                        G3(be0, be1, be2) => {
                            write!(f, "/* {be0}, {be1}, {be2} */ {expr:?}")?;
                        }
                        G4(be0, be1, be2, be3) => {
                            write!(f, "/* {be0}, {be1}, {be2}, {be3} */ {expr:?}")?;
                        }
                        _ => unreachable!("mv construction groups must match")
                    }
                }
                write!(f, "]")?;
            }
            // TODO make these trait invoke debugs more similar (see FloatExpr and IntExpr too)
            MultiVectorVia::TraitInvoke11ToClass(t, mv) => {
                let n = t.as_lower_snake();
                write!(f, "({mv} {n})")?
            }
            MultiVectorVia::TraitInvoke21ToClass(t, mva, mvb) => {
                let n = t.as_lower_snake();
                write!(f, "({mva} {n} {mvb})")?
            }
            MultiVectorVia::TraitInvoke22ToClass(t, mva, mvb) => {
                let n = t.as_lower_snake();
                write!(f, "({mva} {n} {mvb})")?
            }
            MultiVectorVia::TraitInvoke12iToClass(t, mva, i) => {
                let n = t.as_lower_snake();
                write!(f, "({mva} {n} {i})")?
            }
            MultiVectorVia::TraitInvoke12fToClass(t, mva, fe) => {
                let n = t.as_lower_snake();
                write!(f, "({mva} {n} {fe})")?
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}