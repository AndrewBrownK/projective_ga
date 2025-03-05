pub mod data;
pub mod integrations;
pub mod simd;
pub mod traits;
#[allow(non_camel_case_types)]
pub mod elements {
    pub struct scalar;
    pub struct e1;
    pub struct e2;
    pub struct e3;
    pub struct e4;
    pub struct e5;
    pub struct e12;
    pub struct e31;
    pub struct e41;
    pub struct e15;
    pub struct e23;
    pub struct e42;
    pub struct e25;
    pub struct e43;
    pub struct e35;
    pub struct e45;
    pub struct e321;
    pub struct e412;
    pub struct e125;
    pub struct e431;
    pub struct e315;
    pub struct e415;
    pub struct e423;
    pub struct e235;
    pub struct e425;
    pub struct e435;
    pub struct e1234;
    pub struct e3215;
    pub struct e4125;
    pub struct e4315;
    pub struct e4235;
    pub struct e12345;
}
#[test]
fn double_check_this_crate_compiles() {}
