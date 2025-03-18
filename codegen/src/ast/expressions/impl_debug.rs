
#[derive(Clone, Copy)]
pub struct DebugExpression<'e, E> {
    multi_line: bool,
    indent_level: usize,
    expr: &'e E,
}
impl<'e, E> DebugExpression<'e, E> {
    pub fn new(multi_line: bool, expr: &'e E) -> Self {
        DebugExpression {
            multi_line,
            indent_level: 0,
            expr,
        }
    }

    fn also<'e2, E2>(&self, expr: &'e2 E2) -> DebugExpression<'e2, E2> {
        DebugExpression {
            multi_line: self.multi_line,
            indent_level: self.indent_level,
            expr,
        }
    }

    fn also_deeper<'e2, E2>(&self, expr: &'e2 E2) -> DebugExpression<'e2, E2> {
        DebugExpression {
            multi_line: self.multi_line,
            indent_level: self.indent_level + 1,
            expr,
        }
    }

    fn this_newline_and_indent(&self) -> String {
        if self.multi_line {
            format!("\n{}", "    ".repeat(self.indent_level))
        } else {
            " ".to_string()
        }
    }

    fn inner_newline_and_indent(&self) -> String {
        if self.multi_line {
            format!("\n{}", "    ".repeat(self.indent_level + 1))
        } else {
            " ".to_string()
        }
    }
}


impl Debug for AnyExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        DebugExpression::new(false, self).fmt(f)
    }
}
impl Debug for IntExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        DebugExpression::new(false, self).fmt(f)
    }
}
impl Debug for FloatExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        DebugExpression::new(false, self).fmt(f)
    }
}
impl Debug for Vec2Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        DebugExpression::new(false, self).fmt(f)
    }
}
impl Debug for Vec3Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        DebugExpression::new(false, self).fmt(f)
    }
}
impl Debug for Vec4Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        DebugExpression::new(false, self).fmt(f)
    }
}
impl Debug for MultiVectorGroupExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        DebugExpression::new(false, self).fmt(f)
    }
}
impl Debug for MultiVectorExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        DebugExpression::new(false, self).fmt(f)
    }
}



impl<'e> Debug for DebugExpression<'e, AnyExpression> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "/* AnyExpression */ ")?;
        match self.expr {
            AnyExpression::Int(e) => {
                let e = self.also(e);
                write!(f, "{e:?}")?
            },
            AnyExpression::Float(e) => {
                let e = self.also(e);
                write!(f, "{e:?}")?
            },
            AnyExpression::Vec2(e) => {
                let e = self.also(e);
                write!(f, "{e:?}")?
            },
            AnyExpression::Vec3(e) => {
                let e = self.also(e);
                write!(f, "{e:?}")?
            },
            AnyExpression::Vec4(e) => {
                let e = self.also(e);
                write!(f, "{e:?}")?
            },
            AnyExpression::Class(e) => {
                let e = self.also(e);
                write!(f, "{e:?}")?
            },
        }
        Ok(())
    }
}

