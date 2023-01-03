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
pub struct Quaternion<T> {
    pub r: T,
    pub i: T,
    pub j: T,
    pub k: T,
}

impl<T: Add<T,Output=T> + Mul<T,Output=T> + Div<T,Output=T> + Neg<Output=T>> Quaternion<T> {
    pub fn conj(&self) -> Quaternion<T> {
        Quaternion {
            r: self.r,
            i: -self.i,
            j: -self.j,
            k: -self.k,
        }
    }

    pub fn norm(&self) -> T {
        (self.r * self.r + self.i * self.i + self.j * self.j + self.k * self.k).sqrt()
    }

    pub fn recip(&self) -> Quaternion<T> {
        let f = self.r * self.r + self.i * self.i + self.j * self.j + self.k * self.k;
        Quaternion {
            r: self.r / f,
            i: -self.i / f,
            j: -self.j / f,
            k: -self.k / f,
        }
    }
}

impl<T: Zero> Zero for Quaternion<T> { const ZERO: Quaternion<T> = Quaternion { r: T::ZERO,i: T::ZERO, j: T::ZERO, k: T::ZERO, }; }

// quaternion == quaternion
impl<T: PartialEq> PartialEq for Quaternion<T> {
    fn eq(&self,other: &Self) -> bool {
        (self.r == other.r) &&
        (self.i == other.i) &&
        (self.j == other.j) &&
        (self.k == other.k)
    }
}

impl<T: Display + Zero + PartialOrd> Display for Quaternion<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        let si = if self.i < T::ZERO {
            format!("{}i",self.i)
        } else {
            format!("+{}i",self.i)
        };
        let sj = if self.j < T::ZERO {
            format!("{}j",self.j)
        } else {
            format!("+{}j",self.j)
        };
        let sk = if self.k < T::ZERO {
            format!("{}k",self.k)
        } else {
            format!("+{}k",self.k)
        };
        write!(f,"{}{}{}{}",self.r,si,sj,sk)
    }
}

macro_rules! scalar_quaternion {
    ($t:ty) => {

        // scalar * quaternion
        impl Mul<Quaternion<$t>> for $t {
            type Output = Quaternion<$t>;
            fn mul(self,other: Quaternion<$t>) -> Quaternion<$t> {
                Quaternion {
                    r: self * other.r,
                    i: self * other.i,
                    j: self * other.j,
                    k: self * other.k,
                }
            }
        }

        // TODO: Div
    }
}

scalar_quaternion!(f32);
scalar_quaternion!(f64);

// quaternion + quaternion
impl<T: Add<T,Output=T>> Add<Quaternion<T>> for Quaternion<T> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Quaternion {
            r: self.r + other.r,
            i: self.i + other.i,
            j: self.j + other.j,
            k: self.k + other.k,
        }
    }
}

// quaternion - quaternion
impl<T: Sub<T,Output=T>> Sub<Quaternion<T>> for Quaternion<T> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Quaternion {
            r: self.r - other.r,
            i: self.i - other.i,
            j: self.j - other.j,
            k: self.k - other.k,
        }
    }
}

// quaternion * scalar
impl<T: Mul<T,Output=T>> Mul<T> for Quaternion<T> {
    type Output = Self;
    fn mul(self,other: T) -> Self {
        Quaternion {
            r: self.r * other,
            i: self.i * other,
            j: self.j * other,
            k: self.k * other,
        }
    }
}

// quaternion * vec3 (apply quaternion to vec3)
impl<T: Add<T,Output=T> + Sub<T,Output=T> + Mul<T,Output=T>> Mul<Vec3<T>> for Quaternion<T> {
    type Output = Vec3<T>;
    fn mul(self,other: Vec3<T>) -> Vec3<T> {
        let rr = self.r * self.r;
        let ri = self.r * self.i;
        let rj = self.r * self.j;
        let rk = self.r * self.k;
        let ii = self.i * self.i;
        let ij = self.i * self.j;
        let ik = self.i * self.k;
        let jj = self.j * self.j;
        let jk = self.j * self.k;
        let kk = self.k * self.k;
        let ijprk = ij + rk;
        let ijprk2 = ijprk + ijprk;
        let ijmrk = ij - rk;
        let ijmrk2 = ijmrk + ijmrk;
        let jkpri = jk + ri;
        let jkpri2 = jkpri + jkpri;
        let jkmri = jk - ri;
        let jkmri2 = jkmri + jkmri;
        let ikprj = ik + rj;
        let ikprj2 = ikprj + ikprj;
        let ikmrj = ik - rj;
        let ikmrj2 = ikmrj + ikmrj;
        Vec3 {
            x: (rr + ii - jj - kk) * other.x + ijmrk2 * other.y + ikprj2 * other.z,
            y: (rr - ii + jj - kk) * other.y + jkmri2 * other.z + ijprk2 * other.x,
            z: (rr - ii - jj + kk) * other.z + ikmrj2 * other.x + jkpri2 * other.y,
        }
    }
}

// quaternion * quaternion
impl<T: Add<T,Output=T> + Sub<T,Output=T> + Mul<T,Output=T>> Mul<Quaternion<T>> for Quaternion<T> {
    type Output = Self;
    fn mul(self,other: Self) -> Self {
        Quaternion {
            r: self.r * other.r - self.i * other.i - self.j * other.j - self.k * other.k,
            i: self.r * other.i + self.i * other.r + self.j * other.k - self.k * other.j,
            j: self.r * other.j - self.i * other.k + self.j * other.r + self.k * other.i,
            k: self.r * other.k + self.i * other.j - self.j * other.i + self.k * other.r,
        }
    }
}

// quaternion / scalar
impl<T: Div<T,Output=T>> Div<T> for Quaternion<T> {
    type Output = Self;
    fn div(self,other: T) -> Self {
        Quaternion {
            r: self.r / other,
            i: self.i / other,
            j: self.j / other,
            k: self.k / other,
        }
    }
}

// quaternion / quaternion
impl<T: Add<T,Output=T> + Sub<T,Output=T> + Mul<T,Output=T> + Div<T,Output=T> + Neg<Output=T>> Div<Quaternion<T>> for Quaternion<T> {
    type Output = Self;
    fn div(self,other: Self) -> Self {
        self * other.conj()  // TODO: reqlly should be other.inv()
    }
}

#[allow(non_camel_case_types)]
pub type f32q = Quaternion<f32>;
#[allow(non_camel_case_types)]
pub type f64q = Quaternion<f64>;
