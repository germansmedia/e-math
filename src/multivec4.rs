use {
    crate::*,
    std::{
        cmp::PartialEq,
        fmt::{
            Display,
            Debug,
            Formatter,
            Result,
        },
        ops::{
            Add,
            Sub,
            Mul,
            Div,
            AddAssign,
            SubAssign,
            MulAssign,
            DivAssign,
            Neg,
        },
    },
};

#[derive(Copy,Clone,Debug)]
pub struct MultiVec4<T> {
    pub r: T,
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
    pub xy: T,
    pub xz: T,
    pub xw: T,
    pub yz: T,
    pub yw: T,
    pub zw: T,
    pub xyz: T,
    pub xzw: T,
    pub xyw: T,
    pub yzw: T,
    pub xyzw: T,
}
