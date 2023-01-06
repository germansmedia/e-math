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
    },
};

fn _gcd<UT: Copy + Zero + PartialEq + Rem<Output=UT>>(mut a: UT,mut b: UT) -> UT {
    while b != UT::ZERO {
        let c = b;
        b = a % b;
        a = c;
    }
    a
}

/// Rational template.
/// 
/// A rational number has a numerator and denominator. This is useful for cases where exact calculations are needed that
/// cannot be handled by floating point numbers.
/// 
/// Rational numbers are also signed or unsigned integer numbers, so they implement [`Signed`] or [`Unsigned`].
#[derive(Copy,Clone,Debug)]
pub struct Rational<T,UT> {
    n: T,  // negative, 0 or positive
    d: UT,  // never 0 or negative
}

macro_rules! rational_impl {
    ($(($t:ty,$ut:ty))*) => ($(

        impl Rational<$t,$ut> {

            fn _reduce(&mut self) {
                let gcd = _gcd(self.n as $ut,self.d);
                self.n /= gcd as $t;
                self.d /= gcd;
            }
        
            pub fn inverse(&self) -> Self {
                Rational {
                    n: self.d as $t,
                    d: self.n as $ut,
                }
            }
        }
        
        impl Display for Rational<$t,$ut> {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"{}/{}",self.n,self.d)
            }
        }

        // scalar == rational
        impl PartialEq<Rational<$t,$ut>> for $t {
            fn eq(&self,other: &Rational<$t,$ut>) -> bool {
                (other.d == 1) && (self == &other.n)
            }
        }
        
        // rational == scalar
        impl PartialEq<$t> for Rational<$t,$ut> {
            fn eq(&self,other: &$t) -> bool {
                (self.d == 1) && (self.n == *other)
            }
        }

        // rational == rational
        impl PartialEq<Rational<$t,$ut>> for Rational<$t,$ut> {
            fn eq(&self,other: &Self) -> bool {
                (self.n == other.n) &&
                (self.d == other.d)
            }
        }

        // scalar ? rational
        impl PartialOrd<Rational<$t,$ut>> for $t {
            fn partial_cmp(&self, other: &Rational<$t,$ut>) -> Option<Ordering> {
                (self * (other.d as $t)).partial_cmp(&other.n)
            }
        }

        // rational ? scalar
        impl PartialOrd<$t> for Rational<$t,$ut> {
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                self.n.partial_cmp(&(other * (self.d as $t)))
            }
        }

        // rational ? rational
        impl PartialOrd<Rational<$t,$ut>> for Rational<$t,$ut> {
            fn partial_cmp(&self, other: &Rational<$t,$ut>) -> Option<Ordering> {
                (self.n * (other.d as $t)).partial_cmp(&(other.n * (self.d as $t)))
            }
        }

        // scalar + rational
        impl Add<Rational<$t,$ut>> for $t {
            type Output = Rational<$t,$ut>;
            fn add(self,other: Rational<$t,$ut>) -> Self::Output {
                let mut result = Rational {
                    n: self * other.d as $t + other.n,
                    d: other.d,
                };
                result._reduce();
                result
            }
        }

        // rational + scalar
        impl Add<$t> for Rational<$t,$ut> {
            type Output = Self;
            fn add(self,other: $t) -> Self::Output {
                let mut result = Rational {
                    n: self.n + (self.d as $t) * other,
                    d: self.d,
                };
                result._reduce();
                result
            }
        }

        // rational + rational
        impl Add<Rational<$t,$ut>> for Rational<$t,$ut> {
            type Output = Self;
            fn add(self,other: Self) -> Self::Output {
                let mut result = Rational {
                    n: self.n * (other.d as $t) + other.n * (self.d as $t),
                    d: self.d * other.d,
                };
                result._reduce();
                result
            }
        }

        // rational += scalar
        impl AddAssign<$t> for Rational<$t,$ut> {
            fn add_assign(&mut self,other: $t) {
                self.n += other * (self.d as $t);
                self._reduce();
            }
        }

        // rational += rational
        impl AddAssign<Rational<$t,$ut>> for Rational<$t,$ut> {
            fn add_assign(&mut self,other: Self) {
                self.n *= other.d as $t;
                self.n += other.n * (self.d as $t);
                self.d *= other.d;
                self._reduce();
            }
        }

        // scalar - rational
        impl Sub<Rational<$t,$ut>> for $t {
            type Output = Rational<$t,$ut>;
            fn sub(self,other: Rational<$t,$ut>) -> Self::Output {
                let mut result = Rational {
                    n: self * other.d as $t - other.n,
                    d: other.d,
                };
                result._reduce();
                result
            }
        }
        
        // rational - scalar
        impl Sub<$t> for Rational<$t,$ut> {
            type Output = Self;
            fn sub(self,other: $t) -> Self::Output {
                let mut result = Rational {
                    n: self.n - (self.d as $t) * other,
                    d: self.d,
                };
                result._reduce();
                result
            }
        }

        // rational - rational
        impl Sub<Rational<$t,$ut>> for Rational<$t,$ut> {
            type Output = Self;
            fn sub(self,other: Rational<$t,$ut>) -> Self::Output {
                let mut result = Rational {
                    n: self.n * (other.d as $t) - other.n * (self.d as $t),
                    d: self.d * other.d,
                };
                result._reduce();
                result
            }
        }

        // rational -= scalar
        impl SubAssign<$t> for Rational<$t,$ut> {
            fn sub_assign(&mut self,other: $t) {
                self.n -= other * (self.d as $t);
                self._reduce();
            }
        }

        // rational -= rational
        impl SubAssign<Rational<$t,$ut>> for Rational<$t,$ut> {
            fn sub_assign(&mut self,other: Self) {
                self.n *= other.d as $t;
                self.n -= other.n * (self.d as $t);
                self.d *= other.d;
                self._reduce();
            }
        }

        // scalar * rational
        impl Mul<Rational<$t,$ut>> for $t {
            type Output = Rational<$t,$ut>;
            fn mul(self,other: Rational<$t,$ut>) -> Rational<$t,$ut> {
                let mut result = Rational {
                    n: self * other.n,
                    d: other.d,
                };
                result._reduce();
                result
            }
        }

        // rational * scalar
        impl Mul<$t> for Rational<$t,$ut> {
            type Output = Self;
            fn mul(self,other: $t) -> Self::Output {
                let mut result = Rational {
                    n: self.n * other,
                    d: self.d,
                };
                result._reduce();
                result
            }
        }

        // rational * rational
        impl Mul<Rational<$t,$ut>> for Rational<$t,$ut> {
            type Output = Self;
            fn mul(self,other: Self) -> Self::Output {
                let mut result = Rational {
                    n: self.n * other.n,
                    d: self.d * other.d,
                };
                result._reduce();
                result
            }
        }

        // rational *= scalar
        impl MulAssign<$t> for Rational<$t,$ut> {
            fn mul_assign(&mut self,other: $t) {
                self.n *= other;
                self._reduce();
            }
        }

        // rational *= rational
        impl MulAssign<Rational<$t,$ut>> for Rational<$t,$ut> {
            fn mul_assign(&mut self,other: Self) {
                self.n *= other.n;
                self.d *= other.d;
                self._reduce();
            }
        }

        // rational % scalar
        impl Rem<$t> for Rational<$t,$ut> {
            type Output = Rational<$t,$ut>;
            fn rem(self,other: $t) -> Self::Output {
                Rational {
                    n: self.n % other,
                    d: self.d,
                }
            }
        }

        // rational % rational
        impl Rem<Rational<$t,$ut>> for Rational<$t,$ut> {
            type Output = Rational<$t,$ut>;
            fn rem(self,other: Rational<$t,$ut>) -> Rational<$t,$ut> {
                // TODO
                self
            }
        }

        // rational %= scalar
        impl RemAssign<$t> for Rational<$t,$ut> {
            fn rem_assign(&mut self,other: $t) {
                self.n %= other;
            }
        }

        // rational %= rational
        impl RemAssign<Rational<$t,$ut>> for Rational<$t,$ut> {
            fn rem_assign(&mut self,other: Rational<$t,$ut>) {
                // TODO
            }
        }

        impl Unsigned for Rational<$t,$ut> {

            const MIN: Self = Rational { n: <$t>::MIN,d: 1, };

            const MAX: Self = Rational { n: <$t>::MAX,d: 1, };

            const BITS: u32 = <$t>::BITS + <$ut>::BITS;

            fn pow(self,exp: u32) -> Self {
                let mut result = Rational {
                    n: self.n.pow(exp),
                    d: self.d.pow(exp),
                };
                result._reduce();
                result
            }

            fn div_euclid(self,rhs: Self) -> Self {
                self / rhs
            }

            fn rem_euclid(self,rhs: Self) -> Self {
                self % rhs
            }
        }        
    )*)
}

