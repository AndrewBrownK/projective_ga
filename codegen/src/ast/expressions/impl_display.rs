impl Display for AnyExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyExpression::Int(e) => write!(f, "{e}")?,
            AnyExpression::Float(e) => write!(f, "{e}")?,
            AnyExpression::Vec2(e) => write!(f, "{e}")?,
            AnyExpression::Vec3(e) => write!(f, "{e}")?,
            AnyExpression::Vec4(e) => write!(f, "{e}")?,
            AnyExpression::Class(e) => write!(f, "{e}")?,
        }
        Ok(())
    }
}
impl Display for IntExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IntExpr::Variable(v) => {
                let (n, i) = &v.decl.name;
                if *i == 0 {
                    write!(f, "{n}")?;
                } else {
                    let i = i + 1;
                    write!(f, "{n}_{i}")?;
                }
            }
            IntExpr::Literal(l) => write!(f, "{l}")?,
            IntExpr::TraitInvoke10ToInt(t, mv) => {
                let n = t.as_lower_snake();
                write!(f, "({mv} {n})")?
            }
        }
        Ok(())
    }
}
impl Display for FloatExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FloatExpr::Variable(v) => {
                let (n, i) = &v.decl.name;
                if *i == 0 {
                    write!(f, "{n}")?;
                } else {
                    let i = i + 1;
                    write!(f, "{n}_{i}")?;
                }
            }
            FloatExpr::Literal(l) => write!(f, "{l}")?,
            FloatExpr::FromInt(i) => write!(f, "{i}")?,
            FloatExpr::AccessVec2(box v, i) => v.display_indexed(f, *i)?,
            FloatExpr::AccessVec3(box v, i) => v.display_indexed(f, *i)?,
            FloatExpr::AccessVec4(box v, i) => v.display_indexed(f, *i)?,
            FloatExpr::AccessMultiVecGroup(mv, i) => {
                let BasisElementGroup::G1(be0) = mv.mv_class.groups()[*i] else {
                    unreachable!(
                        "Should not be able to access FloatExpr as MultiVecGroup \
                        unless the MultiVecGroup is just one Float"
                    )
                };
                match mv.expr.as_ref() {
                    MultiVectorVia::Variable(v) => {
                        let (n, i) = &v.decl.name;
                        if *i == 0 {
                            write!(f, "{n}[{be0}]")?;
                        } else {
                            let i = i + 1;
                            write!(f, "{n}_{i}[{be0}]")?;
                        }
                    }
                    MultiVectorVia::Construct(gs) => match &gs[*i] {
                        MultiVectorGroupExpr::JustFloat(v) => Display::fmt(v, f)?,
                        _ => unreachable!(
                            "Should not be able to access FloatExpr as MultiVecGroup \
                                unless the MultiVecGroup is just one Float"
                        ),
                    },
                    MultiVectorVia::TraitInvoke11ToClass(t, mv) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mv} {n})[{be0}]")?
                    }
                    MultiVectorVia::TraitInvoke21ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {mvb})[{be0}]")?
                    }
                    MultiVectorVia::TraitInvoke22ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {mvb})[{be0}]")?
                    }
                    MultiVectorVia::TraitInvoke12iToClass(t, mva, i) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {i})[{be0}]")?
                    }
                    MultiVectorVia::TraitInvoke12fToClass(t, mva, fe) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {fe})[{be0}]")?
                    }
                }
            }
            FloatExpr::AccessMultiVecFlat(mv, i) => {
                let gs: Vec<_> = mv.elements().collect();
                let (float, el) = &gs[*i];
                match mv.expr.as_ref() {
                    MultiVectorVia::Variable(v) => {
                        let (n, i) = &v.decl.name;
                        if *i == 0 {
                            write!(f, "{n}[{el}]")?;
                        } else {
                            let i = i + 1;
                            write!(f, "{n}_{i}[{el}]")?;
                        }
                    }
                    // TODO is this even right? I think it is a stack overflow
                    _ => write!(f, "{el}({float})")?,
                }
            }
            FloatExpr::TraitInvoke11ToFloat(t, mv) => {
                let n = t.as_lower_snake();
                write!(f, "({mv} {n})")?
            }
            FloatExpr::Product(v, last_factor) => {
                write!(f, "(")?;
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " * ")?;
                    }
                    if *exponent != 1.0 {
                        write!(f, "(")?;
                    }
                    write!(f, "{factor}")?;
                    if *exponent != 1.0 {
                        write!(f, " ^{exponent})")?;
                    }
                }
                if *last_factor != 1.0 {
                    if !v.is_empty() {
                        write!(f, " * ")?;
                    }
                    write!(f, "{last_factor}")?;
                }
                write!(f, ")")?;
            }
            FloatExpr::Sum(v, last_addend) => {
                write!(f, "(")?;
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (fl, _) if fl == 0.0 => continue,

                        (1.0, false) => write!(f, "{addend}")?,
                        (-1.0, false) => write!(f, "-{addend}")?,
                        (fl, false) => write!(f, "{fl}*{addend}")?,

                        (1.0, true) => write!(f, " + {addend}")?,
                        (-1.0, true) => write!(f, " - {addend}")?,
                        (fl, true) if fl > 0.0 => write!(f, " + {fl}*{addend}")?,
                        (fl, true) if fl < 0.0 => {
                            let fl = -fl;
                            write!(f, " - {fl}*{addend}")?
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                }
                match (*last_addend, !v.is_empty()) {
                    (fl, _) if fl == 0.0 => {}
                    (fl, false) => write!(f, "{fl}")?,
                    (fl, true) if fl > 0.0 => write!(f, " + {fl}")?,
                    (fl, true) if fl < 0.0 => {
                        let fl = -fl;
                        write!(f, " - {fl}")?;
                    }
                    _ => {}
                }
                write!(f, ")")?;
            }
            FloatExpr::Exp(box factor, exponent, last_exponent) => {
                write!(f, "({factor} ^ ")?;
                if let Some(box exponent) = exponent {
                    write!(f, "{exponent}")?;
                    if *last_exponent != 1.0 {
                        write!(f, " * ")?;
                    }
                }
                if *last_exponent != 1.0 {
                    write!(f, "{last_exponent}")?;
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}
impl Display for Vec2Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Vec2Expr::Variable(v) => {
                let (n, i) = &v.decl.name;
                if *i == 0 {
                    write!(f, "{n}")?;
                } else {
                    let i = i + 1;
                    write!(f, "{n}_{i}")?;
                }
            }
            Vec2Expr::Gather1(f0) => {
                write!(f, "[{f0}, {f0}]")?;
            }
            Vec2Expr::Gather2(f0, f1) => {
                write!(f, "[{f0}, {f1}]")?;
            }
            Vec2Expr::Truncate3to2(box v3) => {
                write!(f, "[")?;
                v3.display_indexed(f, 0)?;
                write!(f, ", ")?;
                v3.display_indexed(f, 1)?;
                write!(f, "]")?;
            }
            Vec2Expr::Truncate4to2(box v4) => {
                write!(f, "[")?;
                v4.display_indexed(f, 0)?;
                write!(f, ", ")?;
                v4.display_indexed(f, 1)?;
                write!(f, "]")?;
            }
            Vec2Expr::SwizzleVec2(box v, x, y) => {
                write!(f, "[")?;
                v.display_indexed(f, *x)?;
                write!(f, ", ")?;
                v.display_indexed(f, *y)?;
                write!(f, "]")?;
            }
            Vec2Expr::SwizzleVec3(box v, x, y) => {
                write!(f, "[")?;
                v.display_indexed(f, *x)?;
                write!(f, ", ")?;
                v.display_indexed(f, *y)?;
                write!(f, "]")?;
            }
            Vec2Expr::SwizzleVec4(box v, x, y) => {
                write!(f, "[")?;
                v.display_indexed(f, *x)?;
                write!(f, ", ")?;
                v.display_indexed(f, *y)?;
                write!(f, "]")?;
            }
            Vec2Expr::AccessMultiVecGroup(mv, i) => {
                let BasisElementGroup::G2(be0, be1) = mv.mv_class.groups()[*i] else {
                    unreachable!(
                        "Should not be able to access Vec2Expr as MultiVecGroup \
                        unless the MultiVecGroup is Vec2"
                    )
                };
                match mv.expr.as_ref() {
                    MultiVectorVia::Variable(v) => {
                        let (n, i) = &v.decl.name;
                        if *i == 0 {
                            write!(f, "{n}[{be0}, {be1}]")?;
                        } else {
                            let i = i + 1;
                            write!(f, "{n}_{i}[{be0}, {be1}]")?;
                        }
                    }
                    MultiVectorVia::Construct(gs) => match &gs[*i] {
                        MultiVectorGroupExpr::Vec2(v) => Display::fmt(v, f)?,
                        _ => unreachable!(
                            "Should not be able to access Vec2Expr as MultiVecGroup \
                                unless the MultiVecGroup is Vec2"
                        ),
                    },
                    MultiVectorVia::TraitInvoke11ToClass(t, mv) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mv} {n})[{be0}, {be1}]")?
                    }
                    MultiVectorVia::TraitInvoke21ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {mvb})[{be0}, {be1}]")?
                    }
                    MultiVectorVia::TraitInvoke22ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {mvb})[{be0}, {be1}]")?
                    }
                    MultiVectorVia::TraitInvoke12iToClass(t, mva, i) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {i})[{be0}, {be1}]")?
                    }
                    MultiVectorVia::TraitInvoke12fToClass(t, mva, fe) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {fe})[{be0}, {be1}]")?
                    }
                }
            }
            Vec2Expr::Product(v, last_factor) => {
                write!(f, "(")?;
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " * ")?;
                    }
                    if *exponent != 1.0 {
                        write!(f, "(")?;
                    }
                    write!(f, "{factor}")?;
                    if *exponent != 1.0 {
                        write!(f, " ^{exponent})")?;
                    }
                }
                if *last_factor != [1.0; 2] {
                    if !v.is_empty() {
                        write!(f, " * ")?;
                    }
                    let a0 = last_factor[0];
                    let a1 = last_factor[1];
                    write!(f, "[{a0}, {a1}]")?;
                }
                write!(f, ")")?;
            }
            Vec2Expr::Sum(v, last_addend) => {
                write!(f, "(")?;
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (fl, _) if fl == 0.0 => continue,

                        (1.0, false) => write!(f, "{addend}")?,
                        (-1.0, false) => write!(f, "-{addend}")?,
                        (fl, false) => write!(f, "{fl}*{addend}")?,

                        (1.0, true) => write!(f, " + {addend}")?,
                        (-1.0, true) => write!(f, " - {addend}")?,
                        (fl, true) if fl > 0.0 => write!(f, " + {fl}*{addend}")?,
                        (fl, true) if fl < 0.0 => {
                            let fl = -fl;
                            write!(f, " - {fl}*{addend}")?
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                }
                if *last_addend != [0.0; 2] {
                    if !v.is_empty() {
                        write!(f, " + ")?;
                    }
                    let a0 = last_addend[0];
                    let a1 = last_addend[1];
                    write!(f, "[{a0}, {a1}]")?;
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}
impl Display for Vec3Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Vec3Expr::Variable(v) => {
                let (n, i) = &v.decl.name;
                if *i == 0 {
                    write!(f, "{n}")?;
                } else {
                    let i = i + 1;
                    write!(f, "{n}_{i}")?;
                }
            }
            Vec3Expr::Gather1(f0) => {
                write!(f, "[{f0}, {f0}, {f0}]")?;
            }
            Vec3Expr::Gather3(f0, f1, f2) => {
                write!(f, "[{f0}, {f1}, {f2}]")?;
            }
            Vec3Expr::Extend2to3(v, z) => {
                write!(f, "[")?;
                v.display_indexed(f, 0)?;
                write!(f, ", ")?;
                v.display_indexed(f, 1)?;
                write!(f, ", {z}]")?;
            }
            Vec3Expr::Truncate4to3(v4) => {
                write!(f, "[")?;
                v4.display_indexed(f, 0)?;
                write!(f, ", ")?;
                v4.display_indexed(f, 1)?;
                write!(f, ", ")?;
                v4.display_indexed(f, 2)?;
                write!(f, "]")?;
            }
            Vec3Expr::SwizzleVec2(v, x, y , z) => {
                write!(f, "[")?;
                v.display_indexed(f, *x)?;
                write!(f, ", ")?;
                v.display_indexed(f, *y)?;
                write!(f, ", ")?;
                v.display_indexed(f, *z)?;
                write!(f, "]")?;
            }
            Vec3Expr::SwizzleVec3(box v, x, y , z) => {
                write!(f, "[")?;
                v.display_indexed(f, *x)?;
                write!(f, ", ")?;
                v.display_indexed(f, *y)?;
                write!(f, ", ")?;
                v.display_indexed(f, *z)?;
                write!(f, "]")?;
            }
            Vec3Expr::SwizzleVec4(box v, x, y , z) => {
                write!(f, "[")?;
                v.display_indexed(f, *x)?;
                write!(f, ", ")?;
                v.display_indexed(f, *y)?;
                write!(f, ", ")?;
                v.display_indexed(f, *z)?;
                write!(f, "]")?;
            }
            Vec3Expr::AccessMultiVecGroup(mv, i) => {
                let BasisElementGroup::G3(be0, be1, be2) = mv.mv_class.groups()[*i] else {
                    unreachable!(
                        "Should not be able to access Vec3Expr as MultiVecGroup \
                        unless the MultiVecGroup is Vec3"
                    )
                };
                match mv.expr.as_ref() {
                    MultiVectorVia::Variable(v) => {
                        let (n, i) = &v.decl.name;
                        if *i == 0 {
                            write!(f, "{n}[{be0}, {be1}, {be2}]")?;
                        } else {
                            let i = i + 1;
                            write!(f, "{n}_{i}[{be0}, {be1}, {be2}]")?;
                        }
                    }
                    MultiVectorVia::Construct(gs) => match &gs[*i] {
                        MultiVectorGroupExpr::Vec3(v) => Display::fmt(v, f)?,
                        _ => unreachable!(
                            "Should not be able to access Vec3Expr as MultiVecGroup \
                                unless the MultiVecGroup is Vec3"
                        ),
                    },
                    MultiVectorVia::TraitInvoke11ToClass(t, mv) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mv} {n})[{be0}, {be1}, {be2}]")?
                    }
                    MultiVectorVia::TraitInvoke21ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {mvb})[{be0}, {be1}, {be2}]")?
                    }
                    MultiVectorVia::TraitInvoke22ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {mvb})[{be0}, {be1}, {be2}]")?
                    }
                    MultiVectorVia::TraitInvoke12iToClass(t, mva, i) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {i})[{be0}, {be1}, {be2}]")?
                    }
                    MultiVectorVia::TraitInvoke12fToClass(t, mva, fe) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {fe})[{be0}, {be1}, {be2}]")?
                    }
                }
            }
            Vec3Expr::Product(v, last_factor) => {
                write!(f, "(")?;
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " * ")?;
                    }
                    if *exponent != 1.0 {
                        write!(f, "(")?;
                    }
                    write!(f, "{factor}")?;
                    if *exponent != 1.0 {
                        write!(f, " ^{exponent})")?;
                    }
                }
                if *last_factor != [1.0; 3] {
                    if !v.is_empty() {
                        write!(f, " * ")?;
                    }
                    let a0 = last_factor[0];
                    let a1 = last_factor[1];
                    let a2 = last_factor[2];
                    write!(f, "[{a0}, {a1}, {a2}]")?;
                }
                write!(f, ")")?;
            }
            Vec3Expr::Sum(v, last_addend) => {
                write!(f, "(")?;
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => write!(f, "{addend}")?,
                        (-1.0, false) => write!(f, "-{addend}")?,
                        (fl, false) => write!(f, "{fl}*{addend}")?,

                        (1.0, true) => write!(f, " + {addend}")?,
                        (-1.0, true) => write!(f, " - {addend}")?,
                        (fl, true) if fl > 0.0 => write!(f, " + {fl}*{addend}")?,
                        (fl, true) if fl < 0.0 => {
                            let fl = -fl;
                            write!(f, " - {fl}*{addend}")?
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                }
                if *last_addend != [0.0; 3] {
                    if !v.is_empty() {
                        write!(f, " + ")?;
                    }
                    let a0 = last_addend[0];
                    let a1 = last_addend[1];
                    let a2 = last_addend[2];
                    write!(f, "[{a0}, {a1}, {a2}]")?;
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}
impl Display for Vec4Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Vec4Expr::Variable(v) => {
                let (n, i) = &v.decl.name;
                if *i == 0 {
                    write!(f, "{n}")?;
                } else {
                    let i = i + 1;
                    write!(f, "{n}_{i}")?;
                }
            }
            Vec4Expr::Gather1(f0) => {
                write!(f, "[{f0}, {f0}, {f0}, {f0}]")?;
            }
            Vec4Expr::Gather4(f0, f1, f2, f3) => {
                write!(f, "[{f0}, {f1}, {f2}, {f3}]")?;
            }
            Vec4Expr::Extend2to4(v, z, w) => {
                write!(f, "[")?;
                v.display_indexed(f, 0)?;
                write!(f, ", ")?;
                v.display_indexed(f, 1)?;
                write!(f, ", {z}, {w}]")?;
            }
            Vec4Expr::Extend3to4(v, w) => {
                write!(f, "[")?;
                v.display_indexed(f, 0)?;
                write!(f, ", ")?;
                v.display_indexed(f, 1)?;
                write!(f, ", ")?;
                v.display_indexed(f, 2)?;
                write!(f, ", {w}]")?;
            }
            Vec4Expr::SwizzleVec2(v, x, y, z, w) => {
                write!(f, "[")?;
                v.display_indexed(f, *x)?;
                write!(f, ", ")?;
                v.display_indexed(f, *y)?;
                write!(f, ", ")?;
                v.display_indexed(f, *z)?;
                write!(f, ", ")?;
                v.display_indexed(f, *w)?;
                write!(f, "]")?;
            }
            Vec4Expr::SwizzleVec3(v, x, y, z, w) => {
                write!(f, "[")?;
                v.display_indexed(f, *x)?;
                write!(f, ", ")?;
                v.display_indexed(f, *y)?;
                write!(f, ", ")?;
                v.display_indexed(f, *z)?;
                write!(f, ", ")?;
                v.display_indexed(f, *w)?;
                write!(f, "]")?;
            }
            Vec4Expr::SwizzleVec4(box v, x, y, z, w) => {
                write!(f, "[")?;
                v.display_indexed(f, *x)?;
                write!(f, ", ")?;
                v.display_indexed(f, *y)?;
                write!(f, ", ")?;
                v.display_indexed(f, *z)?;
                write!(f, ", ")?;
                v.display_indexed(f, *w)?;
                write!(f, "]")?;
            }
            Vec4Expr::AccessMultiVecGroup(mv, i) => {
                let BasisElementGroup::G4(be0, be1, be2, be3) = mv.mv_class.groups()[*i] else {
                    unreachable!(
                        "Should not be able to access Vec4Expr as MultiVecGroup \
                        unless the MultiVecGroup is Vec4"
                    )
                };
                match mv.expr.as_ref() {
                    MultiVectorVia::Variable(v) => {
                        let (n, i) = &v.decl.name;
                        if *i == 0 {
                            write!(f, "{n}[{be0}, {be1}, {be2}, {be3}]")?;
                        } else {
                            let i = i + 1;
                            write!(f, "{n}_{i}[{be0}, {be1}, {be2}, {be3}]")?;
                        }
                    }
                    MultiVectorVia::Construct(gs) => match &gs[*i] {
                        MultiVectorGroupExpr::Vec4(v) => Display::fmt(v, f)?,
                        _ => unreachable!(
                            "Should not be able to access Vec4Expr as MultiVecGroup \
                                unless the MultiVecGroup is Vec4"
                        ),
                    },
                    MultiVectorVia::TraitInvoke11ToClass(t, mv) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mv} {n})[{be0}, {be1}, {be2}, {be3}]")?
                    }
                    MultiVectorVia::TraitInvoke21ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {mvb})[{be0}, {be1}, {be2}, {be3}]")?
                    }
                    MultiVectorVia::TraitInvoke22ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {mvb})[{be0}, {be1}, {be2}, {be3}]")?
                    }
                    MultiVectorVia::TraitInvoke12iToClass(t, mva, i) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {i})[{be0}, {be1}, {be2}, {be3}]")?
                    }
                    MultiVectorVia::TraitInvoke12fToClass(t, mva, fe) => {
                        let n = t.as_lower_snake();
                        write!(f, "({mva} {n} {fe})[{be0}, {be1}, {be2}, {be3}]")?
                    }
                }
            }
            Vec4Expr::Product(v, last_factor) => {
                write!(f, "(")?;
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " * ")?;
                    }
                    if *exponent != 1.0 {
                        write!(f, "(")?;
                    }
                    write!(f, "{factor}")?;
                    if *exponent != 1.0 {
                        write!(f, " ^{exponent})")?;
                    }
                }
                if *last_factor != [1.0; 4] {
                    if !v.is_empty() {
                        write!(f, " * ")?;
                    }
                    let a0 = last_factor[0];
                    let a1 = last_factor[1];
                    let a2 = last_factor[2];
                    let a3 = last_factor[3];
                    write!(f, "[{a0}, {a1}, {a2}, {a3}]")?;
                }
                write!(f, ")")?;
            }
            Vec4Expr::Sum(v, last_addend) => {
                write!(f, "(")?;
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (fl, _) if fl == 0.0 => continue,

                        (1.0, false) => write!(f, "{addend}")?,
                        (-1.0, false) => write!(f, "-{addend}")?,
                        (fl, false) => write!(f, "{fl}*{addend}")?,

                        (1.0, true) => write!(f, " + {addend}")?,
                        (-1.0, true) => write!(f, " - {addend}")?,
                        (fl, true) if fl > 0.0 => write!(f, " + {fl}*{addend}")?,
                        (fl, true) if fl < 0.0 => {
                            let fl = -fl;
                            write!(f, " - {fl}*{addend}")?
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                }
                if *last_addend != [0.0; 4] {
                    if !v.is_empty() {
                        write!(f, " + ")?;
                    }
                    let a0 = last_addend[0];
                    let a1 = last_addend[1];
                    let a2 = last_addend[2];
                    let a3 = last_addend[3];
                    write!(f, "[{a0}, {a1}, {a2}, {a3}]")?;
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}

impl Display for MultiVectorExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let n = self.mv_class.name();
        write!(f, "{n}( ")?;
        let via = self.expr.as_ref();
        match via {
            MultiVectorVia::Variable(v) => {
                let (n, i) = &v.decl.name;
                if *i == 0 {
                    write!(f, "{n}")?;
                } else {
                    let i = i + 1;
                    write!(f, "{n}_{i}")?;
                }
            }
            MultiVectorVia::Construct(v) => {
                let mut gs = self.mv_class.groups().into_iter();
                for (i, expr) in v.iter().enumerate() {
                    let group = gs.next().expect("zipping");
                    if i > 0 {
                        write!(f, ",\n")?;
                    }
                    use BasisElementGroup::*;
                    use MultiVectorGroupExpr::*;
                    match (group, expr) {
                        (G1(be0), JustFloat(f0)) => {
                            write!(f, "{be0}: {f0}")?;
                        }
                        (G2(be0, be1), Vec2(v2)) => {
                            write!(f, "{be0}: ")?;
                            v2.display_indexed(f, 0)?;
                            write!(f, ",\n {be1}: ")?;
                            v2.display_indexed(f, 1)?;
                            write!(f, "")?;
                        }
                        (G3(be0, be1, be2), Vec3(v3)) => {
                            write!(f, "{be0}: ")?;
                            v3.display_indexed(f, 0)?;
                            write!(f, ",\n{be1}: ")?;
                            v3.display_indexed(f, 1)?;
                            write!(f, ",\n{be2}: ")?;
                            v3.display_indexed(f, 2)?;
                            write!(f, "")?;
                        }
                        (G4(be0, be1, be2, be3), Vec4(v4)) => {
                            write!(f, "{be0}: ")?;
                            v4.display_indexed(f, 0)?;
                            write!(f, ",\n{be1}: ")?;
                            v4.display_indexed(f, 1)?;
                            write!(f, ",\n{be2}: ")?;
                            v4.display_indexed(f, 2)?;
                            write!(f, ",\n{be3}: ")?;
                            v4.display_indexed(f, 3)?;
                            write!(f, "")?;
                        }
                        _ => unreachable!("mv construction groups must match")
                    }
                }
            }
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
        write!(f, " )")?;
        Ok(())
    }
}


