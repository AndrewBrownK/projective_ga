#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
pub use std::arch::aarch64::*;
#[cfg(all(target_arch = "arm", target_feature = "neon"))]
pub use std::arch::arm::*;
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
pub use std::arch::wasm32::*;
#[cfg(all(target_arch = "x86", target_feature = "sse2"))]
pub use std::arch::x86::*;
#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
pub use std::arch::x86_64::*;
use std::num::NonZeroU64;
use encase::internal::{BufferMut, BufferRef, Reader, Writer};
use encase::private::Metadata;
use nearly::{EpsToleranceType, NearlyEqEps, NearlyEqTol, NearlyEqUlps, UlpsToleranceType};

#[derive(Clone, Copy)]
#[repr(C)]
pub union Simd32x4 {
    // Intel / AMD
    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
    pub f128: __m128,
    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
    pub i128: __m128i,

    // ARM
    #[cfg(all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "neon"))]
    pub f128: float32x4_t,
    #[cfg(all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "neon"))]
    pub i128: int32x4_t,
    #[cfg(all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "neon"))]
    pub u128: uint32x4_t,

    // Web
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    pub v128: v128,

    // Fallback
    pub f32x4: [f32; 4],
    pub i32x4: [i32; 4],
    pub u32x4: [u32; 4],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union Simd32x3 {
    pub v32x4: Simd32x4,

    // Fallback
    pub f32x3: [f32; 3],
    pub i32x3: [i32; 3],
    pub u32x3: [u32; 3],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union Simd32x2 {
    pub v32x4: Simd32x4,
    pub v32x3: Simd32x3,

    // Fallback
    pub f32x2: [f32; 2],
    pub i32x2: [i32; 2],
    pub u32x2: [u32; 2],
}

#[macro_export]
macro_rules! match_architecture {
    ($Simd:ident, $native:tt, $fallback:tt,) => {{
        #[cfg(any(
            all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"),
            all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "neon"),
            all(target_arch = "wasm32", target_feature = "simd128"),
        ))]
        { $Simd $native }
        #[cfg(not(any(
            all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"),
            all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "neon"),
            all(target_arch = "wasm32", target_feature = "simd128"),
        )))]
        unsafe { $Simd $fallback }
    }};
    ($Simd:ident, $x86:tt, $arm:tt, $web:tt, $fallback:tt,) => {{
        #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
        unsafe { $Simd $x86 }
        #[cfg(all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "neon"))]
        unsafe { $Simd $arm }
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        unsafe { $Simd $web }
        #[cfg(not(any(
            all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"),
            all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "neon"),
            all(target_arch = "wasm32", target_feature = "simd128"),
        )))]
        unsafe { $Simd $fallback }
    }};
}

#[macro_export]
macro_rules! swizzle {
    ($self:expr, $x:literal, $y:literal, _, _ $(,)?) => {
        Simd32x2 { v32x4: $crate::swizzle!($self, $x, $y, 2, 3) }
    };
    ($self:expr, $x:literal, $y:literal, $z:literal, _ $(,)?) => {
        Simd32x3 { v32x4: $crate::swizzle!($self, $x, $y, $z, 3) }
    };
    ($self:expr, $x:literal, $y:literal, $z:literal, $w:literal $(,)?) => {
        $crate::match_architecture!(
            Simd32x4,
            // x86
            { f128: $crate::simd::_mm_permute_ps($self.f128, ($x as i32) | (($y as i32) << 2) | (($z as i32) << 4) | (($w as i32) << 6)) },
            // arm
            { f32x4: [
                $self.f32x4[$x],
                $self.f32x4[$y],
                $self.f32x4[$z],
                $self.f32x4[$w],
            ] },
            // web
            { v128: $crate::simd::i32x4_shuffle::<$x, $y, $z, $w>($self.v128, $self.v128) },
            // fallback
            { f32x4: [
                $self.f32x4[$x],
                $self.f32x4[$y],
                $self.f32x4[$z],
                $self.f32x4[$w],
            ] },
        )
    };
    ($self:expr, $x:literal, $y:literal, _ $(,)?) => {
        Simd32x2 { v32x4: $crate::swizzle!($self.v32x4, $x, $y, 2, 3) }
    };
    ($self:expr, $x:literal, $y:literal, $z:literal $(,)?) => {
        $crate::match_architecture!(
            Simd32x3,
            // native
            { v32x4: $crate::swizzle!($self.v32x4, $x, $y, $z, 3) },
            // fallback
            { f32x3: [
                $self.f32x3[$x],
                $self.f32x3[$y],
                $self.f32x3[$z],
            ] },
        )
    };
    ($self:expr, $x:literal, $y:literal $(,)?) => {
        $crate::match_architecture!(
            Simd32x2,
            // native
            { v32x4: $crate::swizzle!($self.v32x4, $x, $y, 2, 3) },
            // fallback
            { f32x2: [
                $self.f32x2[$x],
                $self.f32x2[$y],
            ] },
        )
    };
}

