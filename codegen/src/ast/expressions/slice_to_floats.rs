
// Something like the opposite of transposition
// Because of expressions like this:
/*
// impl AntiConstraintViolation for Flector
Simd32x2::from([-(self[e2] * self[e431]) - (self[e3] * self[e412]), 0.0])
    + (Simd32x2::from([0.0, self[e4] * self[e4] + self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412]]) * Simd32x2::from([0.0, -1.0]))
    + (Simd32x2::from(self[e423]) * Simd32x2::from([self[e1], self[e423]]))
    + (Simd32x2::from(self[e431]) * Simd32x2::from([self[e2], self[e431]]))
    + (Simd32x2::from(self[e412]) * Simd32x2::from([self[e3], self[e412]]))
    - (Simd32x2::from([self[e423], self[e4]]) * self.group0().xw()),
 */

// In other words, the reason we are doing this is in order to unlock stronger term
// cancellation in Sums and Products.

// This is a late stage in the effort to migrate from two-phase simplification to single-phase simplification.
// The reason we previously had two-phase simplification (which worked pretty dang well)
// was so that we could avoid transposition as much as possible, then in "final_simplify"
// perform a transposing simplify, followed by a non-transposing simplify that converts to
// flat multivector access.

// The reason we needed to move away from that system is that multi-line variable inlining
// could no longer simplify (or particularly transpose) expressions that had been converted to
// flat multivector access.

// So we upgraded the simplification methods so that it can convert to flat access and perform
// transposition at the same time. However, we have a different symptom/problem now, which is
// since we get a lot of VecXExpr::Sums instead of FloatExpr::Sums with variables getting inlined,
// a lot of the sum terms don't cancel out optimally anymore. So we have to slice all the
// VecXExpr::Sum and VecXExpr::Product into GatherX of FloatExpr::Sum and FloatExpr::Product, so
// that each lane of the VecX can independently cancel out optimally, and then we can re-extract
// the transposed vectors that remain.

// So why explain all of this if the old code is the old code and no longer exists?

// Well the point is, the intention of simplify() methods is still true. It's supposed to be
// a one and done operation. As much as possible anyway. Well, maybe it is not so possible.
// If you try to save transposition until the end, it simplifies extremely cleanly, but then
// obstructs inlining of destructured variables. We could make it so every VecX::Sum and
// VecX::Product performs slice_to_float and redoes transposition on every simplify, but that is
// ABSURDLY expensive. The destructured variable inlining loop is what demands we have it both ways.
// We want to know if we use a whole VecX, so we can keep it as a variable. But we also
// need inlined destructured terms to cancel appropriately. So the only real solution is to
// slice_to_floats and re-transpose repeatedly in the destructuring loops. The least we can do
// is make the slice_to_floats operation separate form simplification, and thus just do its job
// as fast and efficient as possible, instead of getting roped into an entire expensive simplify
// invocation. So that's the point of this file. These methods will pretty much only get used
// in the destructured variable inlining loop, because that is the only place these symptoms show
// up. Each time such an inlining is performed, we'll slice to floats, and then re-invoke simplify.
// And hopefully everything will finally be maximally simplified then.
// But it sadly goes to show that you can't just one-tap simplify() and be sure the job is 100% done.
// Not to imply spamming simplify() will make it any better - it should never require spamming to
// get as far as it can get. And that is the distinction here. slice_to_floats will be considered
// a separate operation from simplification, even though simplification sometimes needs it
// in order to truly be finished/optimal.


impl AnyExpression {
    pub(crate) fn slice_to_floats(&mut self) {
        match self {
            AnyExpression::Int(_) => {}
            AnyExpression::Float(_) => {}
            AnyExpression::Vec2(e) => e.slice_to_floats(),
            AnyExpression::Vec3(e) => e.slice_to_floats(),
            AnyExpression::Vec4(e) => e.slice_to_floats(),
            AnyExpression::Class(e) => e.slice_to_floats(),
        }
    }
}

