

impl FloatExpr {
    pub fn access_vec_2(v: Vec2Expr, i: usize) -> Self {
        assert!(i < 2);
        FloatExpr::AccessVec2(Box::new(v), i as u8)
    }
    pub fn access_vec_3(v: Vec3Expr, i: usize) -> Self {
        assert!(i < 3);
        FloatExpr::AccessVec3(Box::new(v), i as u8)
    }
    pub fn access_vec_4(v: Vec4Expr, i: usize) -> Self {
        assert!(i < 4);
        FloatExpr::AccessVec4(Box::new(v), i as u8)
    }
    pub fn product(factors: Vec<(FloatExpr, f32)>, literal: f32) -> Self {
        assert!(!factors.is_empty());
        FloatExpr::Product(factors, literal)
    }
    pub fn sum(addends: Vec<(FloatExpr, f32)>, literal: f32) -> Self {
        assert!(!addends.is_empty());
        FloatExpr::Sum(addends, literal)
    }
}

impl Vec2Expr {
    pub fn product(factors: Vec<(Vec2Expr, f32)>, literal: [f32; 2]) -> Self {
        assert!(!factors.is_empty());
        Vec2Expr::Product(factors, literal)
    }
    pub fn sum(addends: Vec<(Vec2Expr, f32)>, literal: [f32; 2]) -> Self {
        assert!(!addends.is_empty());
        Vec2Expr::Sum(addends, literal)
    }
    pub fn swizzle_vec_2(v: Vec2Expr, x: usize, y: usize) -> Self {
        assert!(x < 2);
        assert!(y < 2);
        Vec2Expr::SwizzleVec2(Box::new(v), x as u8, y as u8)
    }
}

impl Vec3Expr {
    pub fn product(factors: Vec<(Vec3Expr, f32)>, literal: [f32; 3]) -> Self {
        assert!(!factors.is_empty());
        Vec3Expr::Product(factors, literal)
    }
    pub fn sum(addends: Vec<(Vec3Expr, f32)>, literal: [f32; 3]) -> Self {
        assert!(!addends.is_empty());
        Vec3Expr::Sum(addends, literal)
    }
    pub fn swizzle_vec_3(v: Vec3Expr, x: usize, y: usize, z: usize) -> Self {
        assert!(x < 3);
        assert!(y < 3);
        assert!(z < 3);
        Vec3Expr::SwizzleVec3(Box::new(v), x as u8, y as u8, z as u8)
    }
}

impl Vec4Expr {
    pub fn product(factors: Vec<(Vec4Expr, f32)>, literal: [f32; 4]) -> Self {
        assert!(!factors.is_empty());
        Vec4Expr::Product(factors, literal)
    }
    pub fn sum(addends: Vec<(Vec4Expr, f32)>, literal: [f32; 4]) -> Self {
        assert!(!addends.is_empty());
        Vec4Expr::Sum(addends, literal)
    }
    pub fn swizzle_vec_4(v: Vec4Expr, x: usize, y: usize, z: usize, w: usize) -> Self {
        assert!(x < 4);
        assert!(y < 4);
        assert!(z < 4);
        assert!(w < 4);
        Vec4Expr::SwizzleVec4(Box::new(v), x as u8, y as u8, z as u8, w as u8)
    }
}