macro_rules! nicer_swizzle {
    ($out:ident $fn_name:ident $($indexes:literal)+) => {
        #[inline]
        pub fn $fn_name(self) -> $out {
            crate::swizzle!(self, $($indexes,)+)
        }
    };
    ($out:ident $fn_name:ident $($indexes:literal)+ _) => {
        #[inline]
        pub fn $fn_name(self) -> $out {
            crate::swizzle!(self, $($indexes,)+ _)
        }
    };
    ($out:ident $fn_name:ident $($indexes:literal)+ _ _) => {
        #[inline]
        pub fn $fn_name(self) -> $out {
            crate::swizzle!(self, $($indexes,)+ _, _)
        }
    };
}
macro_rules! nicer_swizzles {
    ($out:ident; $($fn_name:ident $($indexes:literal)+);+ $(;)?) => {
        $(
        nicer_swizzle! { $out $fn_name $($indexes)+ }
        )+
    };
    ($out:ident; $($fn_name:ident $($indexes:literal)+ _);+ $(;)?) => {
        $(
        nicer_swizzle! { $out $fn_name $($indexes)+ _ }
        )+
    };
    ($out:ident; $($fn_name:ident $($indexes:literal)+ _ _);+ $(;)?) => {
        $(
        nicer_swizzle! { $out $fn_name $($indexes)+ _ _ }
        )+
    };
}



impl Simd32x2 {
    pub fn powi(mut self, exponent: i32) -> Self {
        self[0] = f32::powi(self[0], exponent);
        self[1] = f32::powi(self[1], exponent);
        self
    }
    pub fn powf(mut self, exponent: f32) -> Self {
        self[0] = f32::powf(self[0], exponent);
        self[1] = f32::powf(self[1], exponent);
        self
    }


    pub fn x(self) -> f32 { self[0] }
    pub fn y(self) -> f32 { self[1] }
    #[inline]
    pub fn xy(self) -> Self { self }
    pub fn with_x(mut self, x: f32) -> Self {
        self[0] = x;
        self
    }
    pub fn with_y(mut self, y: f32) -> Self {
        self[1] = y;
        self
    }
    pub fn with_z(self, z: f32) -> Simd32x3 {
        let mut s = unsafe { self.v32x3 };
        s[2] = z;
        s
    }
    pub fn with_zw(self, z: f32, w: f32) -> Simd32x4 {
        let mut s = unsafe { self.v32x4 };
        s[2] = z;
        s[3] = w;
        return s;
    }

    nicer_swizzles! { Simd32x2;
        xx 0 0;
        // xy 0 1;
        yx 1 0;
        yy 1 1;
    }
}

impl Simd32x3 {
    pub fn powi(mut self, exponent: i32) -> Self {
        self[0] = f32::powi(self[0], exponent);
        self[1] = f32::powi(self[1], exponent);
        self[2] = f32::powi(self[2], exponent);
        self
    }
    pub fn powf(mut self, exponent: f32) -> Self {
        self[0] = f32::powf(self[0], exponent);
        self[1] = f32::powf(self[1], exponent);
        self[2] = f32::powf(self[2], exponent);
        self
    }


    pub fn x(self) -> f32 { self[0] }
    pub fn y(self) -> f32 { self[1] }
    pub fn z(self) -> f32 { self[2] }
    pub fn xy(self) -> Simd32x2 {
        unsafe { Simd32x2 { v32x4: self.v32x4 } }
    }
    #[inline]
    pub fn xyz(self) -> Self { self }
    pub fn with_x(mut self, x: f32) -> Self {
        self[0] = x;
        self
    }
    pub fn with_y(mut self, y: f32) -> Self {
        self[1] = y;
        self
    }
    pub fn with_z(mut self, z: f32) -> Self {
        self[2] = z;
        self
    }
    pub fn with_w(self, w: f32) -> Simd32x4 {
        let mut s = unsafe { self.v32x4 };
        s[3] = w;
        return s;
    }

    nicer_swizzles! { Simd32x2;
        xx 0 0 _;
        // xy 0 1 _;
        xz 0 2 _;
        yx 1 0 _;
        yy 1 1 _;
        yz 1 2 _;
        zx 2 0 _;
        zy 2 1 _;
        zz 2 2 _;
    }
    nicer_swizzles! { Simd32x3;
        xxx 0 0 0;
        xxy 0 0 1;
        xxz 0 0 2;
        xyx 0 1 0;
        xyy 0 1 1;
        // xyz 0 1 2;
        xzx 0 2 0;
        xzy 0 2 1;
        xzz 0 2 2;

        yxx 1 0 0;
        yxy 1 0 1;
        yxz 1 0 2;
        yyx 1 1 0;
        yyy 1 1 1;
        yyz 1 1 2;
        yzx 1 2 0;
        yzy 1 2 1;
        yzz 1 2 2;

        zxx 2 0 0;
        zxy 2 0 1;
        zxz 2 0 2;
        zyx 2 1 0;
        zyy 2 1 1;
        zyz 2 1 2;
        zzx 2 2 0;
        zzy 2 2 1;
        zzz 2 2 2;
    }
}

impl Simd32x4 {

    pub fn powi(mut self, exponent: i32) -> Self {
        self[0] = f32::powi(self[0], exponent);
        self[1] = f32::powi(self[1], exponent);
        self[2] = f32::powi(self[2], exponent);
        self[3] = f32::powi(self[3], exponent);
        self
    }
    pub fn powf(mut self, exponent: f32) -> Self {
        self[0] = f32::powf(self[0], exponent);
        self[1] = f32::powf(self[1], exponent);
        self[2] = f32::powf(self[2], exponent);
        self[3] = f32::powf(self[3], exponent);
        self
    }


