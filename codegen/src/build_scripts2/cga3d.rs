#![allow(non_upper_case_globals)]
use crate::{ga, multi_vecs};


multi_vecs!(e12345;
    FlatPoint   as e15, e25, e35, e45;
    Line        as e415, e425, e435 | e235, e315, e125;
    Plane       as e4235, e4315, e4125, e3215;

    RoundPoint  as e1, e2, e3, | e4, e5;
    Dipole      as e41, e42, e43 | e23, e31, e12 | e15, e25, e35, e45;
    Circle      as e423, e431, e412, e321 | e415, e425, e435 | e235, e315, e125;
    Sphere      as e4235, e4315, e4125 | e1234, e3215;

    Motor       as e415, e425, e435, e12345 | e235, e315, e125;
    Flector     as e15, e25, e35, e45 | e4235, e4315, e4125, e3215;
);


/// Lengyel styled CGA of 5 dimensions representing 3 dimensions
#[test]
pub fn cga3d_script() {
    let cga3d = ga!(e12345;
        1 => e1, e2, e3, eP;
        -1 => eM;
        where
        e4 => 0.5 * (eM - eP);
        e5 => eP + eM;
    );
    let mut declarations = register_multi_vecs(cga3d);
    let repo = declarations.finished();


    // TODO get TraitImplRegistry, TraitDefRegistry
    // TODO MultiVectorRegistry?


}