rational_impl! { (usize,usize) (u8,u8) (u16,u16) (u32,u32) (u64,u64) (u128,u128) (isize,usize) (i8,u8) (i16,u16) (i32,u32) (i64,u64) (i128,u128) }

macro_rules! rational_impl_div_unsigned {
    ($(($t:ty,$ut:ty))*) => ($(

        // scalar / rational
        impl Div<Rational<$t,$ut>> for $t {
            type Output = Rational<$t,$ut>;
            fn div(self,other: Rational<$t,$ut>) -> Self::Output {
                let mut result = Rational {
                    n: self * other.d as $t,
                    d: other.n as $ut,
                };
                result._reduce();
                result
            }
        }

        // rational / scalar
        impl Div<$t> for Rational<$t,$ut> {
            type Output = Rational<$t,$ut>;
            fn div(self,other: $t) -> Self::Output {
                let mut result = Rational {
                    n: self.n,
                    d: self.d * (other as $ut),
                };
                result._reduce();
                result
            }
        }

        // rational / rational
        impl Div<Rational<$t,$ut>> for Rational<$t,$ut> {
            type Output = Rational<$t,$ut>;
            fn div(self,other: Rational<$t,$ut>) -> Rational<$t,$ut> {
                let mut result = Rational {
                    n: self.n * (other.d as $t),
                    d: self.d * (other.n as $ut),
                };
                result._reduce();
                result
            }
        }

        // rational /= scalar
        impl DivAssign<$t> for Rational<$t,$ut> {
            fn div_assign(&mut self,other: $t) {
                self.d *= other as $ut;
                self._reduce();
            }
        }

        // rational /= rational
        impl DivAssign<Rational<$t,$ut>> for Rational<$t,$ut> {
            fn div_assign(&mut self,other: Rational<$t,$ut>) {
                self.n *= other.d as $t;
                self.d *= other.n as $ut;
                self._reduce();
            }
        }
    )*)
}