    pub fn x(self) -> f32 { self[0] }
    pub fn y(self) -> f32 { self[1] }
    pub fn z(self) -> f32 { self[2] }
    pub fn w(self) -> f32 { self[3] }
    pub fn xy(self) -> Simd32x2 {
        Simd32x2 { v32x4: self }
    }
    pub fn xyz(self) -> Simd32x3 {
        Simd32x3 { v32x4: self }
    }
    #[inline]
    pub fn xyzw(self) -> Self { self }

    pub fn with_x(mut self, x: f32) -> Self {
        self[0] = x;
        self
    }
    pub fn with_y(mut self, y: f32) -> Self {
        self[1] = y;
        self
    }
    pub fn with_z(mut self, z: f32) -> Self {
        self[2] = z;
        self
    }
    pub fn with_w(mut self, w: f32) -> Simd32x4 {
        self[3] = w;
        self
    }

    nicer_swizzles! { Simd32x2;
        xx 0 0 _ _;
        // xy 0 1 _ _;
        xz 0 2 _ _;
        xw 0 3 _ _;
        yx 1 0 _ _;
        yy 1 1 _ _;
        yz 1 2 _ _;
        yw 1 3 _ _;
        zx 2 0 _ _;
        zy 2 1 _ _;
        zz 2 2 _ _;
        zw 2 3 _ _;
        wx 3 0 _ _;
        wy 3 1 _ _;
        wz 3 2 _ _;
        ww 3 3 _ _;
    }
    nicer_swizzles! { Simd32x3;
        xxx 0 0 0 _;
        xxy 0 0 1 _;
        xxz 0 0 2 _;
        xxw 0 0 3 _;
        xyx 0 1 0 _;
        xyy 0 1 1 _;
        // xyz 0 1 2 _;
        xyw 0 1 3 _;
        xzx 0 2 0 _;
        xzy 0 2 1 _;
        xzz 0 2 2 _;
        xzw 0 2 3 _;
        xwx 0 3 0 _;
        xwy 0 3 1 _;
        xwz 0 3 2 _;
        xww 0 3 3 _;

        yxx 1 0 0 _;
        yxy 1 0 1 _;
        yxz 1 0 2 _;
        yxw 1 0 3 _;
        yyx 1 1 0 _;
        yyy 1 1 1 _;
        yyz 1 1 2 _;
        yyw 1 1 3 _;
        yzx 1 2 0 _;
        yzy 1 2 1 _;
        yzz 1 2 2 _;
        yzw 1 2 3 _;
        ywx 1 3 0 _;
        ywy 1 3 1 _;
        ywz 1 3 2 _;
        yww 1 3 3 _;

        zxx 2 0 0 _;
        zxy 2 0 1 _;
        zxz 2 0 2 _;
        zxw 2 0 3 _;
        zyx 2 1 0 _;
        zyy 2 1 1 _;
        zyz 2 1 2 _;
        zyw 2 1 3 _;
        zzx 2 2 0 _;
        zzy 2 2 1 _;
        zzz 2 2 2 _;
        zzw 2 2 3 _;
        zwx 2 3 0 _;
        zwy 2 3 1 _;
        zwz 2 3 2 _;
        zww 2 3 3 _;

        wxx 3 0 0 _;
        wxy 3 0 1 _;
        wxz 3 0 2 _;
        wxw 3 0 3 _;
        wyx 3 1 0 _;
        wyy 3 1 1 _;
        wyz 3 1 2 _;
        wyw 3 1 3 _;
        wzx 3 2 0 _;
        wzy 3 2 1 _;
        wzz 3 2 2 _;
        wzw 3 2 3 _;
        wwx 3 3 0 _;
        wwy 3 3 1 _;
        wwz 3 3 2 _;
        www 3 3 3 _;
    }
    nicer_swizzles! { Simd32x4;
        xxxx 0 0 0 0;
        xxxy 0 0 0 1;
        xxxz 0 0 0 2;
        xxxw 0 0 0 3;
        xxyx 0 0 1 0;
        xxyy 0 0 1 1;
        xxyz 0 0 1 2;
        xxyw 0 0 1 3;
        xxzx 0 0 2 0;
        xxzy 0 0 2 1;
        xxzz 0 0 2 2;
        xxzw 0 0 2 3;
        xxwx 0 0 3 0;
        xxwy 0 0 3 1;
        xxwz 0 0 3 2;
        xxww 0 0 3 3;
        xyxx 0 1 0 0;
        xyxy 0 1 0 1;
        xyxz 0 1 0 2;
        xyxw 0 1 0 3;
        xyyx 0 1 1 0;
        xyyy 0 1 1 1;
        xyyz 0 1 1 2;
        xyyw 0 1 1 3;
        xyzx 0 1 2 0;
        xyzy 0 1 2 1;
        xyzz 0 1 2 2;
        // xyzw 0 1 2 3;
        xywx 0 1 3 0;
        xywy 0 1 3 1;
        xywz 0 1 3 2;
        xyww 0 1 3 3;
        xzxx 0 2 0 0;
        xzxy 0 2 0 1;
        xzxz 0 2 0 2;
        xzxw 0 2 0 3;
        xzyx 0 2 1 0;
        xzyy 0 2 1 1;
        xzyz 0 2 1 2;
        xzyw 0 2 1 3;
        xzzx 0 2 2 0;
        xzzy 0 2 2 1;
        xzzz 0 2 2 2;
        xzzw 0 2 2 3;
        xzwx 0 2 3 0;
        xzwy 0 2 3 1;
        xzwz 0 2 3 2;
        xzww 0 2 3 3;
        xwxx 0 3 0 0;
        xwxy 0 3 0 1;
        xwxz 0 3 0 2;
        xwxw 0 3 0 3;
        xwyx 0 3 1 0;
        xwyy 0 3 1 1;
        xwyz 0 3 1 2;
        xwyw 0 3 1 3;
        xwzx 0 3 2 0;
        xwzy 0 3 2 1;
        xwzz 0 3 2 2;
        xwzw 0 3 2 3;
        xwwx 0 3 3 0;
        xwwy 0 3 3 1;
        xwwz 0 3 3 2;
        xwww 0 3 3 3;
        yxxx 1 0 0 0;
        yxxy 1 0 0 1;
        yxxz 1 0 0 2;
        yxxw 1 0 0 3;
        yxyx 1 0 1 0;
        yxyy 1 0 1 1;
        yxyz 1 0 1 2;
        yxyw 1 0 1 3;
        yxzx 1 0 2 0;
        yxzy 1 0 2 1;
        yxzz 1 0 2 2;
        yxzw 1 0 2 3;
        yxwx 1 0 3 0;
        yxwy 1 0 3 1;
        yxwz 1 0 3 2;
        yxww 1 0 3 3;
        yyxx 1 1 0 0;
        yyxy 1 1 0 1;
        yyxz 1 1 0 2;
        yyxw 1 1 0 3;
        yyyx 1 1 1 0;
        yyyy 1 1 1 1;
        yyyz 1 1 1 2;
        yyyw 1 1 1 3;
        yyzx 1 1 2 0;
        yyzy 1 1 2 1;
        yyzz 1 1 2 2;
        yyzw 1 1 2 3;
        yywx 1 1 3 0;
        yywy 1 1 3 1;
        yywz 1 1 3 2;
        yyww 1 1 3 3;
        yzxx 1 2 0 0;
        yzxy 1 2 0 1;
        yzxz 1 2 0 2;
        yzxw 1 2 0 3;
        yzyx 1 2 1 0;
        yzyy 1 2 1 1;
        yzyz 1 2 1 2;
        yzyw 1 2 1 3;
        yzzx 1 2 2 0;
        yzzy 1 2 2 1;
        yzzz 1 2 2 2;
        yzzw 1 2 2 3;
        yzwx 1 2 3 0;
        yzwy 1 2 3 1;
        yzwz 1 2 3 2;
        yzww 1 2 3 3;
        ywxx 1 3 0 0;
        ywxy 1 3 0 1;
        ywxz 1 3 0 2;
        ywxw 1 3 0 3;
        ywyx 1 3 1 0;
        ywyy 1 3 1 1;
        ywyz 1 3 1 2;
        ywyw 1 3 1 3;
        ywzx 1 3 2 0;
        ywzy 1 3 2 1;
        ywzz 1 3 2 2;
        ywzw 1 3 2 3;
        ywwx 1 3 3 0;
        ywwy 1 3 3 1;
        ywwz 1 3 3 2;
        ywww 1 3 3 3;
        zxxx 2 0 0 0;
        zxxy 2 0 0 1;
        zxxz 2 0 0 2;
        zxxw 2 0 0 3;
        zxyx 2 0 1 0;
        zxyy 2 0 1 1;
        zxyz 2 0 1 2;
        zxyw 2 0 1 3;
        zxzx 2 0 2 0;
        zxzy 2 0 2 1;
        zxzz 2 0 2 2;
        zxzw 2 0 2 3;
        zxwx 2 0 3 0;
        zxwy 2 0 3 1;
        zxwz 2 0 3 2;
        zxww 2 0 3 3;
        zyxx 2 1 0 0;
        zyxy 2 1 0 1;
        zyxz 2 1 0 2;
        zyxw 2 1 0 3;
        zyyx 2 1 1 0;
        zyyy 2 1 1 1;
        zyyz 2 1 1 2;
        zyyw 2 1 1 3;
        zyzx 2 1 2 0;
        zyzy 2 1 2 1;
        zyzz 2 1 2 2;
        zyzw 2 1 2 3;
        zywx 2 1 3 0;
        zywy 2 1 3 1;
        zywz 2 1 3 2;
        zyww 2 1 3 3;
        zzxx 2 2 0 0;
        zzxy 2 2 0 1;
        zzxz 2 2 0 2;
        zzxw 2 2 0 3;
        zzyx 2 2 1 0;
        zzyy 2 2 1 1;
        zzyz 2 2 1 2;
        zzyw 2 2 1 3;
        zzzx 2 2 2 0;
        zzzy 2 2 2 1;
        zzzz 2 2 2 2;
        zzzw 2 2 2 3;
        zzwx 2 2 3 0;
        zzwy 2 2 3 1;
        zzwz 2 2 3 2;
        zzww 2 2 3 3;
        zwxx 2 3 0 0;
        zwxy 2 3 0 1;
        zwxz 2 3 0 2;
        zwxw 2 3 0 3;
        zwyx 2 3 1 0;
        zwyy 2 3 1 1;
        zwyz 2 3 1 2;
        zwyw 2 3 1 3;
        zwzx 2 3 2 0;
        zwzy 2 3 2 1;
        zwzz 2 3 2 2;
        zwzw 2 3 2 3;
        zwwx 2 3 3 0;
        zwwy 2 3 3 1;
        zwwz 2 3 3 2;
        zwww 2 3 3 3;
        wxxx 3 0 0 0;
        wxxy 3 0 0 1;
        wxxz 3 0 0 2;
        wxxw 3 0 0 3;
        wxyx 3 0 1 0;
        wxyy 3 0 1 1;
        wxyz 3 0 1 2;
        wxyw 3 0 1 3;
        wxzx 3 0 2 0;
        wxzy 3 0 2 1;
        wxzz 3 0 2 2;
        wxzw 3 0 2 3;
        wxwx 3 0 3 0;
        wxwy 3 0 3 1;
        wxwz 3 0 3 2;
        wxww 3 0 3 3;
        wyxx 3 1 0 0;
        wyxy 3 1 0 1;
        wyxz 3 1 0 2;
        wyxw 3 1 0 3;
        wyyx 3 1 1 0;
        wyyy 3 1 1 1;
        wyyz 3 1 1 2;
        wyyw 3 1 1 3;
        wyzx 3 1 2 0;
        wyzy 3 1 2 1;
        wyzz 3 1 2 2;
        wyzw 3 1 2 3;
        wywx 3 1 3 0;
        wywy 3 1 3 1;
        wywz 3 1 3 2;
        wyww 3 1 3 3;
        wzxx 3 2 0 0;
        wzxy 3 2 0 1;
        wzxz 3 2 0 2;
        wzxw 3 2 0 3;
        wzyx 3 2 1 0;
        wzyy 3 2 1 1;
        wzyz 3 2 1 2;
        wzyw 3 2 1 3;
        wzzx 3 2 2 0;
        wzzy 3 2 2 1;
        wzzz 3 2 2 2;
        wzzw 3 2 2 3;
        wzwx 3 2 3 0;
        wzwy 3 2 3 1;
        wzwz 3 2 3 2;
        wzww 3 2 3 3;
        wwxx 3 3 0 0;
        wwxy 3 3 0 1;
        wwxz 3 3 0 2;
        wwxw 3 3 0 3;
        wwyx 3 3 1 0;
        wwyy 3 3 1 1;
        wwyz 3 3 1 2;
        wwyw 3 3 1 3;
        wwzx 3 3 2 0;
        wwzy 3 3 2 1;
        wwzz 3 3 2 2;
        wwzw 3 3 2 3;
        wwwx 3 3 3 0;
        wwwy 3 3 3 1;
        wwwz 3 3 3 2;
        wwww 3 3 3 3;
    }
}


