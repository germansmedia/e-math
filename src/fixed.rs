use {
    crate::*,
    std::{
        cmp::{
            PartialEq,
            PartialOrd,
            Ordering,
        },
        fmt::{
            Display,
            Debug,
            Formatter,
            Result,
        },
        ops::{
            Shl,
            Shr,
        },
    },
};

/// Fixed point template.
/// 
/// Fixed point numbers are sometimes easier to use than floating point numbers. Also, some architectures do not support
/// floating point numbers at all.
/// 
/// Features:
/// * All arithmetic operations exist for `Fixed`, including trigonometry, exponents, logarithms, etc.
/// * Basic arithmetic with integer numbers (i.e. you can `let x: Fixed<i32:16> = 4 * some_other_fixed;`).
/// * Translation to/from `f32`, `f64` and [`Rational`] using [`From`] trait.
/// * `Fixed` can be used to build [`Complex`], [`Quaternion`], vectors, multivectors and matrices.
/// 
/// To make a `Fixed` number, specify the underlying integer as well as the number of fractional bits. `Fixed<i32,16>` creates
/// a 16:16 fixed point number, `Fixed<i128,8>` creates a 120:8 fixed point number, etc.
pub struct Fixed<T,const B: usize>(T);

impl<T,const B: usize> Fixed<T,B> {
    const BITS: usize = B;
}

impl<T: Copy,const B: usize> Display for Fixed<T,B> where f64: From<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        let value = f64::from(self.0) / 2.0f64.powf(Self::BITS as f64);
        write!(f,"{}",value)
    }
}

macro_rules! fixed_impl {
    ($($t:ty)*) => ($(

        // scalar == fixed
        impl<T: From<$t> + Shl<usize,Output=T> + PartialEq,const B: usize> PartialEq<Fixed<T,B>> for $t {
            fn eq(&self,other: &Fixed<T,B>) -> bool {
                let scalar = T::from(*self) << B;
                scalar == other.0
            }
        }

        // fixed == scalar
        impl<T: From<$t> + Shl<usize,Output=T> + PartialEq,const B: usize> PartialEq<$t> for Fixed<T,B> {
            fn eq(&self,other: &$t) -> bool {
                let scalar = T::from(*other) << B;
                self.0 == scalar
            }
        }

        // scalar ? fixed
        impl<T: From<$t> + Shl<usize,Output=T> + PartialOrd,const B: usize> PartialOrd<Fixed<T,B>> for $t {
            fn partial_cmp(&self,other: &Fixed<T,B>) -> Option<Ordering> {
                let scalar = T::from(*self) << B;
                scalar.partial_cmp(&other.0)
            }
        }

        // fixed ? scalar
        impl<T: From<$t> + Shl<usize,Output=T> + PartialOrd,const B: usize> PartialOrd<$t> for Fixed<T,B> {
            fn partial_cmp(&self,other: &$t) -> Option<Ordering> {
                let scalar = T::from(*other) << B;
                self.0.partial_cmp(&scalar)
            }
        }

        // scalar + fixed
        impl<T: From<$t> + Add<Output=T> + Shl<usize,Output=T>,const B: usize> Add<Fixed<T,B>> for $t {
            type Output = Fixed<T,B>;
            fn add(self,other: Fixed<T,B>) -> Self::Output {
                Fixed((T::from(self) << B) + other.0)
            }
        }

        // fixed + scalar
        impl<T: From<$t> + Add<Output=T> + Shl<usize,Output=T>,const B: usize> Add<$t> for Fixed<T,B> {
            type Output = Self;
            fn add(self,other: $t) -> Self::Output {
                Fixed(self.0 + (T::from(other) << B))
            }
        }

        // fixed += scalar
        impl<T: From<$t> + Add<Output=T> + AddAssign + Shl<usize,Output=T>,const B: usize> AddAssign<$t> for Fixed<T,B> {
            fn add_assign(&mut self,other: $t) {
                self.0 += (T::from(other) << B);
            }
        }

        // scalar - fixed
        impl<T: From<$t> + Sub<Output=T> + Shl<usize,Output=T>,const B: usize> Sub<Fixed<T,B>> for $t {
            type Output = Fixed<T,B>;
            fn sub(self,other: Fixed<T,B>) -> Self::Output {
                Fixed((T::from(self) << B) - other.0)
            }
        }

        // fixed - scalar
        impl<T: From<$t> + Sub<Output=T> + Shl<usize,Output=T>,const B: usize> Sub<$t> for Fixed<T,B> {
            type Output = Self;
            fn sub(self,other: $t) -> Self::Output {
                Fixed(self.0 - (T::from(other) << B))
            }
        }

        // fixed -= scalar
        impl<T: From<$t> + Sub<Output=T> + SubAssign + Shl<usize,Output=T>,const B: usize> SubAssign<$t> for Fixed<T,B> {
            fn sub_assign(&mut self,other: $t) {
                self.0 -= (T::from(other) << B);
            }
        }

        // scalar * fixed
        impl<T: From<$t> + Mul<Output=T>,const B: usize> Mul<Fixed<T,B>> for $t {
            type Output = Fixed<T,B>;
            fn mul(self,other: Fixed<T,B>) -> Self::Output {
                Fixed(T::from(self) * other.0)
            }
        }

        // fixed * scalar
        impl<T: From<$t> + Mul<Output=T>,const B: usize> Mul<$t> for Fixed<T,B> {
            type Output = Self;
            fn mul(self,other: $t) -> Self::Output {
                Fixed(self.0 * T::from(other))
            }
        }

        // fixed *= scalar
        impl<T: From<$t> + MulAssign,const B: usize> MulAssign<$t> for Fixed<T,B> {
            fn mul_assign(&mut self,other: $t) {
                self.0 *= T::from(other);
            }
        }

        // scalar / fixed
        impl<T: From<$t> + Div<Output=T> + Shl<usize,Output=T>,const B: usize> Div<Fixed<T,B>> for $t {
            type Output = Fixed<T,B>;
            fn div(self,other: Fixed<T,B>) -> Self::Output {
                Fixed((T::from(self) << (2 * B)) / other.0)
            }
        }

        // fixed / scalar
        impl<T: From<$t> + Div<Output=T>,const B: usize> Div<$t> for Fixed<T,B> {
            type Output = Self;
            fn div(self,other: $t) -> Self::Output {
                Fixed(self.0 / T::from(other))
            }
        }

        // fixed /= scalar
        impl<T: From<$t> + DivAssign,const B: usize> DivAssign<$t> for Fixed<T,B> {
            fn div_assign(&mut self,other: $t) {
                self.0 /= T::from(other);
            }
        }
    )*)
}

