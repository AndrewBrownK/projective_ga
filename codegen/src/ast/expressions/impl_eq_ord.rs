

impl PartialEq for FloatExpr {
    fn eq(&self, other: &Self) -> bool {
        use FloatExpr::*;
        match (self, other) {
            (Variable(a), Variable(b)) => a == b,
            (Literal(a), Literal(b)) => FloatOrd(*a) == FloatOrd(*b),
            (FromInt(a), FromInt(b)) => a == b,
            (AccessVec2(a, ai), AccessVec2(b, bi)) => a == b && ai == bi,
            (AccessVec3(a, ai), AccessVec3(b, bi)) => a == b && ai == bi,
            (AccessVec4(a, ai), AccessVec4(b, bi)) => a == b && ai == bi,
            (AccessMultiVecGroup(a, ai), AccessMultiVecGroup(b, bi)) => a == b && ai == bi,
            (AccessMultiVecFlat(a, ai), AccessMultiVecFlat(b, bi)) => a == b && ai == bi,
            (TraitInvoke11ToFloat(ak, a), TraitInvoke11ToFloat(bk, b)) => ak == bk && a == b,
            (Product(a, al), Product(b, bl)) => {
                if FloatOrd(*al) != FloatOrd(*bl) {
                    return false
                }
                if a.len() != b.len() {
                    return false
                }
                for ((af, ae), (bf, be)) in a.iter().zip(b.iter()) {
                    if FloatOrd(*ae) != FloatOrd(*be) {
                        return false
                    }
                    if af != bf {
                        return false
                    }
                }
                true
            }
            (Sum(a, al), Sum(b, bl)) => {
                if FloatOrd(*al) != FloatOrd(*bl) {
                    return false
                }
                if a.len() != b.len() {
                    return false
                }
                for ((aa, af), (ba, bf)) in a.iter().zip(b.iter()) {
                    if FloatOrd(*af) != FloatOrd(*bf) {
                        return false
                    }
                    if aa != ba {
                        return false
                    }
                }
                true
            }
            (Exp(a, ae, al), Exp(b, be, bl)) => a == b && ae == be && al == bl,

            // Fully list the other branches so that if we add to variants, we get a
            // compiler error if we forget to update this method.
            (Variable(..), _) => false,
            (_, Variable(..)) => false,
            (Literal(..), _) => false,
            (_, Literal(..)) => false,
            (FromInt(..), _) => false,
            (_, FromInt(..)) => false,
            (AccessVec2(..), _) => false,
            (_, AccessVec2(..)) => false,
            (AccessVec3(..), _) => false,
            (_, AccessVec3(..)) => false,
            (AccessVec4(..), _) => false,
            (_, AccessVec4(..)) => false,
            (AccessMultiVecGroup(..), _) => false,
            (_, AccessMultiVecGroup(..)) => false,
            (AccessMultiVecFlat(..), _) => false,
            (_, AccessMultiVecFlat(..)) => false,
            (TraitInvoke11ToFloat(..), _) => false,
            (_, TraitInvoke11ToFloat(..)) => false,
            (Product(..), _) => false,
            (_, Product(..)) => false,
            (Sum(..), _) => false,
            (_, Sum(..)) => false,
            #[allow(unreachable_patterns)]
            (Exp(..), _) => false,
            #[allow(unreachable_patterns)]
            (_, Exp(..)) => false,
        }
    }
}
impl Eq for FloatExpr {}
impl PartialOrd for FloatExpr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}
impl Ord for FloatExpr {
    fn cmp(&self, other: &Self) -> Ordering {
        use FloatExpr::*;
        match (self, other) {
            (Variable(a), Variable(b)) => a.cmp(b),
            (Literal(a), Literal(b)) => FloatOrd(*a).cmp(&FloatOrd(*b)),
            (FromInt(a), FromInt(b)) => a.cmp(&b),
            (AccessVec2(a, ai), AccessVec2(b, bi)) => a.cmp(b).then_with(|| ai.cmp(bi)),
            (AccessVec3(a, ai), AccessVec3(b, bi)) => a.cmp(b).then_with(|| ai.cmp(bi)),
            (AccessVec4(a, ai), AccessVec4(b, bi)) => a.cmp(b).then_with(|| ai.cmp(bi)),
            (AccessMultiVecGroup(a, ai), AccessMultiVecGroup(b, bi)) => a.cmp(b).then_with(|| ai.cmp(bi)),
            (AccessMultiVecFlat(a, ai), AccessMultiVecFlat(b, bi)) => a.cmp(b).then_with(|| ai.cmp(bi)),
            (TraitInvoke11ToFloat(ak, a), TraitInvoke11ToFloat(bk, b)) => ak.cmp(bk).then_with(|| a.cmp(b)),
            (Product(a, al), Product(b, bl)) => {
                let c =  FloatOrd(*al).cmp(&FloatOrd(*bl));
                if c != Ordering::Equal { return c }
                let c = a.len().cmp(&b.len());
                if c != Ordering::Equal { return c }
                for ((af, ae), (bf, be)) in a.iter().zip(b.iter()) {
                    let c = FloatOrd(*ae).cmp(&FloatOrd(*be));
                    if c != Ordering::Equal { return c };
                    let c = af.cmp(bf);
                    if c != Ordering::Equal { return c }
                }
                Ordering::Equal
            }
            (Sum(a, al), Sum(b, bl)) => {
                let c =  FloatOrd(*al).cmp(&FloatOrd(*bl));
                if c != Ordering::Equal { return c }
                let c = a.len().cmp(&b.len());
                if c != Ordering::Equal { return c }
                for ((aa, af), (ba, bf)) in a.iter().zip(b.iter()) {
                    let c = FloatOrd(*af).cmp(&FloatOrd(*bf));
                    if c != Ordering::Equal { return c };
                    let c = aa.cmp(ba);
                    if c != Ordering::Equal { return c }
                }
                Ordering::Equal
            }
            (Exp(a_factor, a_exp, a_lexp), Exp(b_factor, b_exp, b_lexp)) => {
                let c = a_factor.cmp(&b_factor);
                if c != Ordering::Equal { return c }
                let c = a_exp.cmp(&b_exp);
                if c != Ordering::Equal { return c }
                FloatOrd(*a_lexp).cmp(&FloatOrd(*b_lexp))
            }
            (Variable(_), _) => Ordering::Less,
            (_, Variable(_)) => Ordering::Greater,
            (Literal(_), _) => Ordering::Less,
            (_, Literal(_)) => Ordering::Greater,
            (FromInt(_), _) => Ordering::Less,
            (_, FromInt(_)) => Ordering::Greater,
            (AccessVec2(_, _), _) => Ordering::Less,
            (_, AccessVec2(_, _)) => Ordering::Greater,
            (AccessVec3(_, _), _) => Ordering::Less,
            (_, AccessVec3(_, _)) => Ordering::Greater,
            (AccessVec4(_, _), _) => Ordering::Less,
            (_, AccessVec4(_, _)) => Ordering::Greater,
            (AccessMultiVecGroup(_, _), _) => Ordering::Less,
            (_, AccessMultiVecGroup(_, _)) => Ordering::Greater,
            (AccessMultiVecFlat(_, _), _) => Ordering::Less,
            (_, AccessMultiVecFlat(_, _)) => Ordering::Greater,
            (TraitInvoke11ToFloat(_, _), _) => Ordering::Less,
            (_, TraitInvoke11ToFloat(_, _)) => Ordering::Greater,
            (Product(_, _), _) => Ordering::Less,
            (_, Product(_, _)) => Ordering::Greater,
            (Sum(_, _), _) => Ordering::Less,
            (_, Sum(_, _)) => Ordering::Greater,
            #[allow(unreachable_patterns)]
            (Exp(_, _, _), _) => Ordering::Less,
            #[allow(unreachable_patterns)]
            (_, Exp(_, _, _)) => Ordering::Greater,
        }
    }
}