impl std::ops::Index<usize> for Simd32x4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.f32x4[index] }
    }
}

impl std::ops::Index<usize> for Simd32x3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.f32x3[index] }
    }
}

impl std::ops::Index<usize> for Simd32x2 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.f32x2[index] }
    }
}

impl std::ops::IndexMut<usize> for Simd32x4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.f32x4[index] }
    }
}

impl std::ops::IndexMut<usize> for Simd32x3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.f32x3[index] }
    }
}

impl std::ops::IndexMut<usize> for Simd32x2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.f32x2[index] }
    }
}

impl From<Simd32x4> for [f32; 4] {
    fn from(simd: Simd32x4) -> Self {
        unsafe { simd.f32x4 }
    }
}

impl From<Simd32x3> for [f32; 3] {
    fn from(simd: Simd32x3) -> Self {
        unsafe { simd.f32x3 }
    }
}

impl From<Simd32x2> for [f32; 2] {
    fn from(simd: Simd32x2) -> Self {
        unsafe { simd.f32x2 }
    }
}

impl From<[f32; 4]> for Simd32x4 {
    fn from(f32x4: [f32; 4]) -> Self {
        Self { f32x4 }
    }
}

impl From<[f32; 3]> for Simd32x3 {
    fn from(f32x3: [f32; 3]) -> Self {
        Self { f32x3 }
    }
}