fixed_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

// fixed == fixed
impl<T: PartialEq,const B: usize> PartialEq<Fixed<T,B>> for Fixed<T,B> {
    fn eq(&self,other: &Fixed<T,B>) -> bool {
        self.0 == other.0
    }
}

// fixed ? fixed
impl<T: PartialOrd,const B: usize> PartialOrd<Fixed<T,B>> for Fixed<T,B> {
    fn partial_cmp(&self,other: &Fixed<T,B>) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

// fixed + fixed
impl<T: Add<Output=T>,const B: usize> Add<Fixed<T,B>> for Fixed<T,B> {
    type Output = Self;
    fn add(self,other: Self) -> Self::Output {
        Fixed(self.0 + other.0)
    }
}

// fixed += fixed
impl<T: AddAssign,const B: usize> AddAssign<Fixed<T,B>> for Fixed<T,B> {
    fn add_assign(&mut self,other: Fixed<T,B>) {
        self.0 += other.0;
    }
}

// fixed - fixed
impl<T: Sub<Output=T>,const B: usize> Sub<Fixed<T,B>> for Fixed<T,B> {
    type Output = Self;
    fn sub(self,other: Self) -> Self::Output {
        Fixed(self.0 - other.0)
    }
}

// fixed -= fixed
impl<T: SubAssign,const B: usize> SubAssign<Fixed<T,B>> for Fixed<T,B> {
    fn sub_assign(&mut self,other: Fixed<T,B>) {
        self.0 -= other.0;
    }
}

// fixed * fixed
impl<T: Mul<Output=T> + Shr<usize,Output=T>,const B: usize> Mul<Fixed<T,B>> for Fixed<T,B> {
    type Output = Self;
    fn mul(self,other: Self) -> Self::Output {
        Fixed(self.0 * other.0 >> B)
    }
}

// fixed *= fixed
impl<T: Copy + Mul<Output=T> + Shr<usize,Output=T>,const B: usize> MulAssign<Fixed<T,B>> for Fixed<T,B> {
    fn mul_assign(&mut self,other: Fixed<T,B>) {
        self.0 = (self.0 * other.0) >> B;
    }
}

// fixed / fixed
impl<T: Div<Output=T> + Shl<usize,Output=T>,const B: usize> Div<Fixed<T,B>> for Fixed<T,B> {
    type Output = Self;
    fn div(self,other: Self) -> Self::Output {
        Fixed((self.0 << B) / other.0)
    }
}

// fixed /= fixed
impl<T: Copy + Div<Output=T> + Shl<usize,Output=T>,const B: usize> DivAssign<Fixed<T,B>> for Fixed<T,B> {
    fn div_assign(&mut self,other: Fixed<T,B>) {
        self.0 = (self.0 << B) / other.0;
    }
}

// -fixed
impl<T: Neg<Output=T>,const B: usize> Neg for Fixed<T,B> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Fixed(-self.0)
    }
}

impl<T,const B: usize> Real for Fixed<T,B> {

}