impl Vec2Expr {
    pub(crate) fn slice_to_floats(&mut self) {
        match self {
            Vec2Expr::Variable(v) => {
                let x = FloatExpr::access_vec_2(Vec2Expr::Variable(v.clone()), 0);
                let y = FloatExpr::access_vec_2(Vec2Expr::Variable(v.clone()), 1);
                *self = Vec2Expr::Gather2(x, y);
            }
            Vec2Expr::Gather1(f) => {
                let x = f.clone();
                let y = f.take_as_owned();
                *self = Vec2Expr::Gather2(x, y);
            }
            Vec2Expr::Gather2(_, _) => {}
            Vec2Expr::AccessMultiVecGroup(mvg, group_idx) => {
                let x = FloatExpr::access_vec_2(Vec2Expr::AccessMultiVecGroup(mvg.clone(), *group_idx), 0);
                let y = FloatExpr::access_vec_2(Vec2Expr::AccessMultiVecGroup(mvg.clone(), *group_idx), 1);
                *self = Vec2Expr::Gather2(x, y);
            }
            Vec2Expr::Product(v, l) => {
                let mut xs = vec![];
                let mut ys = vec![];
                let mut our_v = vec![];
                mem::swap(v, &mut our_v);
                for (mut v, e) in our_v.into_iter() {
                    xs.push((v.get_slice_of_float(0, true), e));
                    ys.push((v.get_slice_of_float(1, false), e));
                }
                *self = Vec2Expr::Gather2(
                    FloatExpr::Product(xs, l[0]),
                    FloatExpr::Product(ys, l[1]),
                );
            }
            Vec2Expr::Sum(v, l) => {
                let mut xs = vec![];
                let mut ys = vec![];
                let mut our_v = vec![];
                mem::swap(v, &mut our_v);
                for (mut v, c) in our_v.into_iter() {
                    xs.push((v.get_slice_of_float(0, true), c));
                    ys.push((v.get_slice_of_float(1, false), c));
                }
                *self = Vec2Expr::Gather2(
                    FloatExpr::Sum(xs, l[0]),
                    FloatExpr::Sum(ys, l[1]),
                );
            }
            Vec2Expr::SwizzleVec2(box v, ix, iy) => {
                let x = v.get_slice_of_float(*ix, true);
                let y = v.get_slice_of_float(*iy, false);
                *self = Vec2Expr::Gather2(x, y);
            }
            Vec2Expr::SwizzleVec3(box v, ix, iy) => {
                let x = v.get_slice_of_float(*ix, true);
                let y = v.get_slice_of_float(*iy, false);
                *self = Vec2Expr::Gather2(x, y);
            }
            Vec2Expr::SwizzleVec4(box v, ix, iy) => {
                let x = v.get_slice_of_float(*ix, true);
                let y = v.get_slice_of_float(*iy, false);
                *self = Vec2Expr::Gather2(x, y);
            }
            Vec2Expr::Truncate3to2(box v) => {
                let x = v.get_slice_of_float(0, true);
                let y = v.get_slice_of_float(1, false);
                *self = Vec2Expr::Gather2(x, y);
            }
            Vec2Expr::Truncate4to2(box v) => {
                let x = v.get_slice_of_float(0, true);
                let y = v.get_slice_of_float(1, false);
                *self = Vec2Expr::Gather2(x, y);
            }
        }
    }

