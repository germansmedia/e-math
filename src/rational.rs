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

pub fn _gcd<UT: Zero + PartialEq + Rem<Output=UT>>(a: UT,b: UT) -> UT {
    let mut a = a;
    let mut b = b;
    while b != UT::ZERO {
        let c = b;
        b = a % b;
        a = c;
    }
    a
}

#[derive(Copy,Clone,Debug)]
pub struct Rational<T,UT> {
    n: T,  // negative, 0 or positive
    d: UT,  // never 0 or negative
}

impl<T: Display,UT: Display> Display for Rational<T,UT> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"{}/{}",self.n,self.d)
    }
}

impl<T: Zero + PartialEq + Rem<T> + DivAssign,UT: DivAssign> Rational<T,UT> {

    pub fn _reduce(&mut self) {
        let gcd = _gcd(self.n,self.d);
        self.n /= gcd as T;
        self.d /= gcd;
    }

    pub fn inverse(&self) -> Self {
        Rational {
            n: self.d,
            d: self.n,
        }
    }
}

// rational == scalar
impl<T,UT> PartialEq<T> for Rational<T,UT> {
    fn eq(&self,other: &T) -> bool {
        self.n * self.d as T == other
    }
}

// rational == rational
impl<T,UT> PartialEq<Rational<T,UT>> for Rational<T,UT> {
    fn eq(&self,other: &Self) -> bool {
        (self.n == other.n) &&
        (Self.d == other.d)
    }
}

// rational ? scalar
impl<T,UT> PartialOrd<T> for Rational<T,UT> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.n.partial_cmp(&other * self.d as T)
    }
}

// rational ? rational
impl<T,UT> PartialOrd<Rational<T,UT>> for Rational<T,UT> {
    fn partial_cmp(&self, other: &Rational<T,UT>) -> Option<Ordering> {
        (self * other.d as T).partial_cmp(other.n * self.d as T)
    }
}


