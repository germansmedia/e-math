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
pub struct Mat3x3<T> {
    pub x: Vec3<T>,
    pub y: Vec3<T>,
    pub z: Vec3<T>,
}

/*
impl<T: Zero + PartialEq + Add<T,Output=T> + Sub<T,Output=T> + Mul<T,Output=T> + Div<T,Output=T> + Neg<Output=T>> Mat3x3<T> {
    pub fn transpose(self) -> Mat3x3<T> {
        Mat3x3 {
            x: Vec3 { x: self.x.x,y: self.y.x,z: self.z.x, },
            y: Vec3 { x: self.x.y,y: self.y.y,z: self.z.y, },
            z: Vec3 { x: self.x.z,y: self.y.z,z: self.z.z, },
        }
    }

    pub fn determinant(self) -> T {

        // xx  yx  zx
        // xy  yy  zy
        // xz  yz  zz
        let xx = self.x.x;
        let xy = self.x.y;
        let xz = self.x.z;
        let yx = self.y.x;
        let yy = self.y.y;
        let yz = self.y.z;
        let zx = self.z.x;
        let zy = self.z.y;
        let zz = self.z.z;

        // adjoint of first column

        // yy  zy
        // yz  zz
        let axx = yy * zz - zy * yz;
        
        // yz  zz
        // yx  zx
        let axy = -(yz * zx - zz * yx);
        
        // yx  zx
        // yy  zy
        let axz = yx * zy - zx * yy;

        // dot
        xx * axx + xy * axy + xz * axz
    }

    pub fn inverse(self) -> Self {
        // xx  yx  zx
        // xy  yy  zy
        // xz  yz  zz
        let xx = self.x.x;
        let xy = self.x.y;
        let xz = self.x.z;
        let yx = self.y.x;
        let yy = self.y.y;
        let yz = self.y.z;
        let zx = self.z.x;
        let zy = self.z.y;
        let zz = self.z.z;

        // adjoint of first column
        let axx = yy * zz - zy * yz;
        let axy = -(yz * zx - zz * yx);
        let axz = yx * zy - zx * yy;

        // determinant
        let det = xx * axx + xy * axy + xz * axz;
        if det == T::ZERO {
            return self;
        }

        // rest of adjoint
        let ayx = -(zy * xz - xy * zz);
        let ayy = zz * xx - xz * zx;
        let ayz = -(zx * xy - xx * zy);
        let azx = xy * yz - yy * xz;
        let azy = -(xz * yx - yz * xx);
        let azz = xx * yy - yx * xy;
        
        // transpose of adjoint divided by determinant
        Mat3x3 {
            x: Vec3 { x: axx,y: axy,z: axz, },
            y: Vec3 { x: ayx,y: ayy,z: ayz, },
            z: Vec3 { x: azx,y: azy,z: azz, },
        } / det
    }
}

impl<T: Copy> From<[Vec3<T>; 3]> for Mat3x3<T> {
    fn from(array: [Vec3<T>; 3]) -> Self {
        Mat3x3 {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

impl<T: Copy> From<&[Vec3<T>; 3]> for Mat3x3<T> {
    fn from(slice: &[Vec3<T>; 3]) -> Self {
        Mat3x3 {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }
}

impl<T: Copy> From<[T; 9]> for Mat3x3<T> {
    fn from(array: [T; 9]) -> Self {
        Mat3x3 {
            x: Vec3 { x: array[0],y: array[1],z: array[2], },
            y: Vec3 { x: array[3],y: array[4],z: array[5], },
            z: Vec3 { x: array[6],y: array[7],z: array[8], },
        }
    }
}

impl<T: Copy> From<&[T; 9]> for Mat3x3<T> {
    fn from(slice: &[T; 9]) -> Self {
        Mat3x3 {
            x: Vec3 { x: slice[0],y: slice[1],z: slice[2], },
            y: Vec3 { x: slice[3],y: slice[4],z: slice[5], },
            z: Vec3 { x: slice[6],y: slice[7],z: slice[8], },
        }
    }
}

impl<T> PartialEq for Mat3x3<T> where Vec3<T>: PartialEq {
    fn eq(&self,other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y) && (self.z == other.z)
    }
}

impl<T> Display for Mat3x3<T> where Vec3<T>: Display {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"[{},{},{}]",self.x,self.y,self.z)
    }
}

// matrix + matrix
impl<T> Add<Mat3x3<T>> for Mat3x3<T> where Vec3<T>: Add<Vec3<T>,Output=Vec3<T>> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Mat3x3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// matrix += matrix
impl<T> AddAssign<Mat3x3<T>> for Mat3x3<T> where Vec3<T>: AddAssign<Vec3<T>> {
    fn add_assign(&mut self,other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

// matrix - matrix
impl<T> Sub<Mat3x3<T>> for Mat3x3<T> where Vec3<T>: Sub<Vec3<T>,Output=Vec3<T>> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Mat3x3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// matrix -= matrix
impl<T> SubAssign<Mat3x3<T>> for Mat3x3<T> where Vec3<T>: SubAssign<Vec3<T>> {
    fn sub_assign(&mut self,other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

// scalar * matrix
macro_rules! scalar_mat3x3_mul {
    ($($t:ty)+) => {
        $(
            impl Mul<Mat3x3<$t>> for $t {
                type Output = Mat3x3<$t>;
                fn mul(self,other: Mat3x3<$t>) -> Mat3x3<$t> {
                    Mat3x3 {
                        x: self * other.x,
                        y: self * other.y,
                        z: self * other.z,
                    }
                }
            }
        )+
    }
}

scalar_mat3x3_mul!(f32 f64);

// matrix * scalar
impl<T> Mul<T> for Mat3x3<T> where Vec3<T>: Mul<T,Output=Vec3<T>> {
    type Output = Self;
    fn mul(self,other: T) -> Self {
        Mat3x3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

// matrix * vector
impl<T: Copy + Mul<T,Output=T> + Add<T,Output=T>> Mul<Vec3<T>> for Mat3x3<T> {
    type Output = Vec3<T>;
    fn mul(self,other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x.x * other.x + self.x.y * other.y + self.x.z * other.z,
            y: self.y.x * other.x + self.y.y * other.y + self.y.z * other.z,
            z: self.z.x * other.x + self.z.y * other.y + self.z.z * other.z,
        }
    }
}

// matrix * matrix
impl<T: Copy + Mul<T,Output=T> + Add<T,Output=T>> Mul<Mat3x3<T>> for Mat3x3<T> {
    type Output = Mat3x3<T>;
    fn mul(self,other: Mat3x3<T>) -> Mat3x3<T> {
        Mat3x3 {
            x: Vec3 {
                x: self.x.x * other.x.x + self.x.y * other.y.x + self.x.z * other.z.x,
                y: self.x.x * other.x.y + self.x.y * other.y.y + self.x.z * other.z.y,
                z: self.x.x * other.x.z + self.x.y * other.y.z + self.x.z * other.z.z,
            },
            y: Vec3 {
                x: self.y.x * other.x.x + self.y.y * other.y.x + self.y.z * other.z.x,
                y: self.y.x * other.x.y + self.y.y * other.y.y + self.y.z * other.z.y,
                z: self.y.x * other.x.z + self.y.y * other.y.z + self.y.z * other.z.z,
            },
            z: Vec3 {
                x: self.z.x * other.x.x + self.z.y * other.y.x + self.z.z * other.z.x,
                y: self.z.x * other.x.y + self.z.y * other.y.y + self.z.z * other.z.y,
                z: self.z.x * other.x.z + self.z.y * other.y.z + self.z.z * other.z.z,
            },
        }
    }
}

// matrix *= scalar
impl<T> MulAssign<T> for Mat3x3<T> where Vec3<T>: MulAssign<T> {
    fn mul_assign(&mut self,other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

// matrix *= matrix
impl<T: Copy + Mul<T,Output=T> + Add<T,Output=T>> MulAssign<Mat3x3<T>> for Mat3x3<T> {
    fn mul_assign(&mut self,other: Mat3x3<T>) {
        let xx = self.x.x * other.x.x + self.x.y * other.y.x + self.x.z * other.z.x;
        let xy = self.x.x * other.x.y + self.x.y * other.y.y + self.x.z * other.z.y;
        let xz = self.x.x * other.x.z + self.x.y * other.y.z + self.x.z * other.z.z;
        self.x = Vec3 { x: xx,y: xy,z: xz, };
        let yx = self.y.x * other.x.x + self.y.y * other.y.x + self.y.z * other.z.x;
        let yy = self.y.x * other.x.y + self.y.y * other.y.y + self.y.z * other.z.y;
        let yz = self.y.x * other.x.z + self.y.y * other.y.z + self.y.z * other.z.z;
        self.y = Vec3 { x: yx,y: yy,z: yz, };
        let zx = self.z.x * other.x.x + self.z.y * other.y.x + self.z.z * other.z.x;
        let zy = self.z.x * other.x.y + self.z.y * other.y.y + self.z.z * other.z.y;
        let zz = self.z.x * other.x.z + self.z.y * other.y.z + self.z.z * other.z.z;
        self.z = Vec3 { x: zx,y: zy,z: zz, }
    }
}

// matrix / scalar
impl<T: Copy + Div<T,Output=T>> Div<T> for Mat3x3<T> {
    type Output = Mat3x3<T>;
    fn div(self,other: T) -> Mat3x3<T> {
        Mat3x3 {
            x: Vec3 { x: self.x.x / other,y: self.x.y / other,z: self.x.z / other, },
            y: Vec3 { x: self.y.x / other,y: self.y.y / other,z: self.y.z / other, },
            z: Vec3 { x: self.z.x / other,y: self.z.y / other,z: self.z.z / other, },
        }
    }
}

// matrix /= scalar
impl<T: Copy + DivAssign<T>> DivAssign<T> for Mat3x3<T> {
    fn div_assign(&mut self,other: T) {
        self.x.x /= other;
        self.x.y /= other;
        self.x.z /= other;
        self.y.x /= other;
        self.y.y /= other;
        self.y.z /= other;
        self.z.x /= other;
        self.z.y /= other;
        self.z.z /= other;
    }
}

// -matrix
impl<T: Neg<Output=T>> Neg for Mat3x3<T> {
    type Output = Mat3x3<T>;
    fn neg(self) -> Mat3x3<T> {
        Mat3x3 {
            x: Vec3 { x: -self.x.x,y: -self.x.y,z: -self.x.z, },
            y: Vec3 { x: -self.y.x,y: -self.y.y,z: -self.y.z, },
            z: Vec3 { x: -self.z.x,y: -self.z.y,z: -self.z.z, },
        }
    }
}
*/