    fn get_slice_of_float(&mut self, idx: usize, use_clone_not_take: bool) -> FloatExpr {
        assert!(idx < 2);
        match self {
            Vec2Expr::Variable(v) => FloatExpr::access_vec_2(Vec2Expr::Variable(v.clone()), idx),
            Vec2Expr::Gather1(f) => if use_clone_not_take { f.clone() } else { f.take_as_owned() }
            Vec2Expr::Gather2(x, y) => {
                let f = match idx {
                    0 => x, 1 => y, _ => unreachable!("see assert at start of function")
                };
                if use_clone_not_take { f.clone() } else { f.take_as_owned() }
            }
            Vec2Expr::AccessMultiVecGroup(mvg, group_idx) => {
                FloatExpr::access_vec_2(Vec2Expr::AccessMultiVecGroup(mvg.clone(), *group_idx), idx)
            }
            Vec2Expr::Product(v, l) => {
                let mut fs = vec![];
                for (v, e) in v.iter_mut() {
                    fs.push((v.get_slice_of_float(idx, true), *e));
                }
                FloatExpr::Product(fs, l[idx])
            }
            Vec2Expr::Sum(v, l) => {
                let mut fs = vec![];
                for (v, e) in v.iter_mut() {
                    fs.push((v.get_slice_of_float(idx, true), *e));
                }
                FloatExpr::Sum(fs, l[idx])
            }
            Vec2Expr::SwizzleVec2(box v, ix, iy) => {
                let idx = match idx {
                    0 => *ix, 1 => *iy, _ => unreachable!("see assert at start of function")
                };
                v.get_slice_of_float(idx, use_clone_not_take)
            }
            Vec2Expr::SwizzleVec3(box v, ix, iy) => {
                let idx = match idx {
                    0 => *ix, 1 => *iy, _ => unreachable!("see assert at start of function")
                };
                v.get_slice_of_float(idx, use_clone_not_take)
            }
            Vec2Expr::SwizzleVec4(box v, ix, iy) => {
                let idx = match idx {
                    0 => *ix, 1 => *iy, _ => unreachable!("see assert at start of function")
                };
                v.get_slice_of_float(idx, use_clone_not_take)
            }
            Vec2Expr::Truncate3to2(box v) => v.get_slice_of_float(idx, use_clone_not_take),
            Vec2Expr::Truncate4to2(box v) => v.get_slice_of_float(idx, use_clone_not_take),
        }
    }
}
impl Vec3Expr {
    pub(crate) fn slice_to_floats(&mut self) {
        match self {
            Vec3Expr::Variable(v) => {
                let x = FloatExpr::access_vec_3(Vec3Expr::Variable(v.clone()), 0);
                let y = FloatExpr::access_vec_3(Vec3Expr::Variable(v.clone()), 1);
                let z = FloatExpr::access_vec_3(Vec3Expr::Variable(v.clone()), 2);
                *self = Vec3Expr::Gather3(x, y, z);
            }
            Vec3Expr::Gather1(f) => {
                let x = f.clone();
                let y = f.clone();
                let z = f.take_as_owned();
                *self = Vec3Expr::Gather3(x, y, z);
            }
            Vec3Expr::Gather3(_, _, _) => {}
            Vec3Expr::AccessMultiVecGroup(mvg, group_idx) => {
                let x = FloatExpr::access_vec_3(Vec3Expr::AccessMultiVecGroup(mvg.clone(), *group_idx), 0);
                let y = FloatExpr::access_vec_3(Vec3Expr::AccessMultiVecGroup(mvg.clone(), *group_idx), 1);
                let z = FloatExpr::access_vec_3(Vec3Expr::AccessMultiVecGroup(mvg.clone(), *group_idx), 2);
                *self = Vec3Expr::Gather3(x, y, z);
            }
            Vec3Expr::Product(v, l) => {
                let mut xs = vec![];
                let mut ys = vec![];
                let mut zs = vec![];
                let mut our_v = vec![];
                mem::swap(v, &mut our_v);
                for (mut v, e) in our_v.into_iter() {
                    xs.push((v.get_slice_of_float(0, true), e));
                    ys.push((v.get_slice_of_float(1, true), e));
                    zs.push((v.get_slice_of_float(2, false), e));
                }
                *self = Vec3Expr::Gather3(
                    FloatExpr::Product(xs, l[0]),
                    FloatExpr::Product(ys, l[1]),
                    FloatExpr::Product(zs, l[2]),
                );
            }
            Vec3Expr::Sum(v, l) => {
                let mut xs = vec![];
                let mut ys = vec![];
                let mut zs = vec![];
                let mut our_v = vec![];
                mem::swap(v, &mut our_v);
                for (mut v, c) in our_v.into_iter() {
                    xs.push((v.get_slice_of_float(0, true), c));
                    ys.push((v.get_slice_of_float(1, true), c));
                    zs.push((v.get_slice_of_float(2, false), c));
                }
                *self = Vec3Expr::Gather3(
                    FloatExpr::Sum(xs, l[0]),
                    FloatExpr::Sum(ys, l[1]),
                    FloatExpr::Sum(zs, l[2]),
                );
            }
            Vec3Expr::SwizzleVec2(v, ix, iy, iz) => {
                let x = v.get_slice_of_float(*ix, true);
                let y = v.get_slice_of_float(*iy, true);
                let z = v.get_slice_of_float(*iz, false);
                *self = Vec3Expr::Gather3(x, y, z);
            }
            Vec3Expr::SwizzleVec3(box v, ix, iy, iz) => {
                let x = v.get_slice_of_float(*ix, true);
                let y = v.get_slice_of_float(*iy, true);
                let z = v.get_slice_of_float(*iz, false);
                *self = Vec3Expr::Gather3(x, y, z);
            }
            Vec3Expr::SwizzleVec4(box v, ix, iy, iz) => {
                let x = v.get_slice_of_float(*ix, true);
                let y = v.get_slice_of_float(*iy, true);
                let z = v.get_slice_of_float(*iz, false);
                *self = Vec3Expr::Gather3(x, y, z);
            }
            Vec3Expr::Truncate4to3(v) => {
                let x = v.get_slice_of_float(0, true);
                let y = v.get_slice_of_float(1, true);
                let z = v.get_slice_of_float(2, false);
                *self = Vec3Expr::Gather3(x, y, z);
            }
            Vec3Expr::Extend2to3(v, z) => {
                let x = v.get_slice_of_float(0, true);
                let y = v.get_slice_of_float(1, false);
                *self = Vec3Expr::Gather3(x, y, z.take_as_owned());
            }
        }
    }