macro_rules! scalar_rational {
    ($(($t:ty,$ut:ty))*) => ($(

        // scalar ? rational
        //impl PartialOrd<Rational<$t,$ut>> for $t {
        //    fn partial_cmp(&self, other: &Rational<$t,$ut>) -> Option<Ordering> {
        //        (self * other.d as $t).partial_cmp(&other.n)
        //    }
        //}

        // scalar + rational
        impl Add<Rational<$t,$ut>> for $t {
            type Output = Rational<$t,$ut>;
            fn add(self,other: Rational<$t,$ut>) -> Rational<$t,$ut> {
                let mut result = Rational {
                    n: self * other.d as $t + other.n,
                    d: other.d,
                };
                result._reduce();
                result
            }
        }

        // scalar - rational
        impl Sub<Rational<$t,$ut>> for $t {
            type Output = Rational<$t,$ut>;
            fn sub(self,other: Rational<$t,$ut>) -> Rational<$t,$ut> {
                let mut result = Rational {
                    n: self * other.d as $t - other.n,
                    d: other.d,
                };
                result._reduce();
                result
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

        // scalar / rational
        impl Div<Rational<$t,$ut>> for $t {
            type Output = Rational<$t,$ut>;
            fn div(self,other: Rational<$t,$ut>) -> Rational<$t,$ut> {
                let mut result = if other.n < 0 {
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

    )*)
}

scalar_rational! { (usize,usize) (u8,u8) (u16,u16) (u32,u32) (u64,u64) (u128,u128) (isize,usize) (i8,u8) (i16,u16) (i32,u32) (i64,u64) (i128,u128) }

// rational + scalar
impl<T,UT> Add<T> for Rational<T,UT> {
    fn add(self,other: T) -> Self {
        let mut result = Rational {
            n: self.n + self.d as T * other,
            d: self.d,
        };
        result._reduce();
        result
    }
}

// rational + rational
impl<T,UT> Add<Rational<T,UT>> for Rational<T,UT> {
    fn add(self,other: Self) -> Self {
        let mut result = Rational {
            n: self.n * other.d as T + other.n * self.d as T,
            d: self.d * other.d,
        };
        result._reduce();
        result
    }
}

// rational += scalar
impl<T,UT> AddAssign<T> for Rational<T,UT> {
    fn add_assign(&mut self,other: T) {
        self.n += other * self.d as T;
        self._reduce();
    }
}

// rational += rational
impl<T,UT> AddAssign<Rational<T,UT>> for Rational<T,UT> {
    fn add_assign(&mut self,other: Self) {
        self.n *= other.d;
        self.n += other.n * self.d;
        self.d *= other.d;
        self._reduce();
    }
}

// rational - scalar
impl<T,UT> Sub<T> for Rational<T,UT> {
    fn sub(self,other: T) -> Self {
        let mut result = Rational {
            n: self.n - self.d as T * other,
            d: self.d,
        };
        result._reduce();
        result
    }
}

// rational - rational
impl<T,UT> Sub<Rational<T,UT>> for Rational<T,UT> {
    fn sub(self,other: Rational<T,UT>) -> Self {
        let mut result = Rational {
            n: self.n * other.d as T - other.n * self.d as T,
            d: self.d * other.d,
        };
        result._reduce();
        result
    }
}

// rational -= scalar
impl<T,UT> SubAssign<T> for Rational<T,UT> {
    fn sub_assign(&mut self,other: T) {
        self.n -= other * self.d as T;
        self._reduce();
    }
}

// rational -= rational
impl<T,UT> SubAssign<Rational<T,UT>> for Rational<T,UT> {
    fn sub_assign(&mut self,other: Self) {
        self.n *= other.d as T;
        self.n -= other.n * self.d as T;
        self.d *= other.d;
        self._reduce();
    }
}

// rational * scalar
impl<T,UT> Mul<T> for Rational<T,UT> {
    fn mul(self,other: T) -> Self {
        let mut result = Rational {
            n: self.n * other,
            d: self.d,
        };
        result._reduce();
        result
    }
}

// rational * rational
impl<T,UT> Mul<Rational<T,UT>> for Rational<T,UT> {
    fn mul(self,other: Self) -> Self {
        let mut result = Rational {
            n: self.n * other.n,
            d: self.d * other.d,
        };
        result._reduce();
        result
    }
}

// rational *= scalar
impl<T,UT> MulAssign<T> for Rational<T,UT> {
    fn mul_assign(&mut self,other: T) {
        self.n *= other;
        self._reduce();
    }
}

// rational *= rational
impl<T,UT> MulAssign<Rational<T,UT>> for Rational<T,UT> {
    fn mul_assign(&mut self,other: Self) {
        self.n *= other.n;
        self.d *= other.d;
        self._reduce();
    }
}

// rational / scalar
impl<T,UT> Div<T> for Rational<T,UT> {
    fn div(self,other: T) -> Rational<T,UT> {
        let mut result = if other < 0 {
            Rational {
                n: -self.n,
                d: self.d * -other as UT,
            }
        }
        else {
            Rational {
                n: self.n,
                d: self.d * other as UT,
            }
        };
        result._reduce();
        result
    }
}

// rational / rational
impl<T,UT> Div<Rational<T,UT>> for Rational<T,UT> {
    type Output = Rational<T,UT>;
    fn div(self,other: Rational<T,UT>) -> Rational<T,UT> {
        let mut result = if other.n < 0 {
            Rational {
                n: -self.n * other.d as T,
                d: self.d * -other.n as UT,
            }
        }
        else {
            Rational {
                n: self.n * other.d as T,
                d: self.d * other.n as UT,
            }
        };
        result._reduce();
        result
    }
}

// rational /= scalar
impl<T,UT> DivAssign<T> for Rational<T,UT> {
    fn div_assign(&mut self,other: T) {
        if other < 0 {
            self.n = -self.n;
            self.d *= -other as UT;
        }
        else {
            self.d *= other as UT;
        }
        self._reduce();
    }
}

// rational /= rational
impl<T,UT> DivAssign<Rational<T,UT>> for Rational<T,UT> {
    fn div_assign(&mut self,other: Rational<T,UT>) {
        if other.n < 0 {
            self.n *= -(other.d as T);
            self.d *= -other.n as UT;    
        }
        else {
            self.n *= other.d as T;
            self.d *= other.n as UT;
        }
        self._reduce();
    }
}