impl<'e> Debug for DebugExpression<'e, IntExpr> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let IntExpr::Variable(v) = &self.expr {
            let (n, i) = &v.decl.name;
            match (n.as_str(), i) {
                ("self", 0) => write!(f, "slf")?,
                (n, 0) => write!(f, "{n}")?,
                (n, i) => write!(f, "{n}_{}", i + 1)?,
            }
            return write!(f, ".clone().into()");
        }
        write!(f, "IntExpr::")?;
        match self.expr {
            IntExpr::Variable(_) => {}
            IntExpr::Literal(l) => write!(f, "Literal({l})")?,
            IntExpr::TraitInvoke10ToInt(t, m) => write!(f, "TraitInvoke10ToInt({t:?}, {m:?})")?,
        }
        Ok(())
    }
}
impl<'e> Debug for DebugExpression<'e, FloatExpr> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ti = self.this_newline_and_indent();
        let ii = self.inner_newline_and_indent();

        if let FloatExpr::Variable(v) = &self.expr {
            let (n, i) = &v.decl.name;
            match (n.as_str(), i) {
                ("self", 0) => write!(f, "slf")?,
                (n, 0) => write!(f, "{n}")?,
                (n, i) => write!(f, "{n}_{}", i + 1)?,
            }
            return write!(f, ".clone().into()");
        }
        write!(f, "FloatExpr::")?;
        match self.expr {
            FloatExpr::Variable(_) => {}
            FloatExpr::Literal(l) => write!(f, "Literal({l:?})")?,
            FloatExpr::FromInt(i) => {
                let i = self.also(i);
                write!(f, "FromInt({i:?})")?;
            },
            FloatExpr::AccessVec2(box v, i) => {
                let v = self.also(v);
                write!(f, "access_vec_2({:?}, {i})", v)?;
            },
            FloatExpr::AccessVec3(box v, i) => {
                let v = self.also(v);
                write!(f, "access_vec_3({:?}, {i})", v)?;
            },
            FloatExpr::AccessVec4(box v, i) => {
                let v = self.also(v);
                write!(f, "access_vec_4({:?}, {i})", v)?;
            },
            FloatExpr::AccessMultiVecGroup(mve, i) => {
                let mve = self.also(mve);
                write!(f, "AccessMultiVecGroup({mve:?}, {i:?})")?;
            },
            FloatExpr::AccessMultiVecFlat(mve, i) => {
                let mve = self.also(mve);
                write!(f, "AccessMultiVecFlat({mve:?}, {i:?})")?;
            },
            FloatExpr::TraitInvoke11ToFloat(t, m) => {
                let m = self.also(m);
                write!(f, "TraitInvoke11ToFloat({t:?}, {m:?})")?;
            },
            FloatExpr::Product(v, l) => {
                write!(f, "product(vec![")?;
                for (e, exp) in v.iter() {
                    let e = self.also_deeper(e);
                    write!(f, "{ii}({e:?}, {exp:?}),")?;
                }
                write!(f, "{ti}], {l:?})")?;
            }
            FloatExpr::Sum(v, l) => {
                write!(f, "sum(vec![")?;
                for (e, coe) in v.iter() {
                    let e = self.also_deeper(e);
                    write!(f, "{ii}({e:?}, {coe:?}),")?;
                }
                write!(f, "{ti}], {l:?})")?;
            }
            FloatExpr::Exp(box a, b, c) => {
                let a = self.also(a);
                write!(f, "Exp(Box::new({a:?}), ")?;
                match b {
                    None => write!(f, "None")?,
                    Some(box b) => {
                        let b = self.also(b);
                        write!(f, "Some(Box::new({b:?}))")?
                    },
                }
                write!(f, ", {c:?})")?;
            }
        }
        Ok(())
    }
}
impl<'e> Debug for DebugExpression<'e, Vec2Expr> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ti = self.this_newline_and_indent();
        let ii = self.inner_newline_and_indent();

        if let Vec2Expr::Variable(v) = &self.expr {
            let (n, i) = &v.decl.name;
            match (n.as_str(), i) {
                ("self", 0) => write!(f, "slf")?,
                (n, 0) => write!(f, "{n}")?,
                (n, i) => write!(f, "{n}_{}", i + 1)?,
            }
            return write!(f, ".clone().into()");
        }
        write!(f, "Vec2Expr::")?;
        match self.expr {
            Vec2Expr::Variable(_) => {}
            Vec2Expr::Gather1(x) => {
                let x = self.also(x);
                write!(f, "Gather1({x:?})")?;
            },
            Vec2Expr::Gather2(x, y) => {
                let x = self.also_deeper(x);
                let y = self.also_deeper(y);
                write!(f, "Gather2({ii}{x:?},{ii}{y:?}{ti})")?;
            },
            Vec2Expr::AccessMultiVecGroup(mve, i) => {
                let mve = self.also(mve);
                write!(f, "AccessMultiVecGroup({mve:?}, {i:?})")?;
            },
            Vec2Expr::Product(v, l) => {
                write!(f, "product(vec![")?;
                for (e, exp) in v.iter() {
                    let e = self.also_deeper(e);
                    write!(f, "{ii}({e:?}, {exp:?}), ")?;
                }
                write!(f, "{ti}], {l:?})")?;
            }
            Vec2Expr::Sum(v, l) => {
                write!(f, "sum(vec![")?;
                for (e, coe) in v.iter() {
                    let e = self.also_deeper(e);
                    write!(f, "{ii}({e:?}, {coe:?}), ")?;
                }
                write!(f, "{ti}], {l:?})")?;
            }
            Vec2Expr::SwizzleVec2(box v, i0, i1) => {
                let v = self.also(v);
                write!(f, "swizzle_vec_2({v:?}, {i0}, {i1})")?;
            },
            Vec2Expr::SwizzleVec3(box v, i0, i1) => {
                let v = self.also(v);
                write!(f, "swizzle_vec_3({v:?}, {i0}, {i1})")?;
            },
            Vec2Expr::SwizzleVec4(box v, i0, i1) => {
                let v = self.also(v);
                write!(f, "swizzle_vec_4({v:?}, {i0}, {i1})")?;
            },
            Vec2Expr::Truncate3to2(box v) => {
                let v = self.also(v);
                write!(f, "Truncate3to2(Box::new({v:?}))")?;
            },
            Vec2Expr::Truncate4to2(box v) => {
                let v = self.also(v);
                write!(f, "Truncate4to2(Box::new({v:?}))")?;
            },
        }
        Ok(())
    }
}
impl<'e> Debug for DebugExpression<'e, Vec3Expr> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ti = self.this_newline_and_indent();
        let ii = self.inner_newline_and_indent();

        if let Vec3Expr::Variable(v) = &self.expr {
            let (n, i) = &v.decl.name;
            match (n.as_str(), i) {
                ("self", 0) => write!(f, "slf")?,
                (n, 0) => write!(f, "{n}")?,
                (n, i) => write!(f, "{n}_{}", i + 1)?,
            }
            return write!(f, ".clone().into()");
        }
        write!(f, "Vec3Expr::")?;
        match self.expr {
            Vec3Expr::Variable(_) => {}
            Vec3Expr::Gather1(x) => {
                let x = self.also(x);
                write!(f, "Gather1({x:?})")?;
            },
            Vec3Expr::Gather3(x, y, z) => {
                let x = self.also_deeper(x);
                let y = self.also_deeper(y);
                let z = self.also_deeper(z);
                write!(f, "Gather3({ii}{x:?},{ii}{y:?},{ii}{z:?}{ti})")?;
            },
            Vec3Expr::AccessMultiVecGroup(mve, i) => {
                let mve = self.also(mve);
                write!(f, "AccessMultiVecGroup({mve:?}, {i:?})")?;
            },
            Vec3Expr::Product(v, l) => {
                write!(f, "product(vec![")?;
                for (e, exp) in v.iter() {
                    let e = self.also_deeper(e);
                    write!(f, "{ii}({e:?}, {exp:?}), ")?;
                }
                write!(f, "{ti}], {l:?})")?;
            }
            Vec3Expr::Sum(v, l) => {
                write!(f, "sum(vec![")?;
                for (e, coe) in v.iter() {
                    let e = self.also_deeper(e);
                    write!(f, "{ii}({e:?}, {coe:?}), ")?;
                }
                write!(f, "{ti}], {l:?})")?;
            }
            Vec3Expr::SwizzleVec2(v, i0, i1, i2) => {
                let v = self.also(v);
                write!(f, "swizzle_vec_2({v:?}, {i0}, {i1}, {i2})")?;
            },
            Vec3Expr::SwizzleVec3(box v, i0, i1, i2) => {
                let v = self.also(v);
                write!(f, "swizzle_vec_3({v:?}, {i0}, {i1}, {i2})")?;
            },
            Vec3Expr::SwizzleVec4(box v, i0, i1, i2) => {
                let v = self.also(v);
                write!(f, "swizzle_vec_4({v:?}, {i0}, {i1}, {i2})")?;
            },
            Vec3Expr::Truncate4to3(box v) => {
                let v = self.also(v);
                write!(f, "Truncate4to3(Box::new({v:?}))")?;
            },
            Vec3Expr::Extend2to3(v, z) => {
                let v = self.also_deeper(v);
                let z = self.also_deeper(z);
                write!(f, "Extend2to3({ii}{v:?},{ii}{z:?}{ti})")?;
            },
        }
        Ok(())
    }
}
impl<'e> Debug for DebugExpression<'e, Vec4Expr> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ti = self.this_newline_and_indent();
        let ii = self.inner_newline_and_indent();

        if let Vec4Expr::Variable(v) = &self.expr {
            let (n, i) = &v.decl.name;
            match (n.as_str(), i) {
                ("self", 0) => write!(f, "slf")?,
                (n, 0) => write!(f, "{n}")?,
                (n, i) => write!(f, "{n}_{}", i + 1)?,
            }
            return write!(f, ".clone().into()");
        }
        write!(f, "Vec4Expr::")?;
        match self.expr {
            Vec4Expr::Variable(_) => {}
            Vec4Expr::Gather1(x) => {
                let x = self.also(x);
                write!(f, "Gather1({x:?})")?;
            },
            Vec4Expr::Gather4(x, y, z, w) => {
                let x = self.also_deeper(x);
                let y = self.also_deeper(y);
                let z = self.also_deeper(z);
                let w = self.also_deeper(w);
                write!(f, "Gather4({ii}{x:?},{ii}{y:?},{ii}{z:?},{ii}{w:?}{ti})")?;
            },
            Vec4Expr::AccessMultiVecGroup(mve, i) => {
                let mve = self.also(mve);
                write!(f, "AccessMultiVecGroup({mve:?}, {i:?})")?;
            },
            Vec4Expr::Product(v, l) => {
                write!(f, "product(vec![")?;
                for (e, exp) in v.iter() {
                    let e = self.also_deeper(e);
                    write!(f, "{ii}({e:?}, {exp:?}),")?;
                }
                write!(f, "{ti}], {l:?})")?;
            }
            Vec4Expr::Sum(v, l) => {
                write!(f, "sum(vec![")?;
                for (e, coe) in v.iter() {
                    let e = self.also_deeper(e);
                    write!(f, "{ii}({e:?}, {coe:?}),")?;
                }
                write!(f, "{ti}], {l:?})")?;
            }
            Vec4Expr::SwizzleVec2(v, i0, i1, i2, i3) => {
                let v = self.also(v);
                write!(f, "swizzle_vec_2({v:?}, {i0}, {i1}, {i2}, {i3})")?;
            },
            Vec4Expr::SwizzleVec3(v, i0, i1, i2, i3) => {
                let v = self.also(v);
                write!(f, "swizzle_vec_3({v:?}, {i0}, {i1}, {i2}, {i3})")?;
            },
            Vec4Expr::SwizzleVec4(box v, i0, i1, i2, i3) => {
                let v = self.also(v);
                write!(f, "swizzle_vec_4({v:?}, {i0}, {i1}, {i2}, {i3})")?;
            },
            Vec4Expr::Extend2to4(v, z, w) => {
                let v = self.also_deeper(v);
                let z = self.also_deeper(z);
                let w = self.also_deeper(w);
                write!(f, "Extend2to4({ii}{v:?},{ii}{z:?},{ii}{w:?}{ti})")?;
            },
            Vec4Expr::Extend3to4(v, w) => {
                let v = self.also_deeper(v);
                let w = self.also_deeper(w);
                write!(f, "Extend3to4({ii}{v:?},{ii}{w:?}{ti})")?;
            },
        }
        Ok(())
    }
}

