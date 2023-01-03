use {
    crate::*,
    std::{
        cmp::PartialEq,
        cmp::PartialOrd,
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
pub struct Rational<T,UT> {
    n: T,  // negative, 0 or positive
    d: UT,  // never 0 or negative
}

pub fn _gcd<UT: PartialEq>(n: UT,d: UT) -> UT {
    if (n == 0) || (d == 0) {
        return 1;
    }
    if (n == d) {
        return d;
    }
    else if (n < d) {
        return _gcd(n,d - n);
    }
    else {
        return _gcd(n - d,d);
    }
}

impl<T,UT> Rational<T,UT> {
    pub fn new(n: T,d: UT) -> Rational<T,UT> {
        Rational { n,d, }
    }

    pub fn _reduce(&mut self) {
        let gcd = _gcd(if (self.n < 0) { -self.n as UT } else { self.n as UT },self.d)
        self.n /= gcd as T;
        self.d /= gcd;
    }
}

// rational == rational
impl<T: PartialEq,UT: PartialEq> PartialEq for Rational<T,UT> {
    fn eq(&self,other: &Self) -> bool {
        (self.n == other.n) &&
        (Self.d == other.d)
    }
}

// TODO: PartialOrd

impl<T: Zero,UT> Zero for Rational<T,UT> { const ZERO: Self = Rational { n: T::ZERO,d: 1, }; }

impl<T: Zero + Display + PartialOrd> Display for Rational<T,UT> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"{}/{}",self.n,self.d)
    }
}

macro_rules! scalar_rational {
    ($t:ty,$ut:ty) => {

        // scalar + rational
        impl Add<Rational<$t,$ut>> for $t {
            fn add(self,other: Rational<$t,$ut>) -> Rational<$t,$ut> {
                let result = Rational {
                    n: self * other.d + other.n,
                    d: other.d,
                };
                result._reduce();
                result
            }
        }

        // scalar - rational
        impl Sub<Rational<$t,$ut>> for $t {
            fn sub(self,other: Rational<$t,$ut>) -> Rational<$t,$ut> {
                let result = Rational {
                    n: self * other.d - other.n,
                    d: other.d,
                };
                result._reduce();
                result
            }
        }

        // scalar * rational
        impl Mul<Rational<$t,$ut>> for $t {
            fn mul(self,other: Rational<$t,$ut>) -> Rational<$t,$ut> {
                let result = Rational {
                    n: self * other.n,
                    d: other.d,
                };
                result._reduce();
                result
            }
        }

        // scalar / rational
        impl Div<Rational<$t,$ut>> for $t {
            fn mul(self,other: Rational<$t,$ut>) -> Rational<$t,$ut> {
                let result = if (other.n < 0) {
                    Rational {
                        n: -self * other.d as $t,
                        d: -other.n as $ut,
                    }
                }
                else {
                    Rational {
                        n: self * other.d as $t,
                        d: other.n as $ut,
                    }
                };
                result._reduce();
                result
            }
        }
    }
}

scalar_rational!(i8,u8);
scalar_rational!(i16,u16);
scalar_rational!(i32,u32);
scalar_rational!(i64,u64);
scalar_rational!(i128,u128);
scalar_rational!(isize,usize);
scalar_rational!(u8,u8);
scalar_rational!(u16,u16);
scalar_rational!(u32,u32);
scalar_rational!(u64,u64);
scalar_rational!(u128,u128);
scalar_rational!(usize,usize);

// rational + scalar
impl<T: Add<T,Output=T>,UT> Add<T> for Rational<T,UT> {
    type Output = Self;
    fn add(self,other: T) -> Self {
        let result = Rational {
            n: self.n + other.n * self.d,
            d: self.d,
        };
        result._reduce();
        result
    }
}

// rational + rational
impl<T: Add<T,Output=T>,UT> Add<Rational<T,UT>> for Rational<T,UT> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        // TODO: synchronize denominators

    }
}

// TODO: ...

#[allow(non_camel_case_types)]
pub type i8r = Rational<i8,u8>;
#[allow(non_camel_case_types)]
pub type i16r = Rational<i16,u16>;
#[allow(non_camel_case_types)]
pub type i32r = Rational<i32,u32>;
#[allow(non_camel_case_types)]
pub type i64r = Rational<i64,u64>;
#[allow(non_camel_case_types)]
pub type i128r = Rational<i128,u128>;
#[allow(non_camel_case_types)]
pub type isizer = Rational<isize,usize>;
#[allow(non_camel_case_types)]
pub type u8r = Rational<u8,u8>;
#[allow(non_camel_case_types)]
pub type u16r = Rational<u16,u16>;
#[allow(non_camel_case_types)]
pub type u32r = Rational<u32,u32>;
#[allow(non_camel_case_types)]
pub type u64r = Rational<u64,u64>;
#[allow(non_camel_case_types)]
pub type u128r = Rational<u128,u128>;
#[allow(non_camel_case_types)]
pub type usizer = Rational<usize,usize>;