rational_impl_div_unsigned! { (usize,usize) (u8,u8) (u16,u16) (u32,u32) (u64,u64) (u128,u128)  }

macro_rules! rational_impl_div_signed {
    ($(($t:ty,$ut:ty))*) => ($(

        // scalar / rational
        impl Div<Rational<$t,$ut>> for $t {
            type Output = Rational<$t,$ut>;
            fn div(self,other: Rational<$t,$ut>) -> Self::Output {
                let mut result = if other.n < 0 {
                    Rational {
                        n: -(self * other.d as $t),
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

        // rational / scalar
        impl Div<$t> for Rational<$t,$ut> {
            type Output = Rational<$t,$ut>;
            fn div(self,other: $t) -> Self::Output {
                let mut result = if other < 0 {
                    Rational {
                        n: -self.n,
                        d: self.d * (-other as $ut),
                    }
                }
                else {
                    Rational {
                        n: self.n,
                        d: self.d * (other as $ut),
                    }
                };
                result._reduce();
                result
            }
        }

        // rational / rational
        impl Div<Rational<$t,$ut>> for Rational<$t,$ut> {
            type Output = Rational<$t,$ut>;
            fn div(self,other: Rational<$t,$ut>) -> Rational<$t,$ut> {
                let mut result = if other.n < 0 {
                    Rational {
                        n: -self.n * (other.d as $t),
                        d: self.d * (-other.n as $ut),
                    }
                }
                else {
                    Rational {
                        n: self.n * (other.d as $t),
                        d: self.d * (other.n as $ut),
                    }
                };
                result._reduce();
                result
            }
        }

        // rational /= scalar
        impl DivAssign<$t> for Rational<$t,$ut> {
            fn div_assign(&mut self,other: $t) {
                if other < 0 {
                    self.n = -self.n;
                    self.d *= -other as $ut;
                }
                else {
                    self.d *= other as $ut;
                }
                self._reduce();
            }
        }

        // rational /= rational
        impl DivAssign<Rational<$t,$ut>> for Rational<$t,$ut> {
            fn div_assign(&mut self,other: Rational<$t,$ut>) {
                if other.n < 0 {
                    self.n *= -(other.d as $t);
                    self.d *= -other.n as $ut;    
                }
                else {
                    self.n *= other.d as $t;
                    self.d *= other.n as $ut;
                }
                self._reduce();
            }
        }

        // -rational
        impl Neg for Rational<$t,$ut> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Rational {
                    n: -self.n,
                    d: self.d,
                }
            }
        }
    )*)
}

rational_impl_div_signed! { (isize,usize) (i8,u8) (i16,u16) (i32,u32) (i64,u64) (i128,u128) }