impl From<[f32; 2]> for Simd32x2 {
    fn from(f32x2: [f32; 2]) -> Self {
        Self { f32x2 }
    }
}

impl From<f32> for Simd32x4 {
    fn from(value: f32) -> Self {
        Self {
            f32x4: [value, value, value, value],
        }
    }
}

impl From<f32> for Simd32x3 {
    fn from(value: f32) -> Self {
        Self { f32x3: [value, value, value] }
    }
}

impl From<f32> for Simd32x2 {
    fn from(value: f32) -> Self {
        Self { f32x2: [value, value] }
    }
}

impl std::fmt::Debug for Simd32x4 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_list().entries([self[0], self[1], self[2], self[3]].iter()).finish()
    }
}

impl std::fmt::Debug for Simd32x3 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_list().entries([self[0], self[1], self[2]].iter()).finish()
    }
}

impl std::fmt::Debug for Simd32x2 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_list().entries([self[0], self[1]].iter()).finish()
    }
}

impl std::ops::Add<Simd32x4> for Simd32x4 {
    type Output = Simd32x4;

    fn add(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { f128: _mm_add_ps(self.f128, other.f128) },
            { f128: vaddq_f32(self.f128, other.f128) },
            { v128: f32x4_add(self.v128, other.v128) },
            { f32x4: [
                self.f32x4[0] + other.f32x4[0],
                self.f32x4[1] + other.f32x4[1],
                self.f32x4[2] + other.f32x4[2],
                self.f32x4[3] + other.f32x4[3],
            ] },
        )
    }
}

