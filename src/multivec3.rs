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
pub struct MultiVec3<T> {
    pub r: T,
    pub x: T,
    pub y: T,
    pub z: T,
    pub xy: T,
    pub xz: T,
    pub yz: T,
    pub xyz: T,
}
