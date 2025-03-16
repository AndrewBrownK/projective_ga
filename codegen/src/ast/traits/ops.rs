
#[repr(u32)]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash)]
pub enum UnaryOps {
    Neg,
    Not,
}
impl UnaryOps {
    pub fn rust_mod(self) -> &'static str {
        "std::ops::"
        // match self {
        //     UnaryOps::Neg => "core::ops::arith::",
        //     UnaryOps::Not => "core::ops::bit::",
        // }
    }
    pub fn rust_trait_name(self) -> &'static str {
        match self {
            UnaryOps::Neg => "Neg",
            UnaryOps::Not => "Not",
        }
    }
    pub fn rust_trait_method(self) -> &'static str {
        match self {
            UnaryOps::Neg => "neg",
            UnaryOps::Not => "not",
        }
    }
    pub fn rust_operator(self) -> &'static str {
        match self {
            UnaryOps::Neg => "-",
            UnaryOps::Not => "!",
        }
    }
    pub fn slang_trait_method(self) -> &'static str {
        match self {
            UnaryOps::Neg => "operator-",
            UnaryOps::Not => "operator!",
        }
    }
    pub fn slang_operator(self) -> &'static str {
        match self {
            UnaryOps::Neg => "-",
            UnaryOps::Not => "!",
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash)]
pub enum BinaryOps {
    Add,
    Sub,
    Mul,
    Div,
    Shl,
    Shr,
    BitAnd,
    BitOr,
    BitXor,
}
impl BinaryOps {
    pub fn rust_mod(self) -> &'static str {
        "std::ops::"
        // match self {
        //     BinaryOps::Add | BinaryOps::Sub | BinaryOps::Mul | BinaryOps::Div => "core::ops::arith::",
        //     BinaryOps::Shl | BinaryOps::Shr | BinaryOps::BitAnd | BinaryOps::BitOr | BinaryOps::BitXor => "core::ops::bit::",
        // }
    }

    pub fn rust_trait_name(self) -> &'static str {
        match self {
            BinaryOps::Add => "Add",
            BinaryOps::Sub => "Sub",
            BinaryOps::Mul => "Mul",
            BinaryOps::Div => "Div",
            BinaryOps::Shl => "Shl",
            BinaryOps::Shr => "Shr",
            BinaryOps::BitAnd => "BitAnd",
            BinaryOps::BitOr => "BitOr",
            BinaryOps::BitXor => "BitXor",
        }
    }

    pub fn rust_trait_method(self) -> &'static str {
        match self {
            BinaryOps::Add => "add",
            BinaryOps::Sub => "sub",
            BinaryOps::Mul => "mul",
            BinaryOps::Div => "div",
            BinaryOps::Shl => "shl",
            BinaryOps::Shr => "shr",
            BinaryOps::BitAnd => "bitand",
            BinaryOps::BitOr => "bitor",
            BinaryOps::BitXor => "bitxor",
        }
    }

    pub fn rust_operator(self) -> &'static str {
        match self {
            BinaryOps::Add => "+",
            BinaryOps::Sub => "-",
            BinaryOps::Mul => "*",
            BinaryOps::Div => "/",
            BinaryOps::Shl => "<<",
            BinaryOps::Shr => ">>",
            BinaryOps::BitAnd => "&",
            BinaryOps::BitOr => "|",
            BinaryOps::BitXor => "^",
        }
    }

    /// If None is returned, then slang does not support overloading of that operator
    pub fn slang_trait_method(self) -> Option<&'static str> {
        match self {
            BinaryOps::Add => Some("operator +"),
            BinaryOps::Sub => Some("operator -"),
            BinaryOps::Mul => Some("operator *"),
            BinaryOps::Div => Some("operator /"),
            BinaryOps::Shl => None,
            BinaryOps::Shr => None,
            BinaryOps::BitAnd => Some("operator &"),
            BinaryOps::BitOr => Some("operator |"),
            BinaryOps::BitXor => None,
        }
    }

    pub fn slang_operator(self) -> &'static str {
        match self {
            BinaryOps::Add => "+",
            BinaryOps::Sub => "-",
            BinaryOps::Mul => "*",
            BinaryOps::Div => "/",
            BinaryOps::Shl => panic!("Shl operator not supported in slang"),
            BinaryOps::Shr => panic!("Shr operator not supported in slang"),
            BinaryOps::BitAnd => "&",
            BinaryOps::BitOr => "|",
            BinaryOps::BitXor => panic!("bitxor operator not supported in slang"),
        }
    }
}

// TODO any use for these?
#[repr(u32)]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash)]
pub enum ExperimentalOps {
    Deref,
    Fn,
    Index,
    RangeBounds,
    Rem,
    FromResidual,
    OneSidedRange,
    Residual,
    Try,
}
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash)]
pub enum Ops {
    Unary(UnaryOps),
    Binary(BinaryOps),
    // Exp(ExperimentalOps),
}
impl Ops {
    pub fn rust_mod(self) -> &'static str {
        match self {
            Ops::Unary(op) => op.rust_mod(),
            Ops::Binary(op) => op.rust_mod(),
        }
    }
    pub const fn into_u32(self) -> u32 {
        match self {
            Ops::Unary(o) => o as u32,
            Ops::Binary(o) => (o as u32) + 2,
        }
    }
    pub fn rust_trait_name(self) -> &'static str {
        match self {
            Ops::Unary(op) => op.rust_trait_name(),
            Ops::Binary(op) => op.rust_trait_name(),
        }
    }
    pub fn rust_trait_method(self) -> &'static str {
        match self {
            Ops::Unary(op) => op.rust_trait_method(),
            Ops::Binary(op) => op.rust_trait_method(),
        }
    }
    pub fn rust_operator(self) -> &'static str {
        match self {
            Ops::Unary(op) => op.rust_operator(),
            Ops::Binary(op) => op.rust_operator(),
        }
    }
    pub fn slang_trait_method(self) -> Option<&'static str> {
        match self {
            Ops::Unary(op) => Some(op.rust_trait_method()),
            Ops::Binary(op) => op.slang_trait_method(),
        }
    }
    pub fn slang_operator(self) -> &'static str {
        match self {
            Ops::Unary(op) => op.rust_operator(),
            Ops::Binary(op) => op.slang_operator(),
        }
    }
}




#[macro_export]
macro_rules! operators {
    (
        $anti_scalar:ident
        $mv_repo:expr, $tir:ident
        $(; fancy_infix => $itr:ident)?
        $(; binary $($bop:ident => $btr:ident),+)?
        $(; unary $($uop:ident => $utr:ident),+ )?
        $(;)?
    ) => {
        {
            use $crate::build_scripts::common_traits::*;
            use $crate::ast::traits::BinaryOps::*;
            use $crate::ast::traits::UnaryOps::*;
            $($tir.generate_infix_trick($itr);)?
            $($(
                $tir.set_binary_operator::<_, $anti_scalar>($mv_repo.clone(), $bop, $btr);
            )+)?
            $($(
                $tir.set_unary_operator::<_, $anti_scalar>($mv_repo.clone(), $uop, $utr);
            )+)?
        }
    };
}