impl<'e> Debug for DebugExpression<'e, MultiVectorGroupExpr> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MultiVectorGroupExpr::")?;
        match self.expr {
            MultiVectorGroupExpr::JustFloat(v) => {
                let v = self.also(v);
                write!(f, "JustFloat({v:?})")?
            },
            MultiVectorGroupExpr::Vec2(v) => {
                let v = self.also(v);
                write!(f, "Vec2({v:?})")?
            },
            MultiVectorGroupExpr::Vec3(v) => {
                let v = self.also(v);
                write!(f, "Vec3({v:?})")?
            },
            MultiVectorGroupExpr::Vec4(v) => {
                let v = self.also(v);
                write!(f, "Vec4({v:?})")?
            },
        }
        Ok(())
    }
}


impl<'e> Debug for DebugExpression<'e, MultiVectorExpr> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ti = self.this_newline_and_indent();
        let ii = self.inner_newline_and_indent();

        let n = self.expr.mv_class.name();
        let via = self.expr.expr.as_ref();
        if let MultiVectorVia::Variable(v) = &via {
            let (n, i) = &v.decl.name;
            match (n.as_str(), i) {
                ("self", 0) => write!(f, "slf")?,
                (n, 0) => write!(f, "{n}")?,
                (n, i) => write!(f, "{n}_{}", i + 1)?,
            }
            return write!(f, ".clone().into()");
        }
        write!(f, "MultiVectorExpr::new(&{n}, MultiVectorVia::")?;
        match via {
            MultiVectorVia::Variable(_) => {}
            MultiVectorVia::Construct(v) => {
                write!(f, "Construct(vec![")?;
                let mut gs = self.expr.mv_class.groups().into_iter();
                for expr in v.iter() {
                    let group = gs.next().expect("zipping");
                    use BasisElementGroup::*;
                    use MultiVectorGroupExpr::*;
                    let expr = self.also_deeper(expr);
                    match group {
                        G1(be0) => {
                            write!(f, "{ii}/* {be0} */")?;
                            write!(f, "{ii}{expr:?},")?;
                        }
                        G2(be0, be1) => {
                            write!(f, "{ii}/* {be0}, {be1} */")?;
                            write!(f, "{ii}{expr:?},")?;
                        }
                        G3(be0, be1, be2) => {
                            write!(f, "{ii}/* {be0}, {be1}, {be2} */")?;
                            write!(f, "{ii}{expr:?},")?;
                        }
                        G4(be0, be1, be2, be3) => {
                            write!(f, "{ii}/* {be0}, {be1}, {be2}, {be3} */")?;
                            write!(f, "{ii}{expr:?},")?;
                        }
                    }
                }
                write!(f, "{ti}])")?;
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