impl std::ops::Add<Simd32x3> for Simd32x3 {
    type Output = Simd32x3;

    fn add(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { v32x4: unsafe { self.v32x4 + other.v32x4 } },
            { f32x3: [
                self.f32x3[0] + other.f32x3[0],
                self.f32x3[1] + other.f32x3[1],
                self.f32x3[2] + other.f32x3[2],
            ] },
        )
    }
}

impl std::ops::Add<Simd32x2> for Simd32x2 {
    type Output = Simd32x2;

    fn add(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { v32x4: unsafe { self.v32x4 + other.v32x4 } },
            { f32x2: [
                self.f32x2[0] + other.f32x2[0],
                self.f32x2[1] + other.f32x2[1],
            ] },
        )
    }
}

impl std::ops::Sub<Simd32x4> for Simd32x4 {
    type Output = Simd32x4;

    fn sub(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { f128: _mm_sub_ps(self.f128, other.f128) },
            { f128: vsubq_f32(self.f128, other.f128) },
            { v128: f32x4_sub(self.v128, other.v128) },
            { f32x4: [
                self.f32x4[0] - other.f32x4[0],
                self.f32x4[1] - other.f32x4[1],
                self.f32x4[2] - other.f32x4[2],
                self.f32x4[3] - other.f32x4[3],
            ] },
        )
    }
}

impl std::ops::Sub<Simd32x3> for Simd32x3 {
    type Output = Simd32x3;

    fn sub(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { v32x4: unsafe { self.v32x4 - other.v32x4 } },
            { f32x3: [
                self.f32x3[0] - other.f32x3[0],
                self.f32x3[1] - other.f32x3[1],
                self.f32x3[2] - other.f32x3[2],
            ] },
        )
    }
}

impl std::ops::Sub<Simd32x2> for Simd32x2 {
    type Output = Simd32x2;

    fn sub(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { v32x4: unsafe { self.v32x4 - other.v32x4 } },
            { f32x2: [
                self.f32x2[0] - other.f32x2[0],
                self.f32x2[1] - other.f32x2[1],
            ] },
        )
    }
}

impl std::ops::Neg for Simd32x4 {
    type Output = Simd32x4;

    fn neg(self) -> Self {
        Self::from(0.0) - self
    }
}

impl std::ops::Neg for Simd32x3 {
    type Output = Simd32x3;

    fn neg(self) -> Self {
        Self::from(0.0) - self
    }
}

impl std::ops::Neg for Simd32x2 {
    type Output = Simd32x2;

    fn neg(self) -> Self {
        Self::from(0.0) - self
    }
}

impl std::ops::Mul<Simd32x4> for Simd32x4 {
    type Output = Simd32x4;

    fn mul(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { f128: _mm_mul_ps(self.f128, other.f128) },
            { f128: vmulq_f32(self.f128, other.f128) },
            { v128: f32x4_mul(self.v128, other.v128) },
            { f32x4: [
                self.f32x4[0] * other.f32x4[0],
                self.f32x4[1] * other.f32x4[1],
                self.f32x4[2] * other.f32x4[2],
                self.f32x4[3] * other.f32x4[3],
            ] },
        )
    }
}

impl std::ops::Mul<Simd32x3> for Simd32x3 {
    type Output = Simd32x3;

    fn mul(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { v32x4: unsafe { self.v32x4 * other.v32x4 } },
            { f32x3: [
                self.f32x3[0] * other.f32x3[0],
                self.f32x3[1] * other.f32x3[1],
                self.f32x3[2] * other.f32x3[2],
            ] },
        )
    }
}

impl std::ops::Mul<Simd32x2> for Simd32x2 {
    type Output = Simd32x2;

    fn mul(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { v32x4: unsafe { self.v32x4 * other.v32x4 } },
            { f32x2: [
                self.f32x2[0] * other.f32x2[0],
                self.f32x2[1] * other.f32x2[1],
            ] },
        )
    }
}

impl std::ops::Div<Simd32x4> for Simd32x4 {
    type Output = Simd32x4;

    fn div(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { f128: _mm_div_ps(self.f128, other.f128) },
            { f128: vdivq_f32(self.f128, other.f128) },
            { v128: f32x4_div(self.v128, other.v128) },
            { f32x4: [
                self.f32x4[0] / other.f32x4[0],
                self.f32x4[1] / other.f32x4[1],
                self.f32x4[2] / other.f32x4[2],
                self.f32x4[3] / other.f32x4[3],
            ] },
        )
    }
}

impl std::ops::Div<Simd32x3> for Simd32x3 {
    type Output = Simd32x3;

    fn div(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { v32x4: unsafe { self.v32x4 / other.v32x4 } },
            { f32x3: [
                self.f32x3[0] / other.f32x3[0],
                self.f32x3[1] / other.f32x3[1],
                self.f32x3[2] / other.f32x3[2],
            ] },
        )
    }
}

impl std::ops::Div<Simd32x2> for Simd32x2 {
    type Output = Simd32x2;