    fn get_slice_of_float(&mut self, idx: usize, use_clone_not_take: bool) -> FloatExpr {
        assert!(idx < 3);
        match self {
            Vec3Expr::Variable(v) => FloatExpr::access_vec_3(Vec3Expr::Variable(v.clone()), idx),
            Vec3Expr::Gather1(f) => if use_clone_not_take { f.clone() } else { f.take_as_owned() }
            Vec3Expr::Gather3(x, y, z) => {
                let f = match idx {
                    0 => x, 1 => y, 2 => z, _ => unreachable!("see assert at start of function")
                };
                if use_clone_not_take { f.clone() } else { f.take_as_owned() }
            }
            Vec3Expr::AccessMultiVecGroup(mvg, group_idx) => {
                FloatExpr::access_vec_3(Vec3Expr::AccessMultiVecGroup(mvg.clone(), *group_idx), idx)
            }
            Vec3Expr::Product(v, l) => {
                let mut fs = vec![];
                for (v, e) in v.iter_mut() {
                    fs.push((v.get_slice_of_float(idx, true), *e));
                }
                FloatExpr::Product(fs, l[idx])
            }
            Vec3Expr::Sum(v, l) => {
                let mut fs = vec![];
                for (v, e) in v.iter_mut() {
                    fs.push((v.get_slice_of_float(idx, true), *e));
                }
                FloatExpr::Sum(fs, l[idx])
            }
            Vec3Expr::SwizzleVec2(v, ix, iy, iz) => {
                let idx = match idx {
                    0 => *ix, 1 => *iy, 2 => *iz, _ => unreachable!("see assert at start of function")
                };
                v.get_slice_of_float(idx, use_clone_not_take)
            }
            Vec3Expr::SwizzleVec3(box v, ix, iy, iz) => {
                let idx = match idx {
                    0 => *ix, 1 => *iy, 2 => *iz, _ => unreachable!("see assert at start of function")
                };
                v.get_slice_of_float(idx, use_clone_not_take)
            }
            Vec3Expr::SwizzleVec4(box v, ix, iy, iz) => {
                let idx = match idx {
                    0 => *ix, 1 => *iy, 2 => *iz, _ => unreachable!("see assert at start of function")
                };
                v.get_slice_of_float(idx, use_clone_not_take)
            }
            Vec3Expr::Truncate4to3(box v) => v.get_slice_of_float(idx, use_clone_not_take),
            Vec3Expr::Extend2to3(v, z) => {
                if idx == 2 {
                    return if use_clone_not_take { z.clone() } else { z.take_as_owned() }
                }
                v.get_slice_of_float(idx, use_clone_not_take)
            }
        }
    }
}
impl Vec4Expr {
    pub(crate) fn slice_to_floats(&mut self) {
        match self {
            Vec4Expr::Variable(v) => {
                let x = FloatExpr::access_vec_4(Vec4Expr::Variable(v.clone()), 0);
                let y = FloatExpr::access_vec_4(Vec4Expr::Variable(v.clone()), 1);
                let z = FloatExpr::access_vec_4(Vec4Expr::Variable(v.clone()), 2);
                let w = FloatExpr::access_vec_4(Vec4Expr::Variable(v.clone()), 3);
                *self = Vec4Expr::Gather4(x, y, z, w);
            }
            Vec4Expr::Gather1(f) => {
                let x = f.clone();
                let y = f.clone();
                let z = f.clone();
                let w = f.take_as_owned();
                *self = Vec4Expr::Gather4(x, y, z, w);
            }
            Vec4Expr::Gather4(_, _, _, _) => {}
            Vec4Expr::AccessMultiVecGroup(mvg, group_idx) => {
                let x = FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(mvg.clone(), *group_idx), 0);
                let y = FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(mvg.clone(), *group_idx), 1);
                let z = FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(mvg.clone(), *group_idx), 2);
                let w = FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(mvg.clone(), *group_idx), 3);
                *self = Vec4Expr::Gather4(x, y, z, w);
            }
            Vec4Expr::Product(v, l) => {
                let mut xs = vec![];
                let mut ys = vec![];
                let mut zs = vec![];
                let mut ws = vec![];
                let mut our_v = vec![];
                mem::swap(v, &mut our_v);
                for (mut v, e) in our_v.into_iter() {
                    xs.push((v.get_slice_of_float(0, true), e));
                    ys.push((v.get_slice_of_float(1, true), e));
                    zs.push((v.get_slice_of_float(2, true), e));
                    ws.push((v.get_slice_of_float(3, false), e));
                }
                *self = Vec4Expr::Gather4(
                    FloatExpr::Product(xs, l[0]),
                    FloatExpr::Product(ys, l[1]),
                    FloatExpr::Product(zs, l[2]),
                    FloatExpr::Product(ws, l[3]),
                );
            }
            Vec4Expr::Sum(v, l) => {
                let mut xs = vec![];
                let mut ys = vec![];
                let mut zs = vec![];
                let mut ws = vec![];
                let mut our_v = vec![];
                mem::swap(v, &mut our_v);
                for (mut v, c) in our_v.into_iter() {
                    xs.push((v.get_slice_of_float(0, true), c));
                    ys.push((v.get_slice_of_float(1, true), c));
                    zs.push((v.get_slice_of_float(2, true), c));
                    ws.push((v.get_slice_of_float(3, false), c));
                }
                *self = Vec4Expr::Gather4(
                    FloatExpr::Sum(xs, l[0]),
                    FloatExpr::Sum(ys, l[1]),
                    FloatExpr::Sum(zs, l[2]),
                    FloatExpr::Sum(ws, l[3]),
                );
            }
            Vec4Expr::SwizzleVec2(v, ix, iy, iz, iw) => {
                let x = v.get_slice_of_float(*ix, true);
                let y = v.get_slice_of_float(*iy, true);
                let z = v.get_slice_of_float(*iz, true);
                let w = v.get_slice_of_float(*iw, false);
                *self = Vec4Expr::Gather4(x, y, z, w);
            }
            Vec4Expr::SwizzleVec3(v, ix, iy, iz, iw) => {
                let x = v.get_slice_of_float(*ix, true);
                let y = v.get_slice_of_float(*iy, true);
                let z = v.get_slice_of_float(*iz, true);
                let w = v.get_slice_of_float(*iw, false);
                *self = Vec4Expr::Gather4(x, y, z, w);
            }
            Vec4Expr::SwizzleVec4(box v, ix, iy, iz, iw) => {
                let x = v.get_slice_of_float(*ix, true);
                let y = v.get_slice_of_float(*iy, true);
                let z = v.get_slice_of_float(*iz, true);
                let w = v.get_slice_of_float(*iw, false);
                *self = Vec4Expr::Gather4(x, y, z, w);
            }
            Vec4Expr::Extend2to4(v, z, w) => {
                let x = v.get_slice_of_float(0, true);
                let y = v.get_slice_of_float(1, false);
                *self = Vec4Expr::Gather4(x, y, z.take_as_owned(), w.take_as_owned());
            }
            Vec4Expr::Extend3to4(v, w) => {
                let x = v.get_slice_of_float(0, true);
                let y = v.get_slice_of_float(1, true);
                let z = v.get_slice_of_float(2, false);
                *self = Vec4Expr::Gather4(x, y, z, w.take_as_owned());
            }
        }
    }

    fn get_slice_of_float(&mut self, idx: usize, use_clone_not_take: bool) -> FloatExpr {
        assert!(idx < 4);
        match self {
            Vec4Expr::Variable(v) => FloatExpr::access_vec_4(Vec4Expr::Variable(v.clone()), idx),
            Vec4Expr::Gather1(f) => if use_clone_not_take { f.clone() } else { f.take_as_owned() }
            Vec4Expr::Gather4(x, y, z, w) => {
                let f = match idx {
                    0 => x, 1 => y, 2 => z, 3 => w, _ => unreachable!("see assert at start of function")
                };
                if use_clone_not_take { f.clone() } else { f.take_as_owned() }
            }
            Vec4Expr::AccessMultiVecGroup(mvg, group_idx) => {
                FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(mvg.clone(), *group_idx), idx)
            }
            Vec4Expr::Product(v, l) => {
                let mut fs = vec![];
                for (v, e) in v.iter_mut() {
                    fs.push((v.get_slice_of_float(idx, true), *e));
                }
                FloatExpr::Product(fs, l[idx])
            }
            Vec4Expr::Sum(v, l) => {
                let mut fs = vec![];
                for (v, e) in v.iter_mut() {
                    fs.push((v.get_slice_of_float(idx, true), *e));
                }
                FloatExpr::Sum(fs, l[idx])
            }
            Vec4Expr::SwizzleVec2(v, ix, iy, iz, iw) => {
                let idx = match idx {
                    0 => *ix, 1 => *iy, 2 => *iz, 3 => *iw, _ => unreachable!("see assert at start of function")
                };
                v.get_slice_of_float(idx, use_clone_not_take)
            }
            Vec4Expr::SwizzleVec3(v, ix, iy, iz, iw) => {
                let idx = match idx {
                    0 => *ix, 1 => *iy, 2 => *iz, 3 => *iw, _ => unreachable!("see assert at start of function")
                };
                v.get_slice_of_float(idx, use_clone_not_take)
            }
            Vec4Expr::SwizzleVec4(box v, ix, iy, iz, iw) => {
                let idx = match idx {
                    0 => *ix, 1 => *iy, 2 => *iz, 3 => *iw, _ => unreachable!("see assert at start of function")
                };
                v.get_slice_of_float(idx, use_clone_not_take)
            }
            Vec4Expr::Extend2to4(v, z, w) => {
                if idx == 3 {
                    return if use_clone_not_take { w.clone() } else { w.take_as_owned() }
                }
                if idx == 2 {
                    return if use_clone_not_take { z.clone() } else { z.take_as_owned() }
                }
                v.get_slice_of_float(idx, use_clone_not_take)
            }
            Vec4Expr::Extend3to4(v, w) => {
                if idx == 3 {
                    return if use_clone_not_take { w.clone() } else { w.take_as_owned() }
                }
                v.get_slice_of_float(idx, use_clone_not_take)
            }
        }
    }
}
impl MultiVectorExpr {
    pub(crate) fn slice_to_floats(&mut self) {
        match &mut *self.expr {
            MultiVectorVia::Variable(_) => {}
            MultiVectorVia::Construct(gs) => {
                for g in gs.iter_mut() {
                    match g {
                        MultiVectorGroupExpr::JustFloat(_) => {}
                        MultiVectorGroupExpr::Vec2(v) => v.slice_to_floats(),
                        MultiVectorGroupExpr::Vec3(v) => v.slice_to_floats(),
                        MultiVectorGroupExpr::Vec4(v) => v.slice_to_floats(),
                    }
                }
            }
            MultiVectorVia::TraitInvoke11ToClass(_, _) => {}
            MultiVectorVia::TraitInvoke12iToClass(_, _, _) => {}
            MultiVectorVia::TraitInvoke12fToClass(_, _, _) => {}
            MultiVectorVia::TraitInvoke21ToClass(_, _, _) => {}
            MultiVectorVia::TraitInvoke22ToClass(_, _, _) => {}
        }
    }
}