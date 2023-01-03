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
pub struct Complex<T> {
    pub r: T,
    pub i: T,
}

impl<T: Mul<T,Output=T> + Add<T,Output=T> + Neg<Output=T>> Complex<T> {

    // |complex|
    pub fn norm(&self) -> T {
        (self.r * self.r + self.i * self.i).sqrt()
    }

    // complex argument
    pub fn arg(&self) -> T {
        atan2(self.r,self.i)
    }

    // complex conjugate
    pub fn conj(&self) -> Complex<T> {
        Complex {
            r: self.r,
            i: -self.i,
        }
    }
}

impl<T: Zero> Zero for Complex<T> {
    const ZERO: Self = Complex {
        r: T::ZERO,
        i: T::ZERO,
    };
}

// complex == complex
impl<T: PartialEq> PartialEq for Complex<T> {
    fn eq(&self,other: &Self) -> bool {
        (self.r == other.r) &&
        (self.i == other.i)
    }
}

impl<T: Zero + Display + PartialOrd> Display for Complex<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        let si = if self.i < T::ZERO {
            format!("{}i",self.i)
        }
        else {
            format!("+{}i",self.i)
        };
        write!(f,"{}{}",self.r,si)
    }
}

macro_rules! scalar_complex {
    ($t:ty) => {

        // scalar + complex
        impl Add<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn add(self,other: Complex<$t>) -> Complex<$t> {
                Complex {
                    r: self + other.r,
                    i: other.i,
                }
            }
        }

        // scalar - complex
        impl Sub<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn sub(self,other: Complex<$t>) -> Complex<$t> {
                Complex {
                    r: self - other.r,
                    i: -other.i,
                }
            }
        }

        // scalar * conplex
        impl Mul<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn mul(self,other: Complex<$t>) -> Complex<$t> {
                Complex {
                    r: self * other.r,
                    i: self * other.i,
                }
            }
        }

        // scalar / complex
        impl Div<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn div(self,other: Complex<$t>) -> Complex<$t> {
                let f = 1.0 / (other.r * other.r + other.i * other.i);
                Complex {
                    r: self * other.r * f,
                    i: -self * other.i * f,
                }
            }
        }
    }
}

scalar_complex!(f32);
scalar_complex!(f64);

// complex + scalar
impl<T: Add<T,Output=T>> Add<T> for Complex<T> {
    type Output = Self;
    fn add(self,other: T) -> Self {
        Complex {
            r: self.r + other,
            i: self.i,
        }
    }
}

// complex + complex
impl<T: Add<T,Output=T>> Add<Complex<T>> for Complex<T> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Complex {
            r: self.r + other.r,
            i: self.i + other.i,
        }
    }
}

// complex - scalar
impl<T: Add<T,Output=T> + Sub<T,Output=T>> Sub<T> for Complex<T> {
    type Output = Self;
    fn sub(self,other: T) -> Self {
        Complex {
            r: self.r + other,
            i: self.i,
        }
    }
}

// complex - complex
impl<T: Sub<T,Output=T>> Sub<Complex<T>> for Complex<T> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Complex {
            r: self.r - other.r,
            i: self.i - other.i,
        }
    }
}

// complex * scalar
impl<T: Mul<T,Output=T>> Mul<T> for Complex<T> {
    type Output = Self;
    fn mul(self,other: T) -> Self {
        Complex {
            r: self.r * other,
            i: self.i * other,
        }
    }
}

// complex * complex
impl<T: Add<T,Output=T> + Sub<T,Output=T> + Mul<T,Output=T>> Mul<Complex<T>> for Complex<T> {
    type Output = Self;
    fn mul(self,other: Self) -> Self {
        Complex {
            r: self.r * other.r - self.i * other.i,
            i: self.r * other.i + self.i * other.r,
        }
    }
}

// complex / scalar
impl<T: Div<T,Output=T>> Div<T> for Complex<T> {
    type Output = Self;
    fn div(self,other: T) -> Self {
        Complex {
            r: self.r / other,
            i: self.i / other,
        }
    }
}

// complex / complex
impl<T: Add<T,Output=T> + Sub<T,Output=T> + Mul<T,Output=T> + Div<T,Output=T>> Div<Complex<T>> for Complex<T> {
    type Output = Self;
    fn div(self,other: Self) -> Self {
        let f = other.r * other.r + other.i * other.i;
        Complex {
            r: (self.r * other.r + self.i * other.i) / f,
            i: (self.i * other.r - self.r * other.i) / f,
        }
    }
}

// complex += scalar
impl<T: AddAssign<T>> AddAssign<T> for Complex<T> {
    fn add_assign(&mut self,other: T) {
        self.r += other;
    }
}

// complex += complex
impl<T: AddAssign<T>> AddAssign<Complex<T>> for Complex<T> {
    fn add_assign(&mut self,other: Self) {
        self.r += other.r;
        self.i += other.i;
    }
}

// complex -= scalar
impl<T: SubAssign<T>> SubAssign<T> for Complex<T> {
    fn sub_assign(&mut self,other: T) {
        self.r -= other;
    }
}

// complex -= complex
impl<T: SubAssign<T>> SubAssign<Complex<T>> for Complex<T> {
    fn sub_assign(&mut self,other: Self) {
        self.r -= other.r;
        self.i -= other.i;
    }
}

// complex *= scalar
impl<T: MulAssign<T>> MulAssign<T> for Complex<T> {
    fn mul_assign(&mut self,other: T) {
        self.r *= other;
        self.i *= other;
    }
}

// complex *= complex
impl<T: Add<T,Output=T> + Sub<T,Output=T> + Mul<T,Output=T> + MulAssign<T>> MulAssign<Complex<T>> for Complex<T> {
    fn mul_assign(&mut self,other: Self) {
        let r = self.r * other.r - self.i * other.i;
        let i = self.r * other.i + self.i * other.r;
        self.r = r;
        self.i = i;
    }
}

// complex /= scalar
impl<T: DivAssign<T>> DivAssign<T> for Complex<T> {
    fn div_assign(&mut self,other: T) {
        self.r /= other;
        self.i /= other;
    }
}

// complex /= complex
impl<T: Add<T,Output=T> + Sub<T,Output=T> + Mul<T,Output=T> + Div<T,Output=T> + DivAssign<T>> DivAssign<Complex<T>> for Complex<T> {
    fn div_assign(&mut self,other: Self) {
        let f = other.r * other.r + other.i * other.i;
        let r = (self.r * other.r + self.i * other.i) / f;
        let i = (self.i * other.r - self.r * other.i) / f;
        self.r = r;
        self.i = i;
    }
}

// -complex
impl<T: Neg<Output=T>> Neg for Complex<T> {
    type Output = Complex<T>;
    fn neg(self) -> Complex<T> {
        Complex {
            r: -self.r,
            i: -self.i,
        }
    }
}

#[allow(non_camel_case_types)]
pub type f32c = Complex<f32>;
#[allow(non_camel_case_types)]
pub type f64c = Complex<f64>;
