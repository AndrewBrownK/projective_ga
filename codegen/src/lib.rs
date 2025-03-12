#![feature(try_blocks)]
#![feature(iter_intersperse)]
#![feature(exit_status_error)]
#![feature(associated_type_defaults)]
// TODO evaluate if I actually need/use these
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(unsized_const_params)]
#![feature(const_trait_impl)]
#![feature(marker_trait_attr)]
#![feature(concat_idents)]
#![feature(box_patterns)]
#![feature(let_chains)]
// TODO rendering round objects in polygons (triangles)
//  https://www.youtube.com/watch?v=VEnglRKNHjU

// TODO how to handle stuff like Hamish Todd's interpretation where an object of a single grade can
//  be multiple different objects? like in CGA a grade 2 object being a dipole or circle, depending on characteristics.
//  The answer is (I think) wrapper types. In a situation like this, you might leave the
//  multivector names to their defaults like Vector1 Vector2 Vector3 etc. Then the wrapper type
//  could be Circle<V>, Dipole<V>, etc. And a Circle could accept a Vector2 or Vector3 inside it.
//  Then control how it can be instantiated by using non-generic constructors. The return type
//  of the constructors would be Options or Results, depending on if the underlying Vector
//  meets the appropriate criteria. You could also take the raw Vector give it a method that
//  returns Either<Circle, Dipole> depending on what the data matches. In fact it would probably
//  have to be Either<Circle, NullObject, Dipole>. The annoying thing is making these wrapper types
//  work seamlessly with traits methods/operations.



// TODO calculus: https://en.wikipedia.org/wiki/Geometric_calculus
pub mod algebra;
pub mod ast;
pub mod emit;
mod shader_support;
pub mod utility;
mod validate;
// TODO eventually migrate to Portable SIMD once it is stabilized
//  https://github.com/rust-lang/rust/issues/86656
//  That would also be the appropriate time to consider f64 support. Not eager until then.
//  See also "Transmutability": https://github.com/rust-lang/rust/issues/99571
mod simd;
pub mod build_scripts {
    pub mod common_traits;
}
pub mod debug_scripts {
    mod cga3d_faceted;
    mod pga3d;
    mod rga3d;
}

const SIMD_SRC: &'static str = include_str!("simd.rs");

pub use emit::rust::Rust;
pub use emit::slang::Slang;
pub mod elements {
    pub use crate::algebra::basis::elements::*;
}