    fn div(self, other: Self) -> Self {
        match_architecture!(
            Self,
            { v32x4: unsafe { self.v32x4 / other.v32x4 } },
            { f32x2: [
                self.f32x2[0] / other.f32x2[0],
                self.f32x2[1] / other.f32x2[1],
            ] },
        )
    }
}








// encase implementations are based on this code
//  https://github.com/teoxoy/encase/blob/main/src/types/vector.rs



impl encase::private::AsRefVectorParts<f32, 4> for Simd32x4 {
    #[inline]
    fn as_ref_parts(&self) -> &[f32; 4] {
        unsafe { &self.f32x4 }
    }
}
impl encase::private::AsMutVectorParts<f32, 4> for Simd32x4 {
    #[inline]
    fn as_mut_parts(&mut self) -> &mut [f32; 4] {
        unsafe { &mut self.f32x4 }
    }
}
impl encase::private::FromVectorParts<f32, 4> for Simd32x4 {
    #[inline]
    fn from_parts(parts: [f32; 4]) -> Self {
        Simd32x4::from(parts)
    }
}
impl encase::ShaderType for Simd32x4 {
    type ExtraMetadata = ();
    const METADATA: Metadata<Self::ExtraMetadata> = {
        let f32_size = <f32 as encase::private::ShaderSize>::SHADER_SIZE;
        let four = NonZeroU64::new(4u64);
        // const eval can be annoying
        let four_f32s = match four {
            Some(four) => f32_size.saturating_mul(four),
            None => f32_size,
        };
        let size = encase::private::SizeValue::from(four_f32s);
        let alignment = encase::private::AlignmentValue::from_next_power_of_two_size(size);
        Metadata {
            alignment,
            has_uniform_min_alignment: false,
            min_size: size,
            is_pod: <[f32; 4] as encase::ShaderType>::METADATA.is_pod(),
            extra: (),
        }
    };
}
impl encase::ShaderSize for Simd32x4 {}
impl encase::private::WriteInto for Simd32x4 {
    #[inline]
    fn write_into<B>(&self, writer: &mut Writer<B>) where B: BufferMut
    {
        let elements = encase::private::AsRefVectorParts::<f32, 4>::as_ref_parts(self);
        encase::private::WriteInto::write_into(elements, writer);
    }
}
impl encase::private::ReadFrom for Simd32x4 {
    #[inline]
    fn read_from<B>(&mut self, reader: &mut Reader<B>) where B: BufferRef
    {
        let elements = encase::private::AsMutVectorParts::<f32, 4>::as_mut_parts(self);
        encase::private::ReadFrom::read_from(elements, reader);
    }
}
impl encase::private::CreateFrom for Simd32x4 {
    #[inline]
    fn create_from<B>(reader: &mut Reader<B>) -> Self where B: BufferRef
    {
        let elements = encase::private::CreateFrom::create_from(reader);
        encase::private::FromVectorParts::<f32, 4>::from_parts(elements)
    }
}










impl encase::private::AsRefVectorParts<f32, 3> for Simd32x3 {
    #[inline]
    fn as_ref_parts(&self) -> &[f32; 3] {
        unsafe { &self.f32x3 }
    }
}
impl encase::private::AsMutVectorParts<f32, 3> for Simd32x3 {
    #[inline]
    fn as_mut_parts(&mut self) -> &mut [f32; 3] {
        unsafe { &mut self.f32x3 }
    }
}
impl encase::private::FromVectorParts<f32, 3> for Simd32x3 {
    #[inline]
    fn from_parts(parts: [f32; 3]) -> Self {
        Simd32x3::from(parts)
    }
}
impl encase::ShaderType for Simd32x3 {
    type ExtraMetadata = ();
    const METADATA: Metadata<Self::ExtraMetadata> = {
        let f32_size = <f32 as encase::private::ShaderSize>::SHADER_SIZE;
        let three = NonZeroU64::new(3u64);
        // const eval can be annoying
        let three_f32s = match three {
            Some(three) => f32_size.saturating_mul(three),
            None => f32_size,
        };
        let size = encase::private::SizeValue::from(three_f32s);
        let alignment = encase::private::AlignmentValue::from_next_power_of_two_size(size);
        Metadata {
            alignment,
            has_uniform_min_alignment: false,
            min_size: size,
            is_pod: <[f32; 3] as encase::ShaderType>::METADATA.is_pod(),
            extra: (),
        }
    };
}
impl encase::ShaderSize for Simd32x3 {}
impl encase::private::WriteInto for Simd32x3 {
    #[inline]
    fn write_into<B>(&self, writer: &mut Writer<B>) where B: BufferMut
    {
        let elements = encase::private::AsRefVectorParts::<f32, 3>::as_ref_parts(self);
        encase::private::WriteInto::write_into(elements, writer);
    }
}
impl encase::private::ReadFrom for Simd32x3 {
    #[inline]
    fn read_from<B>(&mut self, reader: &mut Reader<B>) where B: BufferRef
    {
        let elements = encase::private::AsMutVectorParts::<f32, 3>::as_mut_parts(self);
        encase::private::ReadFrom::read_from(elements, reader);
    }
}
impl encase::private::CreateFrom for Simd32x3 {
    #[inline]
    fn create_from<B>(reader: &mut Reader<B>) -> Self where B: BufferRef
    {
        let elements = encase::private::CreateFrom::create_from(reader);
        encase::private::FromVectorParts::<f32, 3>::from_parts(elements)
    }
}











