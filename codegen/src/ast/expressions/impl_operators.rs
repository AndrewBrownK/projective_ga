
impl<FE: Into<FloatExpr>> Add<FE> for FloatExpr {
    type Output = FloatExpr;

    fn add(self, rhs: FE) -> Self::Output {
        let rhs = rhs.into();
        let mut s = FloatExpr::sum(vec![(self, 1.0), (rhs, 1.0)], 0.0);
        s.simplify();
        s
    }
}
impl<FE: Into<FloatExpr>> AddAssign<FE> for FloatExpr {
    fn add_assign(&mut self, rhs: FE) {
        let rhs = rhs.into();
        let mut x = FloatExpr::Literal(0.0);
        mem::swap(&mut x, self);
        *self = FloatExpr::sum(vec![(x, 1.0), (rhs, 1.0)], 0.0);
        self.simplify();
    }
}
impl<FE: Into<FloatExpr>> Mul<FE> for FloatExpr {
    type Output = FloatExpr;

    fn mul(self, rhs: FE) -> Self::Output {
        let rhs = rhs.into();
        let mut s = FloatExpr::product(vec![(self, 1.0), (rhs, 1.0)], 1.0);
        s.simplify();
        s
    }
}
impl<FE: Into<FloatExpr>> MulAssign<FE> for FloatExpr {
    fn mul_assign(&mut self, rhs: FE) {
        let rhs = rhs.into();
        let mut x = FloatExpr::Literal(1.0);
        mem::swap(&mut x, self);
        *self = FloatExpr::product(vec![(x, 1.0), (rhs, 1.0)], 1.0);
        self.simplify();
    }
}
impl Neg for FloatExpr {
    type Output = FloatExpr;

    fn neg(self) -> Self::Output {
        let mut result = self.mul(FloatExpr::Literal(-1.0));
        result.simplify();
        result
    }
}
impl Sub for FloatExpr {
    type Output = FloatExpr;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(-rhs)
    }
}
impl SubAssign for FloatExpr {
    fn sub_assign(&mut self, rhs: Self) {
        self.add_assign(-rhs);
    }
}

impl<V: Into<Vec2Expr>> Add<V> for Vec2Expr {
    type Output = Vec2Expr;

    fn add(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        let mut s = Vec2Expr::sum(vec![(self, 1.0), (rhs, 1.0)], [0.0; 2]);
        s.simplify();
        s
    }
}
impl<V: Into<Vec2Expr>> AddAssign<V> for Vec2Expr {
    fn add_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let mut x = Vec2Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        *self = Vec2Expr::sum(vec![(x, 1.0), (rhs, 1.0)], [0.0; 2]);
        self.simplify();
    }
}
impl<V: Into<Vec2Expr>> Mul<V> for Vec2Expr {
    type Output = Vec2Expr;

    fn mul(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        let mut s = Vec2Expr::product(vec![(self, 1.0), (rhs, 1.0)], [1.0; 2]);
        s.simplify();
        s
    }
}
impl<V: Into<Vec2Expr>> MulAssign<V> for Vec2Expr {
    fn mul_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let mut x = Vec2Expr::Gather1(FloatExpr::Literal(1.0));
        mem::swap(&mut x, self);
        *self = Vec2Expr::product(vec![(x, 1.0), (rhs, 1.0)], [1.0; 2]);
        self.simplify();
    }
}
impl Neg for Vec2Expr {
    type Output = Vec2Expr;

    fn neg(self) -> Self::Output {
        let mut result = self.mul(Vec2Expr::Gather1(FloatExpr::Literal(-1.0)));
        result.simplify();
        result
    }
}
impl Sub for Vec2Expr {
    type Output = Vec2Expr;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(-rhs)
    }
}
impl SubAssign for Vec2Expr {
    fn sub_assign(&mut self, rhs: Self) {
        self.add_assign(-rhs);
    }
}

impl<V: Into<Vec3Expr>> Add<V> for Vec3Expr {
    type Output = Vec3Expr;

    fn add(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        let mut s = Vec3Expr::sum(vec![(self, 1.0), (rhs, 1.0)], [0.0; 3]);
        s.simplify();
        s
    }
}
impl<V: Into<Vec3Expr>> AddAssign<V> for Vec3Expr {
    fn add_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let mut x = Vec3Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        *self = Vec3Expr::sum(vec![(x, 1.0), (rhs, 1.0)], [0.0; 3]);
        self.simplify();
    }
}
impl<V: Into<Vec3Expr>> Mul<V> for Vec3Expr {
    type Output = Vec3Expr;

    fn mul(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        let mut s = Vec3Expr::product(vec![(self, 1.0), (rhs, 1.0)], [1.0; 3]);
        s.simplify();
        s
    }
}
impl<V: Into<Vec3Expr>> MulAssign<V> for Vec3Expr {
    fn mul_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let mut x = Vec3Expr::Gather1(FloatExpr::Literal(1.0));
        mem::swap(&mut x, self);
        *self = Vec3Expr::product(vec![(x, 1.0), (rhs, 1.0)], [1.0; 3]);
        self.simplify();
    }
}
impl Neg for Vec3Expr {
    type Output = Vec3Expr;

    fn neg(self) -> Self::Output {
        let mut result = self.mul(Vec3Expr::Gather1(FloatExpr::Literal(-1.0)));
        result.simplify();
        result
    }
}
impl Sub for Vec3Expr {
    type Output = Vec3Expr;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(-rhs)
    }
}
impl SubAssign for Vec3Expr {
    fn sub_assign(&mut self, rhs: Self) {
        self.add_assign(-rhs);
    }
}

impl<V: Into<Vec4Expr>> Add<V> for Vec4Expr {
    type Output = Vec4Expr;

    fn add(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        let mut s = Vec4Expr::sum(vec![(self, 1.0), (rhs, 1.0)], [0.0; 4]);
        s.simplify();
        s
    }
}
impl<V: Into<Vec4Expr>> AddAssign<V> for Vec4Expr {
    fn add_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let mut x = Vec4Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        *self = Vec4Expr::sum(vec![(x, 1.0), (rhs, 1.0)], [0.0; 4]);
        self.simplify();
    }
}
impl<V: Into<Vec4Expr>> Mul<V> for Vec4Expr {
    type Output = Vec4Expr;

    fn mul(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        let mut s = Vec4Expr::product(vec![(self, 1.0), (rhs, 1.0)], [1.0; 4]);
        s.simplify();
        s
    }
}
impl<V: Into<Vec4Expr>> MulAssign<V> for Vec4Expr {
    fn mul_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        let mut x = Vec4Expr::Gather1(FloatExpr::Literal(1.0));
        mem::swap(&mut x, self);
        *self = Vec4Expr::product(vec![(x, 1.0), (rhs, 1.0)], [1.0; 4]);
        self.simplify();
    }
}
impl Neg for Vec4Expr {
    type Output = Vec4Expr;

    fn neg(self) -> Self::Output {
        let mut result = self.mul(Vec4Expr::Gather1(FloatExpr::Literal(-1.0)));
        result.simplify();
        result
    }
}
impl Sub for Vec4Expr {
    type Output = Vec4Expr;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(-rhs)
    }
}
impl SubAssign for Vec4Expr {
    fn sub_assign(&mut self, rhs: Self) {
        self.add_assign(-rhs);
    }
}