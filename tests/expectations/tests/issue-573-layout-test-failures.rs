/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Default)]
pub struct Outer {
    pub i: u8,
}
#[repr(C)]
pub struct AutoIdVector {
    pub ar: Outer,
}
#[test]
fn bindgen_test_layout_AutoIdVector() {
    assert_eq!(::std::mem::size_of::<AutoIdVector>() , 1usize , concat ! (
               "Size of: " , stringify ! ( AutoIdVector ) ));
    assert_eq! (::std::mem::align_of::<AutoIdVector>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( AutoIdVector ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const AutoIdVector ) ) . ar as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( AutoIdVector ) , "::" ,
                stringify ! ( ar ) ));
}
impl Default for AutoIdVector {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}