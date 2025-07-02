

impl FloatExpr {
    pub fn access_vec_2(v: Vec2Expr, i: usize) -> Self {
        assert!(i < 2);
        FloatExpr::AccessVec2(Box::new(v), i)
    }
    pub fn access_vec_3(v: Vec3Expr, i: usize) -> Self {
        assert!(i < 3);
        FloatExpr::AccessVec3(Box::new(v), i)
    }
    pub fn access_vec_4(v: Vec4Expr, i: usize) -> Self {
        assert!(i < 4);
        FloatExpr::AccessVec4(Box::new(v), i)
    }
    pub fn product(mut factors: Vec<(FloatExpr, f32)>, literal: f32) -> Self {
        assert!(!factors.is_empty());
        factors.sort_with_f32();
        FloatExpr::Product(factors, literal)
    }
    pub fn sum(mut addends: Vec<(FloatExpr, f32)>, literal: f32) -> Self {
        assert!(!addends.is_empty());
        addends.sort_with_f32();
        FloatExpr::Sum(addends, literal)
    }
}

impl Vec2Expr {
    pub fn product(mut factors: Vec<(Vec2Expr, f32)>, literal: [f32; 2]) -> Self {
        assert!(!factors.is_empty());
        factors.sort_with_f32();
        Vec2Expr::Product(factors, literal)
    }
    pub fn sum(mut addends: Vec<(Vec2Expr, f32)>, literal: [f32; 2]) -> Self {
        assert!(!addends.is_empty());
        addends.sort_with_f32();
        Vec2Expr::Sum(addends, literal)
    }
    pub fn swizzle_vec_2(v: Vec2Expr, x: usize, y: usize) -> Self {
        assert!(x < 2);
        assert!(y < 2);
        if x == 0 && y == 1 {
            v
        } else if x == y {
            Vec2Expr::Gather1(FloatExpr::AccessVec2(Box::new(v), x))
        } else {
            Vec2Expr::SwizzleVec2(Box::new(v), x, y)
        }
    }
    pub fn swizzle_vec_3(v: Vec3Expr, x: usize, y: usize) -> Self {
        assert!(x < 3);
        assert!(y < 3);
        match v {
            Vec3Expr::Extend2to3(v2, _) if x == 0 && y == 1 => v2,
            v if x == y => Vec2Expr::Gather1(FloatExpr::AccessVec3(Box::new(v), x)),
            v => Vec2Expr::SwizzleVec3(Box::new(v), x, y),
        }
    }
    pub fn swizzle_vec_4(v: Vec4Expr, x: usize, y: usize) -> Self {
        assert!(x < 4);
        assert!(y < 4);
        match v {
            Vec4Expr::Extend2to4(v2, _, _) if x == 0 && y == 1 => v2,
            v if x == y => Vec2Expr::Gather1(FloatExpr::AccessVec4(Box::new(v), x)),
            v => Vec2Expr::SwizzleVec4(Box::new(v), x, y),
        }
    }
}

impl Vec3Expr {
    pub fn product(mut factors: Vec<(Vec3Expr, f32)>, literal: [f32; 3]) -> Self {
        assert!(!factors.is_empty());
        factors.sort_with_f32();
        Vec3Expr::Product(factors, literal)
    }
    pub fn sum(mut addends: Vec<(Vec3Expr, f32)>, literal: [f32; 3]) -> Self {
        assert!(!addends.is_empty());
        addends.sort_with_f32();
        Vec3Expr::Sum(addends, literal)
    }
    pub fn swizzle_vec_2(v: Vec2Expr, x: usize, y: usize, z: usize) -> Self {
        assert!(x < 2);
        assert!(y < 2);
        assert!(z < 2);
        if x == y && y == z {
            Vec3Expr::Gather1(FloatExpr::AccessVec2(Box::new(v), x))
        } else {
            Vec3Expr::SwizzleVec2(v, x, y, z)
        }
    }
    pub fn swizzle_vec_3(v: Vec3Expr, x: usize, y: usize, z: usize) -> Self {
        assert!(x < 3);
        assert!(y < 3);
        assert!(z < 3);
        if x == 0 && y == 1 && z == 2 {
            v
        } else if x == y && y == z {
            Vec3Expr::Gather1(FloatExpr::AccessVec3(Box::new(v), x))
        } else {
            Vec3Expr::SwizzleVec3(Box::new(v), x, y, z)
        }
    }
    pub fn swizzle_vec_4(v: Vec4Expr, x: usize, y: usize, z: usize) -> Self {
        assert!(x < 4);
        assert!(y < 4);
        assert!(z < 4);
        match v {
            Vec4Expr::Extend3to4(v3, _) if x == 0 && y == 1 && z == 2 => v3,
            v if x == y && y == z => Vec3Expr::Gather1(FloatExpr::AccessVec4(Box::new(v), x)),
            v => Vec3Expr::SwizzleVec4(Box::new(v), x, y, z),
        }
    }
}

impl Vec4Expr {
    pub fn product(mut factors: Vec<(Vec4Expr, f32)>, literal: [f32; 4]) -> Self {
        assert!(!factors.is_empty());
        factors.sort_with_f32();
        Vec4Expr::Product(factors, literal)
    }
    pub fn sum(mut addends: Vec<(Vec4Expr, f32)>, literal: [f32; 4]) -> Self {
        assert!(!addends.is_empty());
        addends.sort_with_f32();
        Vec4Expr::Sum(addends, literal)
    }
    pub fn swizzle_vec_2(v: Vec2Expr, x: usize, y: usize, z: usize, w: usize) -> Self {
        assert!(x < 2);
        assert!(y < 2);
        assert!(z < 2);
        assert!(w < 2);
        if x == y && y == z && z == w {
            Vec4Expr::Gather1(FloatExpr::AccessVec2(Box::new(v), x))
        } else {
            Vec4Expr::SwizzleVec2(v, x, y, z, w)
        }
    }
    pub fn swizzle_vec_3(v: Vec3Expr, x: usize, y: usize, z: usize, w: usize) -> Self {
        assert!(x < 3);
        assert!(y < 3);
        assert!(z < 3);
        assert!(w < 3);
        if x == y && y == z && z == w {
            Vec4Expr::Gather1(FloatExpr::AccessVec3(Box::new(v), x))
        } else {
            Vec4Expr::SwizzleVec3(v, x, y, z, w)
        }
    }
    pub fn swizzle_vec_4(v: Vec4Expr, x: usize, y: usize, z: usize, w: usize) -> Self {
        assert!(x < 4);
        assert!(y < 4);
        assert!(z < 4);
        assert!(w < 4);
        if x == 0 && y == 1 && z == 2 && w == 3 {
            v
        } else if x == y && y == z && z == w {
            Vec4Expr::Gather1(FloatExpr::AccessVec4(Box::new(v), x))
        } else {
            Vec4Expr::SwizzleVec4(Box::new(v), x, y, z, w)
        }
    }
}