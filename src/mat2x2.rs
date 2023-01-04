use {
    crate::*,
    std::{
        cmp::PartialEq,
        fmt::{
            Display,
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
pub struct Mat2x2<T> {
    pub x: Vec2<T>,
    pub y: Vec2<T>,
}

impl<T: Zero + Add<T,Output=T> + Mul<T,Output=T> + Div<T,Output=T> + Neg<Output=T> + PartialEq> Mat2x2<T> {
    pub fn transpose(self) -> Mat2x2<T> {
        Mat2x2 {
            x: Vec2 { x: self.x.x,y: self.y.x, },
            y: Vec2 { x: self.x.y,y: self.y.y, },
        }
    }

    pub fn determinant(self) -> T {
        // xx  yx
        // xy  yy
        let xx = self.x.x;
        let xy = self.x.y;
        let yx = self.y.x;
        let yy = self.y.y;

        // adjoint of first column
        let axx = yy;
        let axy = -yx;

        // determinant
        xx * axx + xy * axy
    }

    pub fn inverse(self) -> Self {
        // xx  yx
        // xy  yy
        let xx = self.x.x;
        let xy = self.x.y;
        let yx = self.y.x;
        let yy = self.y.y;

        // adjoint of first column
        let axx = yy;
        let axy = -yx;

        // determinant
        let det = xx * axx + xy * axy;
        if det == T::ZERO {
            return self;
        }

        // rest of adjoint
        let ayx = -xy;
        let ayy = xx;

        // transpose of adjoint divided by determinant
        Mat2x2 {
            x: Vec2 { x: axx,y: axy, },
            y: Vec2 { x: ayx,y: ayy, },
        } / det
    }
}

impl<T: Copy> From<[Vec2<T>; 2]> for Mat2x2<T> {
    fn from(array: [Vec2<T>; 2]) -> Self {
        Mat2x2 {
            x: array[0],
            y: array[1],
        }
    }
}

impl<T: Copy> From<&[Vec2<T>; 2]> for Mat2x2<T> {
    fn from(slice: &[Vec2<T>; 2]) -> Self {
        Mat2x2 {
            x: slice[0],
            y: slice[1],
        }
    }
}

impl<T: Copy> From<[T; 4]> for Mat2x2<T> {
    fn from(array: [T; 4]) -> Self {
        Mat2x2 {
            x: Vec2 { x: array[0],y: array[1], },
            y: Vec2 { x: array[2],y: array[3], },
        }
    }
}

impl<T: Copy> From<&[T; 4]> for Mat2x2<T> {
    fn from(slice: &[T; 4]) -> Self {
        Mat2x2 {
            x: Vec2 { x: slice[0],y: slice[1], },
            y: Vec2 { x: slice[2],y: slice[3], },
        }
    }
}

impl<T> PartialEq for Mat2x2<T> where Vec2<T>: PartialEq {
    fn eq(&self,other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

impl<T: Display> Display for Mat2x2<T> where Vec2<T>: Display {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"[{},{}]",self.x,self.y)
    }
}

// matrix + matrix
impl<T: Add<T,Output=T>> Add<Mat2x2<T>> for Mat2x2<T> where Vec2<T>: Add<Vec2<T>,Output=Vec2<T>> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Mat2x2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// matrix - matrix
impl<T: Sub<T,Output=T>> Sub<Mat2x2<T>> for Mat2x2<T> where Vec2<T>: Sub<Vec2<T>,Output=Vec2<T>> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Mat2x2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// scalar * matrix
macro_rules! scalar_mat2x2_mul {
    ($($t:ty)+) => {
        $(
            impl Mul<Mat2x2<$t>> for $t {
                type Output = Mat2x2<$t>;
                fn mul(self,other: Mat2x2<$t>) -> Mat2x2<$t> {
                    Mat2x2 {
                        x: self * other.x,
                        y: self * other.y,
                    }
                }
            }
        )+
    }
}

scalar_mat2x2_mul!(f32 f64);

// matrix * scalar
impl<T> Mul<T> for Mat2x2<T> where Vec2<T>: Mul<T,Output=Vec2<T>> {
    type Output = Self;
    fn mul(self,other: T) -> Self {
        Mat2x2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

// matrix * vector
impl<T: Copy + Mul<T,Output=T> + Add<T,Output=T>> Mul<Vec2<T>> for Mat2x2<T> {
    type Output = Vec2<T>;
    fn mul(self,other: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x.x * other.x + self.x.y * other.y,
            y: self.y.x * other.x + self.y.y * other.y,
        }
    }
}

// matrix * matrix
impl<T: Copy + Mul<T,Output=T> + Add<T,Output=T>> Mul<Mat2x2<T>> for Mat2x2<T> {
    type Output = Mat2x2<T>;
    fn mul(self,other: Mat2x2<T>) -> Mat2x2<T> {
        Mat2x2 {
            x: Vec2 {
                x: self.x.x * other.x.x + self.x.y * other.y.x,
                y: self.x.x * other.x.y + self.x.y * other.y.y,
            },
            y: Vec2 {
                x: self.y.x * other.x.x + self.y.y * other.y.x,
                y: self.y.x * other.x.y + self.y.y * other.y.y,
            },
        }
    }
}

// matrix / scalar
impl<T: Copy + Div<T,Output=T>> Div<T> for Mat2x2<T> {
    type Output = Mat2x2<T>;
    fn div(self,other: T) -> Mat2x2<T> {
        Mat2x2 {
            x: Vec2 { x: self.x.x / other,y: self.x.y / other, },
            y: Vec2 { x: self.y.x / other,y: self.y.y / other, }
        }
    }
}

// matrix += matrix
impl<T> AddAssign<Mat2x2<T>> for Mat2x2<T> where Vec2<T>: AddAssign<Vec2<T>> {
    fn add_assign(&mut self,other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

// matrix -= matrix
impl<T> SubAssign<Mat2x2<T>> for Mat2x2<T> where Vec2<T>: SubAssign<Vec2<T>> {
    fn sub_assign(&mut self,other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

// matrix *= scalar
impl<T: Copy + MulAssign<T>> MulAssign<T> for Mat2x2<T> {
    fn mul_assign(&mut self,other: T) {
        self.x.x *= other;
        self.x.y *= other;
        self.y.x *= other;
        self.y.y *= other;
    }
}

// matrix *= matrix
impl<T: Copy + Mul<T,Output=T> + Add<T,Output=T>> MulAssign<Mat2x2<T>> for Mat2x2<T> {
    fn mul_assign(&mut self,other: Mat2x2<T>) {
        let xx = self.x.x * other.x.x + self.x.y * other.y.x;
        let xy = self.x.x * other.x.y + self.x.y * other.y.y;
        self.x = Vec2 { x: xx,y: xy, };
        let yx = self.y.x * other.x.x + self.y.y * other.y.x;
        let yy = self.y.x * other.x.y + self.y.y * other.y.y;
        self.y = Vec2 { x: yx,y: yy, };
    }
}

// matrix /= scalar
impl<T: Copy + DivAssign<T>> DivAssign<T> for Mat2x2<T> {
    fn div_assign(&mut self,other: T) {
        self.x.x /= other;
        self.x.y /= other;
        self.y.x /= other;
        self.y.y /= other;
    }
}

// -matrix
impl<T: Neg<Output=T>> Neg for Mat2x2<T> {
    type Output = Mat2x2<T>;
    fn neg(self) -> Mat2x2<T> {
        Mat2x2 {
            x: Vec2 { x: -self.x.x,y: -self.x.y, },
            y: Vec2 { x: -self.y.x,y: -self.y.y, },
        }
    }
}
