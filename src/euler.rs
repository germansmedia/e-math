use {
    crate::*,
    std::{
        cmp::PartialEq,
        fmt::{
            Display,
            Debug,
            Formatter,
            Result
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
            Neg
        },
    },
};

#[derive(Copy,Clone,Debug)]
pub struct Euler<T> {
    pub y: T,
    pub p: T,
    pub r: T,
}