impl encase::private::AsRefVectorParts<f32, 2> for Simd32x2 {
    #[inline]
    fn as_ref_parts(&self) -> &[f32; 2] {
        unsafe { &self.f32x2 }
    }
}
impl encase::private::AsMutVectorParts<f32, 2> for Simd32x2 {
    #[inline]
    fn as_mut_parts(&mut self) -> &mut [f32; 2] {
        unsafe { &mut self.f32x2 }
    }
}
impl encase::private::FromVectorParts<f32, 2> for Simd32x2 {
    #[inline]
    fn from_parts(parts: [f32; 2]) -> Self {
        Simd32x2::from(parts)
    }
}
impl encase::ShaderType for Simd32x2 {
    type ExtraMetadata = ();
    const METADATA: Metadata<Self::ExtraMetadata> = {
        let f32_size = <f32 as encase::private::ShaderSize>::SHADER_SIZE;
        let two = NonZeroU64::new(2u64);
        // const eval can be annoying
        let two_f32s = match two {
            Some(two) => f32_size.saturating_mul(two),
            None => f32_size,
        };
        let size = encase::private::SizeValue::from(two_f32s);
        let alignment = encase::private::AlignmentValue::from_next_power_of_two_size(size);
        Metadata {
            alignment,
            has_uniform_min_alignment: false,
            min_size: size,
            is_pod: <[f32; 2] as encase::ShaderType>::METADATA.is_pod(),
            extra: (),
        }
    };
}
impl encase::ShaderSize for Simd32x2 {}
impl encase::private::WriteInto for Simd32x2 {
    #[inline]
    fn write_into<B>(&self, writer: &mut Writer<B>) where B: BufferMut
    {
        let elements = encase::private::AsRefVectorParts::<f32, 2>::as_ref_parts(self);
        encase::private::WriteInto::write_into(elements, writer);
    }
}
impl encase::private::ReadFrom for Simd32x2 {
    #[inline]
    fn read_from<B>(&mut self, reader: &mut Reader<B>) where B: BufferRef
    {
        let elements = encase::private::AsMutVectorParts::<f32, 2>::as_mut_parts(self);
        encase::private::ReadFrom::read_from(elements, reader);
    }
}
impl encase::private::CreateFrom for Simd32x2 {
    #[inline]
    fn create_from<B>(reader: &mut Reader<B>) -> Self where B: BufferRef
    {
        let elements = encase::private::CreateFrom::create_from(reader);
        encase::private::FromVectorParts::<f32, 2>::from_parts(elements)
    }
}










impl NearlyEqTol<Self, f32, f32> for Simd32x4 {}
impl NearlyEqEps<Self, f32, f32> for Simd32x4 {
    fn nearly_eq_eps(&self, other: &Self, eps: &EpsToleranceType<f32, f32>) -> bool {
        let s = unsafe { &self.f32x4 };
        let o = unsafe { &other.f32x4 };
        s.nearly_eq_eps(o, eps)
    }
}
impl NearlyEqUlps<Self, f32, f32> for Simd32x4 {
    fn nearly_eq_ulps(&self, other: &Self, ulps: &UlpsToleranceType<f32, f32>) -> bool {
        let s = unsafe { &self.f32x4 };
        let o = unsafe { &other.f32x4 };
        s.nearly_eq_ulps(o, ulps)
    }
}
impl nearly::NearlyEq<Self, f32, f32> for Simd32x4 {}











impl NearlyEqTol<Self, f32, f32> for Simd32x3 {}
impl NearlyEqEps<Self, f32, f32> for Simd32x3 {
    fn nearly_eq_eps(&self, other: &Self, eps: &EpsToleranceType<f32, f32>) -> bool {
        let s = unsafe { &self.f32x3 };
        let o = unsafe { &other.f32x3 };
        s.nearly_eq_eps(o, eps)
    }
}
impl NearlyEqUlps<Self, f32, f32> for Simd32x3 {
    fn nearly_eq_ulps(&self, other: &Self, ulps: &UlpsToleranceType<f32, f32>) -> bool {
        let s = unsafe { &self.f32x3 };
        let o = unsafe { &other.f32x3 };
        s.nearly_eq_ulps(o, ulps)
    }
}
impl nearly::NearlyEq<Self, f32, f32> for Simd32x3 {}











impl NearlyEqTol<Self, f32, f32> for Simd32x2 {}
impl NearlyEqEps<Self, f32, f32> for Simd32x2 {
    fn nearly_eq_eps(&self, other: &Self, eps: &EpsToleranceType<f32, f32>) -> bool {
        let s = unsafe { &self.f32x2 };
        let o = unsafe { &other.f32x2 };
        s.nearly_eq_eps(o, eps)
    }
}
impl NearlyEqUlps<Self, f32, f32> for Simd32x2 {
    fn nearly_eq_ulps(&self, other: &Self, ulps: &UlpsToleranceType<f32, f32>) -> bool {
        let s = unsafe { &self.f32x2 };
        let o = unsafe { &other.f32x2 };
        s.nearly_eq_ulps(o, ulps)
    }
}
impl nearly::NearlyEq<Self, f32, f32> for Simd32x2 {}




















//