impl Vec2Expr {
    pub fn display_indexed(&self, f: &mut Formatter<'_>, idx: usize) -> std::fmt::Result {
        match self {
            Vec2Expr::Variable(v) => {
                let (n, i) = &v.decl.name;
                if *i == 0 {
                    write!(f, "{n}")?;
                } else {
                    let i = i + 1;
                    write!(f, "{n}_{i}")?;
                }
                write!(f, "[{idx}]")?;
            }
            Vec2Expr::Gather1(f0) => {
                write!(f, "{f0}")?;
            }
            Vec2Expr::Gather2(f0, f1) => {
                if idx == 0 {
                    write!(f, "{f0}")?;
                }
                if idx == 1 {
                    write!(f, "{f1}")?;
                }
            }
            Vec2Expr::Truncate3to2(v) => {
                v.display_indexed(f, idx)?;
            }
            Vec2Expr::Truncate4to2(v) => {
                v.display_indexed(f, idx)?;
            }
            Vec2Expr::SwizzleVec2(v, i0, i1) => {
                if idx == 0 {
                    v.display_indexed(f, *i0)?;
                }
                if idx == 1 {
                    v.display_indexed(f, *i1)?;
                }
            }
            Vec2Expr::SwizzleVec3(v, i0, i1) => {
                if idx == 0 {
                    v.display_indexed(f, *i0)?;
                }
                if idx == 1 {
                    v.display_indexed(f, *i1)?;
                }
            }
            Vec2Expr::SwizzleVec4(v, i0, i1) => {
                if idx == 0 {
                    v.display_indexed(f, *i0)?;
                }
                if idx == 1 {
                    v.display_indexed(f, *i1)?;
                }
            }
            Vec2Expr::AccessMultiVecGroup(mv, i) => {
                let BasisElementGroup::G2(be0, be1) = mv.mv_class.groups()[*i] else {
                    unreachable!(
                        "Should not be able to access Vec2Expr as MultiVecGroup \
                        unless the MultiVecGroup is Vec2"
                    )
                };
                match mv.expr.as_ref() {
                    MultiVectorVia::Variable(v) => {
                        let (n, i) = &v.decl.name;
                        if *i == 0 {
                            write!(f, "{n}[")?;
                        } else {
                            let i = i + 1;
                            write!(f, "{n}_{i}[")?;
                        }
                        if idx == 0 {
                            write!(f, "{be0}]")?;
                        }
                        if idx == 1 {
                            write!(f, "{be1}]")?;
                        }
                    }
                    MultiVectorVia::Construct(gs) => match &gs[*i] {
                        MultiVectorGroupExpr::Vec2(v) => Display::fmt(v, f)?,
                        _ => unreachable!(
                            "Should not be able to access Vec2Expr as MultiVecGroup \
                                unless the MultiVecGroup is Vec2"
                        ),
                    },
                    MultiVectorVia::TraitInvoke11ToClass(t, mv) => {
                        let n = t.as_lower_snake();
                        if idx == 0 {
                            write!(f, "({mv} {n})[{be0}]")?;
                        }
                        if idx == 1 {
                            write!(f, "({mv} {n})[{be1}]")?;
                        }
                    }
                    MultiVectorVia::TraitInvoke21ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        if idx == 0 {
                            write!(f, "({mva} {n} {mvb})[{be0}]")?;
                        }
                        if idx == 1 {
                            write!(f, "({mva} {n} {mvb})[{be1}]")?;
                        }
                    }
                    MultiVectorVia::TraitInvoke22ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        if idx == 0 {
                            write!(f, "({mva} {n} {mvb})[{be0}]")?;
                        }
                        if idx == 1 {
                            write!(f, "({mva} {n} {mvb})[{be1}]")?;
                        }
                    }
                    MultiVectorVia::TraitInvoke12iToClass(t, mva, i) => {
                        let n = t.as_lower_snake();
                        if idx == 0 {
                            write!(f, "({mva} {n} {i})[{be0}]")?;
                        }
                        if idx == 1 {
                            write!(f, "({mva} {n} {i})[{be1}]")?;
                        }
                    }
                    MultiVectorVia::TraitInvoke12fToClass(t, mva, fe) => {
                        let n = t.as_lower_snake();
                        if idx == 0 {
                            write!(f, "({mva} {n} {fe})[{be0}]")?;
                        }
                        if idx == 1 {
                            write!(f, "({mva} {n} {fe})[{be1}]")?;
                        }
                    }
                }
            }
            Vec2Expr::Product(v, last_factor) => {
                write!(f, "(")?;
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " * ")?;
                    }
                    if *exponent != 1.0 {
                        write!(f, "(")?;
                    }
                    factor.display_indexed(f, idx)?;
                    if *exponent != 1.0 {
                        write!(f, " ^{exponent})")?;
                    }
                }
                let a = last_factor[idx];
                if a != 1.0 {
                    if !v.is_empty() {
                        write!(f, " * ")?;
                    }
                    write!(f, "{a}")?;
                }
                write!(f, ")")?;
            }
            Vec2Expr::Sum(v, last_addend) => {
                write!(f, "(")?;
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (fl, _) if fl == 0.0 => continue,

                        (1.0, false) => {},
                        (-1.0, false) => write!(f, "-")?,
                        (fl, false) => write!(f, "{fl}*")?,

                        (1.0, true) => write!(f, " + ")?,
                        (-1.0, true) => write!(f, " - ")?,
                        (fl, true) if fl > 0.0 => write!(f, " + {fl}*")?,
                        (fl, true) if fl < 0.0 => {
                            let fl = -fl;
                            write!(f, " - {fl}*")?
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                    addend.display_indexed(f, idx)?;
                }
                let a = last_addend[idx];
                if a != 0.0 {
                    if !v.is_empty() {
                        write!(f, " + ")?;
                    }
                    write!(f, "{a}")?;
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}
impl Vec3Expr {
    pub fn display_indexed(&self, f: &mut Formatter<'_>, idx: usize) -> std::fmt::Result {
        match self {
            Vec3Expr::Variable(v) => {
                let (n, i) = &v.decl.name;
                if *i == 0 {
                    write!(f, "{n}")?;
                } else {
                    let i = i + 1;
                    write!(f, "{n}_{i}")?;
                }
                write!(f, "[{idx}]")?;
            }
            Vec3Expr::Gather1(f0) => {
                write!(f, "{f0}")?;
            }
            Vec3Expr::Gather3(f0, f1, f2) => {
                if idx == 0 {
                    write!(f, "{f0}")?;
                }
                if idx == 1 {
                    write!(f, "{f1}")?;
                }
                if idx == 2 {
                    write!(f, "{f2}")?;
                }
            }
            Vec3Expr::Extend2to3(v2, f1) => {
                if idx < 2 {
                    v2.display_indexed(f, idx)?;
                }
                if idx == 2 {
                    write!(f, "{f1}")?;
                }
            }
            Vec3Expr::Truncate4to3(v) => {
                v.display_indexed(f, idx)?;
            }
            Vec3Expr::SwizzleVec2(v, i0, i1, i2) => {
                if idx == 0 {
                    v.display_indexed(f, *i0)?;
                }
                if idx == 1 {
                    v.display_indexed(f, *i1)?;
                }
                if idx == 2 {
                    v.display_indexed(f, *i2)?;
                }
            }
            Vec3Expr::SwizzleVec3(v, i0, i1, i2) => {
                if idx == 0 {
                    v.display_indexed(f, *i0)?;
                }
                if idx == 1 {
                    v.display_indexed(f, *i1)?;
                }
                if idx == 2 {
                    v.display_indexed(f, *i2)?;
                }
            }
            Vec3Expr::SwizzleVec4(v, i0, i1, i2) => {
                if idx == 0 {
                    v.display_indexed(f, *i0)?;
                }
                if idx == 1 {
                    v.display_indexed(f, *i1)?;
                }
                if idx == 2 {
                    v.display_indexed(f, *i2)?;
                }
            }
            Vec3Expr::AccessMultiVecGroup(mv, i) => {
                let BasisElementGroup::G3(be0, be1, be2) = mv.mv_class.groups()[*i] else {
                    unreachable!(
                        "Should not be able to access Vec3Expr as MultiVecGroup \
                        unless the MultiVecGroup is Vec3"
                    )
                };
                match mv.expr.as_ref() {
                    MultiVectorVia::Variable(v) => {
                        let (n, i) = &v.decl.name;
                        if *i == 0 {
                            write!(f, "{n}[")?;
                        } else {
                            let i = i + 1;
                            write!(f, "{n}_{i}[")?;
                        }
                        if idx == 0 {
                            write!(f, "{be0}]")?;
                        }
                        if idx == 1 {
                            write!(f, "{be1}]")?;
                        }
                        if idx == 2 {
                            write!(f, "{be2}]")?;
                        }
                    }
                    MultiVectorVia::Construct(gs) => match &gs[*i] {
                        MultiVectorGroupExpr::Vec3(v) => Display::fmt(v, f)?,
                        _ => unreachable!(
                            "Should not be able to access Vec3Expr as MultiVecGroup \
                                unless the MultiVecGroup is Vec3"
                        ),
                    },
                    MultiVectorVia::TraitInvoke11ToClass(t, mv) => {
                        let n = t.as_lower_snake();
                        if idx == 0 {
                            write!(f, "({mv} {n})[{be0}]")?;
                        }
                        if idx == 1 {
                            write!(f, "({mv} {n})[{be1}]")?;
                        }
                        if idx == 2 {
                            write!(f, "({mv} {n})[{be2}]")?;
                        }
                    }
                    MultiVectorVia::TraitInvoke21ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        if idx == 0 {
                            write!(f, "({mva} {n} {mvb})[{be0}]")?;
                        }
                        if idx == 1 {
                            write!(f, "({mva} {n} {mvb})[{be1}]")?;
                        }
                        if idx == 2 {
                            write!(f, "({mva} {n} {mvb})[{be2}]")?;
                        }
                    }
                    MultiVectorVia::TraitInvoke22ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        if idx == 0 {
                            write!(f, "({mva} {n} {mvb})[{be0}]")?;
                        }
                        if idx == 1 {
                            write!(f, "({mva} {n} {mvb})[{be1}]")?;
                        }
                        if idx == 2 {
                            write!(f, "({mva} {n} {mvb})[{be2}]")?;
                        }
                    }
                    MultiVectorVia::TraitInvoke12iToClass(t, mva, i) => {
                        let n = t.as_lower_snake();
                        if idx == 0 {
                            write!(f, "({mva} {n} {i})[{be0}]")?;
                        }
                        if idx == 1 {
                            write!(f, "({mva} {n} {i})[{be1}]")?;
                        }
                        if idx == 2 {
                            write!(f, "({mva} {n} {i})[{be2}]")?;
                        }
                    }
                    MultiVectorVia::TraitInvoke12fToClass(t, mva, fe) => {
                        let n = t.as_lower_snake();
                        if idx == 0 {
                            write!(f, "({mva} {n} {fe})[{be0}]")?;
                        }
                        if idx == 1 {
                            write!(f, "({mva} {n} {fe})[{be1}]")?;
                        }
                        if idx == 2 {
                            write!(f, "({mva} {n} {fe})[{be2}]")?;
                        }
                    }
                }
            }
            Vec3Expr::Product(v, last_factor) => {
                write!(f, "(")?;
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " * ")?;
                    }
                    if *exponent != 1.0 {
                        write!(f, "(")?;
                    }
                    factor.display_indexed(f, idx)?;
                    if *exponent != 1.0 {
                        write!(f, " ^{exponent})")?;
                    }
                }
                let a = last_factor[idx];
                if a != 1.0 {
                    if !v.is_empty() {
                        write!(f, " * ")?;
                    }
                    write!(f, "{a}")?;
                }
                write!(f, ")")?;
            }
            Vec3Expr::Sum(v, last_addend) => {
                write!(f, "(")?;
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => {},
                        (-1.0, false) => write!(f, "-")?,
                        (fl, false) => write!(f, "{fl}*")?,

                        (1.0, true) => write!(f, " + ")?,
                        (-1.0, true) => write!(f, " - ")?,
                        (fl, true) if fl > 0.0 => write!(f, " + {fl}*")?,
                        (fl, true) if fl < 0.0 => {
                            let fl = -fl;
                            write!(f, " - {fl}*")?
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                    addend.display_indexed(f, idx)?;
                }
                let a = last_addend[idx];
                if a != 0.0 {
                    if !v.is_empty() {
                        write!(f, " + ")?;
                    }
                    write!(f, "{a}")?;
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}
impl Vec4Expr {
    pub fn display_indexed(&self, f: &mut Formatter<'_>, idx: usize) -> std::fmt::Result {
        match self {
            Vec4Expr::Variable(v) => {
                let (n, i) = &v.decl.name;
                if *i == 0 {
                    write!(f, "{n}")?;
                } else {
                    let i = i + 1;
                    write!(f, "{n}_{i}")?;
                }
                write!(f, "[{idx}]")?;
            }
            Vec4Expr::Gather1(f0) => {
                write!(f, "{f0}")?;
            }
            Vec4Expr::Gather4(f0, f1, f2, f3) => {
                if idx == 0 {
                    write!(f, "{f0}")?;
                }
                if idx == 1 {
                    write!(f, "{f1}")?;
                }
                if idx == 2 {
                    write!(f, "{f2}")?;
                }
                if idx == 3 {
                    write!(f, "{f3}")?;
                }
            }
            Vec4Expr::Extend2to4(v2, f1, f2) => {
                if idx < 2 {
                    v2.display_indexed(f, idx)?;
                }
                if idx == 2 {
                    write!(f, "{f1}")?;
                }
                if idx == 3 {
                    write!(f, "{f2}")?;
                }
            }
            Vec4Expr::Extend3to4(v3, f1) => {
                if idx < 3 {
                    v3.display_indexed(f, idx)?;
                }
                if idx == 3 {
                    write!(f, "{f1}")?;
                }
            }
            Vec4Expr::SwizzleVec2(v, i0, i1, i2, i3) => {
                if idx == 0 {
                    v.display_indexed(f, *i0)?;
                }
                if idx == 1 {
                    v.display_indexed(f, *i1)?;
                }
                if idx == 2 {
                    v.display_indexed(f, *i2)?;
                }
                if idx == 3 {
                    v.display_indexed(f, *i3)?;
                }
            }
            Vec4Expr::SwizzleVec3(v, i0, i1, i2, i3) => {
                if idx == 0 {
                    v.display_indexed(f, *i0)?;
                }
                if idx == 1 {
                    v.display_indexed(f, *i1)?;
                }
                if idx == 2 {
                    v.display_indexed(f, *i2)?;
                }
                if idx == 3 {
                    v.display_indexed(f, *i3)?;
                }
            }
            Vec4Expr::SwizzleVec4(v, i0, i1, i2, i3) => {
                if idx == 0 {
                    v.display_indexed(f, *i0)?;
                }
                if idx == 1 {
                    v.display_indexed(f, *i1)?;
                }
                if idx == 2 {
                    v.display_indexed(f, *i2)?;
                }
                if idx == 3 {
                    v.display_indexed(f, *i3)?;
                }
            }
            Vec4Expr::AccessMultiVecGroup(mv, i) => {
                let BasisElementGroup::G4(be0, be1, be2, be3) = mv.mv_class.groups()[*i] else {
                    unreachable!(
                        "Should not be able to access Vec4Expr as MultiVecGroup \
                        unless the MultiVecGroup is Vec4"
                    )
                };
                match mv.expr.as_ref() {
                    MultiVectorVia::Variable(v) => {
                        let (n, i) = &v.decl.name;
                        if *i == 0 {
                            write!(f, "{n}[")?;
                        } else {
                            let i = i + 1;
                            write!(f, "{n}_{i}[")?;
                        }
                        if idx == 0 {
                            write!(f, "{be0}]")?;
                        }
                        if idx == 1 {
                            write!(f, "{be1}]")?;
                        }
                        if idx == 2 {
                            write!(f, "{be2}]")?;
                        }
                        if idx == 3 {
                            write!(f, "{be3}]")?;
                        }
                    }
                    MultiVectorVia::Construct(gs) => match &gs[*i] {
                        MultiVectorGroupExpr::Vec4(v) => Display::fmt(v, f)?,
                        _ => unreachable!(
                            "Should not be able to access Vec4Expr as MultiVecGroup \
                                unless the MultiVecGroup is Vec4"
                        ),
                    },
                    MultiVectorVia::TraitInvoke11ToClass(t, mv) => {
                        let n = t.as_lower_snake();
                        if idx == 0 {
                            write!(f, "({mv} {n})[{be0}]")?;
                        }
                        if idx == 1 {
                            write!(f, "({mv} {n})[{be1}]")?;
                        }
                        if idx == 2 {
                            write!(f, "({mv} {n})[{be2}]")?;
                        }
                        if idx == 3 {
                            write!(f, "({mv} {n})[{be3}]")?;
                        }
                    }
                    MultiVectorVia::TraitInvoke21ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        if idx == 0 {
                            write!(f, "({mva} {n} {mvb})[{be0}]")?;
                        }
                        if idx == 1 {
                            write!(f, "({mva} {n} {mvb})[{be1}]")?;
                        }
                        if idx == 2 {
                            write!(f, "({mva} {n} {mvb})[{be2}]")?;
                        }
                        if idx == 3 {
                            write!(f, "({mva} {n} {mvb})[{be3}]")?;
                        }
                    }
                    MultiVectorVia::TraitInvoke22ToClass(t, mva, mvb) => {
                        let n = t.as_lower_snake();
                        if idx == 0 {
                            write!(f, "({mva} {n} {mvb})[{be0}]")?;
                        }
                        if idx == 1 {
                            write!(f, "({mva} {n} {mvb})[{be1}]")?;
                        }
                        if idx == 2 {
                            write!(f, "({mva} {n} {mvb})[{be2}]")?;
                        }
                        if idx == 3 {
                            write!(f, "({mva} {n} {mvb})[{be3}]")?;
                        }
                    }
                    MultiVectorVia::TraitInvoke12iToClass(t, mva, i) => {
                        let n = t.as_lower_snake();
                        if idx == 0 {
                            write!(f, "({mva} {n} {i})[{be0}]")?;
                        }
                        if idx == 1 {
                            write!(f, "({mva} {n} {i})[{be1}]")?;
                        }
                        if idx == 2 {
                            write!(f, "({mva} {n} {i})[{be2}]")?;
                        }
                        if idx == 3 {
                            write!(f, "({mva} {n} {i})[{be3}]")?;
                        }
                    }
                    MultiVectorVia::TraitInvoke12fToClass(t, mva, fe) => {
                        let n = t.as_lower_snake();
                        if idx == 0 {
                            write!(f, "({mva} {n} {fe})[{be0}]")?;
                        }
                        if idx == 1 {
                            write!(f, "({mva} {n} {fe})[{be1}]")?;
                        }
                        if idx == 2 {
                            write!(f, "({mva} {n} {fe})[{be2}]")?;
                        }
                        if idx == 3 {
                            write!(f, "({mva} {n} {fe})[{be3}]")?;
                        }
                    }
                }
            }
            Vec4Expr::Product(v, last_factor) => {
                write!(f, "(")?;
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " * ")?;
                    }
                    if *exponent != 1.0 {
                        write!(f, "(")?;
                    }
                    factor.display_indexed(f, idx)?;
                    if *exponent != 1.0 {
                        write!(f, " ^{exponent})")?;
                    }
                }
                let a = last_factor[idx];
                if a != 1.0 {
                    if !v.is_empty() {
                        write!(f, " * ")?;
                    }
                    write!(f, "{a}")?;
                }
                write!(f, ")")?;
            }
            Vec4Expr::Sum(v, last_addend) => {
                write!(f, "(")?;
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (fl, _) if fl == 0.0 => continue,

                        (1.0, false) => {},
                        (-1.0, false) => write!(f, "-")?,
                        (fl, false) => write!(f, "{fl}*")?,

                        (1.0, true) => write!(f, " + ")?,
                        (-1.0, true) => write!(f, " - ")?,
                        (fl, true) if fl > 0.0 => write!(f, " + {fl}*")?,
                        (fl, true) if fl < 0.0 => {
                            let fl = -fl;
                            write!(f, " - {fl}*")?;
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                    addend.display_indexed(f, idx)?;
                }
                let a = last_addend[idx];
                if a != 0.0 {
                    if !v.is_empty() {
                        write!(f, " + ")?;
                    }
                    write!(f, "{a}")?;
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}