impl PartialEq for Vec2Expr {
    fn eq(&self, other: &Self) -> bool {
        use Vec2Expr::*;
        match (self, other) {
            (Variable(a), Variable(b)) => a.eq(&b),
            (Gather1(a), Gather1(b)) => a.eq(&b),
            (Gather2(a0, a1), Gather2(b0, b1)) => a0 == b0 && a1 == b1,
            (AccessMultiVecGroup(amv, ai), AccessMultiVecGroup(bmv, bi)) => amv == bmv && ai == bi,
            (Product(a, al), Product(b, bl)) => {
                if FloatOrd(al[0]) != FloatOrd(bl[0]) {
                    return false
                }
                if FloatOrd(al[1]) != FloatOrd(bl[1]) {
                    return false
                }
                if a.len() != b.len() {
                    return false
                }
                for ((af, ae), (bf, be)) in a.iter().zip(b.iter()) {
                    if FloatOrd(*ae) != FloatOrd(*be) {
                        return false
                    }
                    if af != bf {
                        return false
                    }
                }
                true
            }
            (Sum(a, al), Sum(b, bl)) => {
                if FloatOrd(al[0]) != FloatOrd(bl[0]) {
                    return false
                }
                if FloatOrd(al[1]) != FloatOrd(bl[1]) {
                    return false
                }
                if a.len() != b.len() {
                    return false
                }
                for ((aa, af), (ba, bf)) in a.iter().zip(b.iter()) {
                    if FloatOrd(*af) != FloatOrd(*bf) {
                        return false
                    }
                    if aa != ba {
                        return false
                    }
                }
                true
            }
            (SwizzleVec2(av, a0, a1), SwizzleVec2(bv, b0, b1)) => av == bv && a0 == b0 && a1 == b1,
            (SwizzleVec3(av, a0, a1), SwizzleVec3(bv, b0, b1)) => av == bv && a0 == b0 && a1 == b1,
            (SwizzleVec4(av, a0, a1), SwizzleVec4(bv, b0, b1)) => av == bv && a0 == b0 && a1 == b1,
            (Truncate3to2(a), Truncate3to2(b)) => a == b,
            (Truncate4to2(a), Truncate4to2(b)) => a == b,

            // Fully list the other branches so that if we add to variants, we get a
            // compiler error if we forget to update this method.
            (Variable(..), _) => false,
            (_, Variable(..)) => false,
            (Gather1(..), _) => false,
            (_, Gather1(..)) => false,
            (Gather2(..), _) => false,
            (_, Gather2(..)) => false,
            (AccessMultiVecGroup(..), _) => false,
            (_, AccessMultiVecGroup(..)) => false,
            (Product(..), _) => false,
            (_, Product(..)) => false,
            (Sum(..), _) => false,
            (_, Sum(..)) => false,
            (SwizzleVec2(..), _) => false,
            (_, SwizzleVec2(..)) => false,
            (SwizzleVec3(..), _) => false,
            (_, SwizzleVec3(..)) => false,
            (SwizzleVec4(..), _) => false,
            (_, SwizzleVec4(..)) => false,
            (Truncate3to2(..), _) => false,
            (_, Truncate3to2(..)) => false,
            #[allow(unreachable_patterns)]
            (Truncate4to2(..), _) => false,
            #[allow(unreachable_patterns)]
            (_, Truncate4to2(..)) => false,
        }
    }
}
impl Eq for Vec2Expr {}
impl PartialOrd for Vec2Expr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}
impl Ord for Vec2Expr {
    fn cmp(&self, other: &Self) -> Ordering {
        use Vec2Expr::*;
        match (self, other) {
            (Variable(a), Variable(b)) => a.cmp(&b),
            (Gather1(a), Gather1(b)) => a.cmp(&b),
            (Gather2(a0, a1), Gather2(b0, b1)) => {
                let c = a0.cmp(b0);
                if c != Ordering::Equal { return c }
                a1.cmp(b1)
            },
            (AccessMultiVecGroup(amv, ai), AccessMultiVecGroup(bmv, bi)) => {
                let c = amv.cmp(bmv);
                if c != Ordering::Equal { return c }
                ai.cmp(bi)
            }
            (Product(a, al), Product(b, bl)) => {
                let c =  FloatOrd(al[0]).cmp(&FloatOrd(bl[0]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[1]).cmp(&FloatOrd(bl[1]));
                if c != Ordering::Equal { return c }
                let c = a.len().cmp(&b.len());
                if c != Ordering::Equal { return c }
                for ((af, ae), (bf, be)) in a.iter().zip(b.iter()) {
                    let c = FloatOrd(*ae).cmp(&FloatOrd(*be));
                    if c != Ordering::Equal { return c };
                    let c = af.cmp(bf);
                    if c != Ordering::Equal { return c }
                }
                Ordering::Equal
            }
            (Sum(a, al), Sum(b, bl)) => {
                let c =  FloatOrd(al[0]).cmp(&FloatOrd(bl[0]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[1]).cmp(&FloatOrd(bl[1]));
                if c != Ordering::Equal { return c }
                let c = a.len().cmp(&b.len());
                if c != Ordering::Equal { return c }
                for ((aa, af), (ba, bf)) in a.iter().zip(b.iter()) {
                    let c = FloatOrd(*af).cmp(&FloatOrd(*bf));
                    if c != Ordering::Equal { return c };
                    let c = aa.cmp(ba);
                    if c != Ordering::Equal { return c }
                }
                Ordering::Equal
            },
            (SwizzleVec2(av, a0, a1), SwizzleVec2(bv, b0, b1)) => {
                let c = av.cmp(bv);
                if c != Ordering::Equal { return c }
                let c = a0.cmp(b0);
                if c != Ordering::Equal { return c }
                a1.cmp(b1)
            },
            (SwizzleVec3(av, a0, a1), SwizzleVec3(bv, b0, b1)) => {
                let c = av.cmp(bv);
                if c != Ordering::Equal { return c }
                let c = a0.cmp(b0);
                if c != Ordering::Equal { return c }
                a1.cmp(b1)
            },
            (SwizzleVec4(av, a0, a1), SwizzleVec4(bv, b0, b1)) => {
                let c = av.cmp(bv);
                if c != Ordering::Equal { return c }
                let c = a0.cmp(b0);
                if c != Ordering::Equal { return c }
                a1.cmp(b1)
            },
            (Truncate3to2(box a), Truncate3to2(box b)) => {
                a.cmp(b)
            }
            (Truncate4to2(box a), Truncate4to2(box b)) => {
                a.cmp(b)
            }
            (Variable(_), _) => Ordering::Less,
            (_, Variable(_)) => Ordering::Greater,
            (Gather1(_), _) => Ordering::Less,
            (_, Gather1(_)) => Ordering::Greater,
            (Gather2(_, _), _) => Ordering::Less,
            (_, Gather2(_, _)) => Ordering::Greater,
            (AccessMultiVecGroup(_, _), _) => Ordering::Less,
            (_, AccessMultiVecGroup(_, _)) => Ordering::Greater,
            (Product(_, _), _) => Ordering::Less,
            (_, Product(_, _)) => Ordering::Greater,
            (Sum(_, _), _) => Ordering::Less,
            (_, Sum(_, _)) => Ordering::Greater,
            (SwizzleVec2(_, _, _), _) => Ordering::Less,
            (_, SwizzleVec2(_, _, _)) => Ordering::Greater,
            (SwizzleVec3(_, _, _), _) => Ordering::Less,
            (_, SwizzleVec3(_, _, _)) => Ordering::Greater,
            (SwizzleVec4(_, _, _), _) => Ordering::Less,
            (_, SwizzleVec4(_, _, _)) => Ordering::Greater,
            (Truncate3to2(_), _) => Ordering::Less,
            (_, Truncate3to2(_)) => Ordering::Greater,
            #[allow(unreachable_patterns)]
            (Truncate4to2(_), _) => Ordering::Less,
            #[allow(unreachable_patterns)]
            (_, Truncate4to2(_)) => Ordering::Greater,
        }
    }
}

impl PartialEq for Vec3Expr {
    fn eq(&self, other: &Self) -> bool {
        use Vec3Expr::*;
        match (self, other) {
            (Variable(a), Variable(b)) => a.eq(&b),
            (Gather1(a), Gather1(b)) => a.eq(&b),
            (Gather3(a0, a1, a2), Gather3(b0, b1, b2)) => a0 == b0 && a1 == b1 && a2 == b2,
            (AccessMultiVecGroup(amv, ai), AccessMultiVecGroup(bmv, bi)) => amv == bmv && ai == bi,
            (Product(a, al), Product(b, bl)) => {
                if FloatOrd(al[0]) != FloatOrd(bl[0]) {
                    return false
                }
                if FloatOrd(al[1]) != FloatOrd(bl[1]) {
                    return false
                }
                if FloatOrd(al[2]) != FloatOrd(bl[2]) {
                    return false
                }
                if a.len() != b.len() {
                    return false
                }
                for ((af, ae), (bf, be)) in a.iter().zip(b.iter()) {
                    if FloatOrd(*ae) != FloatOrd(*be) {
                        return false
                    }
                    if af != bf {
                        return false
                    }
                }
                true
            }
            (Sum(a, al), Sum(b, bl)) => {
                if FloatOrd(al[0]) != FloatOrd(bl[0]) {
                    return false
                }
                if FloatOrd(al[1]) != FloatOrd(bl[1]) {
                    return false
                }
                if FloatOrd(al[2]) != FloatOrd(bl[2]) {
                    return false
                }
                if a.len() != b.len() {
                    return false
                }
                for ((aa, af), (ba, bf)) in a.iter().zip(b.iter()) {
                    if FloatOrd(*af) != FloatOrd(*bf) {
                        return false
                    }
                    if aa != ba {
                        return false
                    }
                }
                true
            }
            (SwizzleVec2(av, a0, a1, a2), SwizzleVec2(bv, b0, b1, b2)) => av == bv && a0 == b0 && a1 == b1 && a2 == b2,
            (SwizzleVec3(av, a0, a1, a2), SwizzleVec3(bv, b0, b1, b2)) => av == bv && a0 == b0 && a1 == b1 && a2 == b2,
            (SwizzleVec4(av, a0, a1, a2), SwizzleVec4(bv, b0, b1, b2)) => av == bv && a0 == b0 && a1 == b1 && a2 == b2,
            (Truncate4to3(a), Truncate4to3(b)) => a == b,
            (Extend2to3(a, az), Extend2to3(b, bz)) => a == b && az == bz,

            // Fully list the other branches so that if we add to variants, we get a
            // compiler error if we forget to update this method.
            (Variable(..), _) => false,
            (_, Variable(..)) => false,
            (Gather1(..), _) => false,
            (_, Gather1(..)) => false,
            (Gather3(..), _) => false,
            (_, Gather3(..)) => false,
            (AccessMultiVecGroup(..), _) => false,
            (_, AccessMultiVecGroup(..)) => false,
            (Product(..), _) => false,
            (_, Product(..)) => false,
            (Sum(..), _) => false,
            (_, Sum(..)) => false,
            (SwizzleVec2(..), _) => false,
            (_, SwizzleVec2(..)) => false,
            (SwizzleVec3(..), _) => false,
            (_, SwizzleVec3(..)) => false,
            (SwizzleVec4(..), _) => false,
            (_, SwizzleVec4(..)) => false,
            (Truncate4to3(..), _) => false,
            (_, Truncate4to3(..)) => false,
            #[allow(unreachable_patterns)]
            (Extend2to3(..), _) => false,
            #[allow(unreachable_patterns)]
            (_, Extend2to3(..)) => false,
        }
    }
}

impl Eq for Vec3Expr {}
impl PartialOrd for Vec3Expr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}
impl Ord for Vec3Expr {
    fn cmp(&self, other: &Self) -> Ordering {
        use Vec3Expr::*;
        match (self, other) {
            (Variable(a), Variable(b)) => a.cmp(&b),
            (Gather1(a), Gather1(b)) => a.cmp(&b),
            (Gather3(a0, a1, a2), Gather3(b0, b1, b2)) => {
                let c = a0.cmp(b0);
                if c != Ordering::Equal { return c }
                let c = a1.cmp(b1);
                if c != Ordering::Equal { return c }
                a2.cmp(b2)
            },
            (AccessMultiVecGroup(amv, ai), AccessMultiVecGroup(bmv, bi)) => {
                let c = amv.cmp(bmv);
                if c != Ordering::Equal { return c }
                ai.cmp(bi)
            }
            (Product(a, al), Product(b, bl)) => {
                let c =  FloatOrd(al[0]).cmp(&FloatOrd(bl[0]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[1]).cmp(&FloatOrd(bl[1]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[2]).cmp(&FloatOrd(bl[2]));
                if c != Ordering::Equal { return c }
                let c = a.len().cmp(&b.len());
                if c != Ordering::Equal { return c }
                for ((af, ae), (bf, be)) in a.iter().zip(b.iter()) {
                    let c = FloatOrd(*ae).cmp(&FloatOrd(*be));
                    if c != Ordering::Equal { return c };
                    let c = af.cmp(bf);
                    if c != Ordering::Equal { return c }
                }
                Ordering::Equal
            }
            (Sum(a, al), Sum(b, bl)) => {
                let c =  FloatOrd(al[0]).cmp(&FloatOrd(bl[0]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[1]).cmp(&FloatOrd(bl[1]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[2]).cmp(&FloatOrd(bl[2]));
                if c != Ordering::Equal { return c }
                let c = a.len().cmp(&b.len());
                if c != Ordering::Equal { return c }
                for ((aa, af), (ba, bf)) in a.iter().zip(b.iter()) {
                    let c = FloatOrd(*af).cmp(&FloatOrd(*bf));
                    if c != Ordering::Equal { return c };
                    let c = aa.cmp(ba);
                    if c != Ordering::Equal { return c }
                }
                Ordering::Equal
            },
            (SwizzleVec2(av, a0, a1, a2), SwizzleVec2(bv, b0, b1, b2)) => {
                let c = av.cmp(bv);
                if c != Ordering::Equal { return c }
                let c = a0.cmp(b0);
                if c != Ordering::Equal { return c }
                let c = a1.cmp(b1);
                if c != Ordering::Equal { return c }
                a2.cmp(b2)
            },
            (SwizzleVec3(av, a0, a1, a2), SwizzleVec3(bv, b0, b1, b2)) => {
                let c = av.cmp(bv);
                if c != Ordering::Equal { return c }
                let c = a0.cmp(b0);
                if c != Ordering::Equal { return c }
                let c = a1.cmp(b1);
                if c != Ordering::Equal { return c }
                a2.cmp(b2)
            },
            (SwizzleVec4(av, a0, a1, a2), SwizzleVec4(bv, b0, b1, b2)) => {
                let c = av.cmp(bv);
                if c != Ordering::Equal { return c }
                let c = a0.cmp(b0);
                if c != Ordering::Equal { return c }
                let c = a1.cmp(b1);
                if c != Ordering::Equal { return c }
                a2.cmp(b2)
            },
            (Truncate4to3(box a), Truncate4to3(box b)) => {
                a.cmp(b)
            }
            (Extend2to3(a_v2, a_z), Extend2to3(b_v2, b_z)) => {
                let c = a_v2.cmp(b_v2);
                if c != Ordering::Equal { return c }
                a_z.cmp(b_z)
            }
            (Variable(_), _) => Ordering::Less,
            (_, Variable(_)) => Ordering::Greater,
            (Gather1(_), _) => Ordering::Less,
            (_, Gather1(_)) => Ordering::Greater,
            (Gather3(_, _, _), _) => Ordering::Less,
            (_, Gather3(_, _, _)) => Ordering::Greater,
            (AccessMultiVecGroup(_, _), _) => Ordering::Less,
            (_, AccessMultiVecGroup(_, _)) => Ordering::Greater,
            (Product(_, _), _) => Ordering::Less,
            (_, Product(_, _)) => Ordering::Greater,
            (Sum(_, _), _) => Ordering::Less,
            (_, Sum(_, _)) => Ordering::Greater,
            (SwizzleVec2(_, _, _, _), _) => Ordering::Less,
            (_, SwizzleVec2(_, _, _, _)) => Ordering::Greater,
            (SwizzleVec3(_, _, _, _), _) => Ordering::Less,
            (_, SwizzleVec3(_, _, _, _)) => Ordering::Greater,
            (SwizzleVec4(_, _, _, _), _) => Ordering::Less,
            (_, SwizzleVec4(_, _, _, _)) => Ordering::Greater,
            (Truncate4to3(_), _) => Ordering::Less,
            (_, Truncate4to3(_)) => Ordering::Greater,
            #[allow(unreachable_patterns)]
            (Extend2to3(_, _), _) => Ordering::Less,
            #[allow(unreachable_patterns)]
            (_, Extend2to3(_, _)) => Ordering::Greater,
        }
    }
}

impl PartialEq for Vec4Expr {
    fn eq(&self, other: &Self) -> bool {
        use Vec4Expr::*;
        match (self, other) {
            (Variable(a), Variable(b)) => a.eq(&b),
            (Gather1(a), Gather1(b)) => a.eq(&b),
            (Gather4(a0, a1, a2, a3), Gather4(b0, b1, b2, b3)) => a0 == b0 && a1 == b1 && a2 == b2 && a3 == b3,
            (AccessMultiVecGroup(amv, ai), AccessMultiVecGroup(bmv, bi)) => amv == bmv && ai == bi,
            (Product(a, al), Product(b, bl)) => {
                if FloatOrd(al[0]) != FloatOrd(bl[0]) {
                    return false
                }
                if FloatOrd(al[1]) != FloatOrd(bl[1]) {
                    return false
                }
                if FloatOrd(al[2]) != FloatOrd(bl[2]) {
                    return false
                }
                if FloatOrd(al[3]) != FloatOrd(bl[3]) {
                    return false
                }
                if a.len() != b.len() {
                    return false
                }
                for ((af, ae), (bf, be)) in a.iter().zip(b.iter()) {
                    if FloatOrd(*ae) != FloatOrd(*be) {
                        return false
                    }
                    if af != bf {
                        return false
                    }
                }
                true
            }
            (Sum(a, al), Sum(b, bl)) => {
                if FloatOrd(al[0]) != FloatOrd(bl[0]) {
                    return false
                }
                if FloatOrd(al[1]) != FloatOrd(bl[1]) {
                    return false
                }
                if FloatOrd(al[2]) != FloatOrd(bl[2]) {
                    return false
                }
                if FloatOrd(al[3]) != FloatOrd(bl[3]) {
                    return false
                }
                if a.len() != b.len() {
                    return false
                }
                for ((aa, af), (ba, bf)) in a.iter().zip(b.iter()) {
                    if FloatOrd(*af) != FloatOrd(*bf) {
                        return false
                    }
                    if aa != ba {
                        return false
                    }
                }
                true
            },
            (SwizzleVec2(av, a0, a1, a2, a3), SwizzleVec2(bv, b0, b1, b2, b3)) => av == bv && a0 == b0 && a1 == b1 && a2 == b2 && a3 == b3,
            (SwizzleVec3(av, a0, a1, a2, a3), SwizzleVec3(bv, b0, b1, b2, b3)) => av == bv && a0 == b0 && a1 == b1 && a2 == b2 && a3 == b3,
            (SwizzleVec4(av, a0, a1, a2, a3), SwizzleVec4(bv, b0, b1, b2, b3)) => av == bv && a0 == b0 && a1 == b1 && a2 == b2 && a3 == b3,
            (Extend2to4(a, az, aw), Extend2to4(b, bz, bw)) => a == b && az == bz && aw == bw,
            (Extend3to4(a, aw), Extend3to4(b, bw)) => a == b && aw == bw,

            // Fully list the other branches so that if we add to variants, we get a
            // compiler error if we forget to update this method.
            (Variable(..), _) => false,
            (_, Variable(..)) => false,
            (Gather1(..), _) => false,
            (_, Gather1(..)) => false,
            (Gather4(..), _) => false,
            (_, Gather4(..)) => false,
            (AccessMultiVecGroup(..), _) => false,
            (_, AccessMultiVecGroup(..)) => false,
            (Product(..), _) => false,
            (_, Product(..)) => false,
            (Sum(..), _) => false,
            (_, Sum(..)) => false,
            (SwizzleVec2(..), _) => false,
            (_, SwizzleVec2(..)) => false,
            (SwizzleVec3(..), _) => false,
            (_, SwizzleVec3(..)) => false,
            (SwizzleVec4(..), _) => false,
            (_, SwizzleVec4(..)) => false,
            (Extend2to4(..), _) => false,
            (_, Extend2to4(..)) => false,
            #[allow(unreachable_patterns)]
            (Extend3to4(..), _) => false,
            #[allow(unreachable_patterns)]
            (_, Extend3to4(..)) => false,
        }
    }
}
impl Eq for Vec4Expr {}
impl PartialOrd for Vec4Expr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}
impl Ord for Vec4Expr {
    fn cmp(&self, other: &Self) -> Ordering {
        use Vec4Expr::*;
        match (self, other) {
            (Variable(a), Variable(b)) => a.cmp(&b),
            (Gather1(a), Gather1(b)) => a.cmp(&b),
            (Gather4(a0, a1, a2, a3), Gather4(b0, b1, b2, b3)) => {
                let c = a0.cmp(b0);
                if c != Ordering::Equal { return c }
                let c = a1.cmp(b1);
                if c != Ordering::Equal { return c }
                let c = a2.cmp(b2);
                if c != Ordering::Equal { return c }
                a3.cmp(b3)
            },
            (AccessMultiVecGroup(amv, ai), AccessMultiVecGroup(bmv, bi)) => {
                let c = amv.cmp(bmv);
                if c != Ordering::Equal { return c }
                ai.cmp(bi)
            }
            (Product(a, al), Product(b, bl)) => {
                let c =  FloatOrd(al[0]).cmp(&FloatOrd(bl[0]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[1]).cmp(&FloatOrd(bl[1]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[2]).cmp(&FloatOrd(bl[2]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[3]).cmp(&FloatOrd(bl[3]));
                if c != Ordering::Equal { return c }
                let c = a.len().cmp(&b.len());
                if c != Ordering::Equal { return c }
                for ((af, ae), (bf, be)) in a.iter().zip(b.iter()) {
                    let c = FloatOrd(*ae).cmp(&FloatOrd(*be));
                    if c != Ordering::Equal { return c };
                    let c = af.cmp(bf);
                    if c != Ordering::Equal { return c }
                }
                Ordering::Equal
            }
            (Sum(a, al), Sum(b, bl)) => {
                let c =  FloatOrd(al[0]).cmp(&FloatOrd(bl[0]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[1]).cmp(&FloatOrd(bl[1]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[2]).cmp(&FloatOrd(bl[2]));
                if c != Ordering::Equal { return c }
                let c =  FloatOrd(al[3]).cmp(&FloatOrd(bl[3]));
                if c != Ordering::Equal { return c }
                let c = a.len().cmp(&b.len());
                if c != Ordering::Equal { return c }
                for ((aa, af), (ba, bf)) in a.iter().zip(b.iter()) {
                    let c = FloatOrd(*af).cmp(&FloatOrd(*bf));
                    if c != Ordering::Equal { return c };
                    let c = aa.cmp(ba);
                    if c != Ordering::Equal { return c }
                }
                Ordering::Equal
            },
            (SwizzleVec2(av, a0, a1, a2, a3), SwizzleVec2(bv, b0, b1, b2, b3)) => {
                let c = av.cmp(bv);
                if c != Ordering::Equal { return c }
                let c = a0.cmp(b0);
                if c != Ordering::Equal { return c }
                let c = a1.cmp(b1);
                if c != Ordering::Equal { return c }
                let c = a2.cmp(b2);
                if c != Ordering::Equal { return c }
                a3.cmp(b3)
            },
            (SwizzleVec3(av, a0, a1, a2, a3), SwizzleVec3(bv, b0, b1, b2, b3)) => {
                let c = av.cmp(bv);
                if c != Ordering::Equal { return c }
                let c = a0.cmp(b0);
                if c != Ordering::Equal { return c }
                let c = a1.cmp(b1);
                if c != Ordering::Equal { return c }
                let c = a2.cmp(b2);
                if c != Ordering::Equal { return c }
                a3.cmp(b3)
            },
            (SwizzleVec4(av, a0, a1, a2, a3), SwizzleVec4(bv, b0, b1, b2, b3)) => {
                let c = av.cmp(bv);
                if c != Ordering::Equal { return c }
                let c = a0.cmp(b0);
                if c != Ordering::Equal { return c }
                let c = a1.cmp(b1);
                if c != Ordering::Equal { return c }
                let c = a2.cmp(b2);
                if c != Ordering::Equal { return c }
                a3.cmp(b3)
            },
            (Extend2to4(a_v2, a_z, a_w), Extend2to4(b_v2, b_z, b_w)) => {
                let c = a_v2.cmp(b_v2);
                if c != Ordering::Equal { return c }
                let c = a_z.cmp(b_z);
                if c != Ordering::Equal { return c }
                a_w.cmp(b_w)
            }
            (Extend3to4(a_v2, a_w), Extend3to4(b_v2, b_w)) => {
                let c = a_v2.cmp(b_v2);
                if c != Ordering::Equal { return c }
                a_w.cmp(b_w)
            }
            (Variable(_), _) => Ordering::Less,
            (_, Variable(_)) => Ordering::Greater,
            (Gather1(_), _) => Ordering::Less,
            (_, Gather1(_)) => Ordering::Greater,
            (Gather4(_, _, _, _), _) => Ordering::Less,
            (_, Gather4(_, _, _, _)) => Ordering::Greater,
            (AccessMultiVecGroup(_, _), _) => Ordering::Less,
            (_, AccessMultiVecGroup(_, _)) => Ordering::Greater,
            (Product(_, _), _) => Ordering::Less,
            (_, Product(_, _)) => Ordering::Greater,
            (Sum(_, _), _) => Ordering::Less,
            (_, Sum(_, _)) => Ordering::Greater,
            (SwizzleVec2(_, _, _, _, _), _) => Ordering::Less,
            (_, SwizzleVec2(_, _, _, _, _)) => Ordering::Greater,
            (SwizzleVec3(_, _, _, _, _), _) => Ordering::Less,
            (_, SwizzleVec3(_, _, _, _, _)) => Ordering::Greater,
            (SwizzleVec4(_, _, _, _, _), _) => Ordering::Less,
            (_, SwizzleVec4(_, _, _, _, _)) => Ordering::Greater,
            (Extend2to4(_, _, _), _) => Ordering::Less,
            (_, Extend2to4(_, _, _)) => Ordering::Greater,
            #[allow(unreachable_patterns)]
            (Extend3to4(_, _), _) => Ordering::Less,
            #[allow(unreachable_patterns)]
            (_, Extend3to4(_, _)) => Ordering::Greater,
        }
    }
}