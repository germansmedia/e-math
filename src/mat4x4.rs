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
pub struct Mat4x4<T> {
    pub x: Vec4<T>,
    pub y: Vec4<T>,
    pub z: Vec4<T>,
    pub w: Vec4<T>,
}

impl<T: PartialEq + Zero + Add<T,Output=T> + Sub<T,Output=T> + Mul<T,Output=T> + Neg<Output=T>> Mat4x4<T> {

    pub fn transpose(self) -> Mat4x4<T> {
        Mat4x4 {
            x: Vec4 { x: self.x.x,y: self.y.x,z: self.z.x,w: self.w.x, },
            y: Vec4 { x: self.x.y,y: self.y.y,z: self.z.y,w: self.w.y, },
            z: Vec4 { x: self.x.z,y: self.y.z,z: self.z.z,w: self.w.z, },
            w: Vec4 { x: self.x.w,y: self.y.w,z: self.z.w,w: self.w.w, },
        }
    }

    pub fn determinant(self) -> T {
        let xx = self.x.x;
        let xy = self.x.y;
        let xz = self.x.z;
        let xw = self.x.w;
        let yx = self.y.x;
        let yy = self.y.y;
        let yz = self.y.z;
        let yw = self.y.w;
        let zx = self.z.x;
        let zy = self.z.y;
        let zz = self.z.z;
        let zw = self.z.w;
        let wx = self.w.x;
        let wy = self.w.y;
        let wz = self.w.z;
        let ww = self.w.w;
        let axx = yy * (zz * ww - wz * zw) - yz * (zw * wy - ww * zy) + yw * (zy * wz - wy * zz);
        let axy = -(yz * (zw * wx - ww * zx) - yw * (zx * wz - wx * zz) + yx * (zz * ww - wz * zw));
        let axz = yw * (zx * wy - wx * zy) - yx * (zy * ww - wy * zw) + yy * (zw * wx - ww * zx);
        let axw = -(yx * (zy * wz - wy * zz) - yy * (zz * wx - wz * wx) + yz * (wx * wy - wx * zy));
        xx * axx + xy * axy + xz * axz + xw * axw
    }

    pub fn inverse(self) -> Self {
        let xx = self.x.x;
        let xy = self.x.y;
        let xz = self.x.z;
        let xw = self.x.w;
        let yx = self.y.x;
        let yy = self.y.y;
        let yz = self.y.z;
        let yw = self.y.w;
        let zx = self.z.x;
        let zy = self.z.y;
        let zz = self.z.z;
        let zw = self.z.w;
        let wx = self.w.x;
        let wy = self.w.y;
        let wz = self.w.z;
        let ww = self.w.w;
        let axx = yy * (zz * ww - wz * zw) - yz * (zw * wy - ww * zy) + yw * (zy * wz - wy * zz);
        let axy = -(yz * (zw * wx - ww * zx) - yw * (zx * wz - wx * zz) + yx * (zz * ww - wz * zw));
        let axz = yw * (zx * wy - wx * zy) - yx * (zy * ww - wy * zw) + yy * (zw * wx - ww * zx);
        let axw = -(yx * (zy * wz - wy * zz) - yy * (zz * wx - wz * wx) + yz * (wx * wy - wx * zy));
        let det = xx * axx + xy * axy + xz * axz + xw * axw;
        if det == T::ZERO {
            return self;
        }
        let ayx = -(zy * (wz * xw - xz * ww) - zz * (ww * xy - xw * wy) + zw * (wy * xz - xy * wz));
        let ayy = zz * (ww * xx - xw * wx) - zw * (wx * xz - xx * wz) + zx * (wz * xw - xz * ww);
        let ayz = -(zw * (wx * xy - xx * wy) - zx * (wy * xw - xy * ww) + zy * (ww * xx - xw * wx));
        let ayw = zx * (wy * xz - xy * wz) - zy * (wz * xx - xz * wx) + zz * (wx * xy - xx * wy);
        let azx = wy * (xz * yw - yz * xw) - wz * (xw * yy - yz * xy) + ww * (xy * yz - yy * xz);
        let azy = -(wz * (xw * yx - yw * xx) - ww * (xx * yz - yx * xz) + wx * (xz * yw - yz * xw));
        let azz = ww * (xx * yy - yx * xy) - wx * (xy * yw - yy * xw) + wy * (xw * yx - yw * xx);
        let azw = -(wx * (xy * yz - yy * xz) - wy * (xz * yx - yz * xx) + wz * (xx * yy - yx * xy));
        let awx = -(xy * (yz * zw - zz * yw) - xz * (yw * zy - zw * yy) + xw * (yy * zz - zy * yz));
        let awy = xz * (yw * zx - zw * yx) - xw * (yx * zz - zx * yz) + xx * (yz * zw - zz * yw);
        let awz = -(xw * (yx * zy - zx * yy) - xx * (yy * zw - zy * yw) + xy * (yw * zx - zw * yx));
        let aww = xx * (yy * zz - zy * yz) - xy * (yz * zx - zz * yx) + xz * (yx * zy - zx * yy);
        Mat4x4 {
            x: Vec4 { x: axx,y: axy,z: axz,w: axw, },
            y: Vec4 { x: ayx,y: ayy,z: ayz,w: ayw, },
            z: Vec4 { x: azx,y: azy,z: azz,w: azw, },
            w: Vec4 { x: awx,y: awy,z: awz,w: aww, },
        } / det
    }
}

impl<T> From<[Vec4<T>; 4]> for Mat4x4<T> {
    fn from(array: [Vec4<T>; 4]) -> Self {
        Mat4x4 {
            x: array[0],
            y: array[1],
            z: array[2],
            w: array[3],
        }
    }
}

impl<T> From<&[Vec4<T>; 4]> for Mat4x4<T> {
    fn from(slice: &[Vec4<T>; 4]) -> Self {
        Mat4x4 {
            x: slice[0],
            y: slice[1],
            z: slice[2],
            w: slice[3],
        }
    }
}

impl<T> From<[T; 16]> for Mat4x4<T> {
    fn from(array: [T; 16]) -> Self {
        Mat4x4 {
            x: Vec4 { x: array[0],y: array[1],z: array[2],w: array[3], },
            y: Vec4 { x: array[4],y: array[5],z: array[6],w: array[7], },
            z: Vec4 { x: array[8],y: array[9],z: array[10],w: array[11], },
            w: Vec4 { x: array[12],y: array[13],z: array[14],w: array[15], },
        }
    }
}

impl<T> From<&[T; 16]> for Mat4x4<T> {
    fn from(slice: &[T; 16]) -> Self {
        Mat4x4 {
            x: Vec4 { x: slice[0],y: slice[1],z: slice[2],w: slice[3], },
            y: Vec4 { x: slice[4],y: slice[5],z: slice[6],w: slice[7], },
            z: Vec4 { x: slice[8],y: slice[9],z: slice[10],w: slice[11], },
            w: Vec4 { x: slice[12],y: slice[13],z: slice[14],w: slice[15], },
        }
    }
}

impl<T> PartialEq for Mat4x4<T> where Vec4<T>: PartialEq {
    fn eq(&self,other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y) && (self.z == other.z) && (self.w == other.w)
    }
}

impl<T> Display for Mat4x4<T> where Vec4<T>: Display {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"[{},{},{},{}]",self.x,self.y,self.z,self.w)
    }
}

// matrix + matrix
impl<T> Add<Mat4x4<T>> for Mat4x4<T> where Vec4<T>: Add<Vec4<T>,Output=Vec4<T>> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Mat4x4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

// matrix - matrix
impl<T> Sub<Mat4x4<T>> for Mat4x4<T> where Vec4<T>: Sub<Vec4<T>,Output=Vec4<T>> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Mat4x4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

// scalar * matrix
macro_rules! scalar_mat4x4_mul {
    ($($t:ty)+) => {
        $(
            impl Mul<Mat4x4<$t>> for $t {
                type Output = Mat4x4<$t>;
                fn mul(self,other: Mat4x4<$t>) -> Mat4x4<$t> {
                    Mat4x4 {
                        x: self * other.x,
                        y: self * other.y,
                        z: self * other.z,
                        w: self * other.w,
                    }
                }
            }
        )+
    }
}

scalar_mat4x4_mul!(f32 f64);

// matrix * scalar
impl<T> Mul<T> for Mat4x4<T> where Vec4<T>: Mul<T,Output=Vec4<T>> {
    type Output = Self;
    fn mul(self,other: T) -> Self {
        Mat4x4 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

// matrix * vector
impl<T: Add<T,Output=T> + Mul<T,Output=T>> Mul<Vec4<T>> for Mat4x4<T> {
    type Output = Vec4<T>;
    fn mul(self,other: Vec4<T>) -> Vec4<T> {
        Vec4 {
            x: self.x.x * other.x + self.x.y * other.y + self.x.z * other.z + self.x.w * other.w,
            y: self.y.x * other.x + self.y.y * other.y + self.y.z * other.z + self.y.w * other.w,
            z: self.z.x * other.x + self.z.y * other.y + self.z.z * other.z + self.z.w * other.w,
            w: self.w.x * other.x + self.w.y * other.y + self.w.z * other.z + self.w.w * other.w,
        }
    }
}

// matrix * matrix
impl<T: Copy + Mul<T,Output=T> + Add<T,Output=T>> Mul<Mat4x4<T>> for Mat4x4<T> {
    type Output = Mat4x4<T>;
    fn mul(self,other: Mat4x4<T>) -> Mat4x4<T> {
        Mat4x4 {
            x: Vec4 {
                x: self.x.x * other.x.x + self.x.y * other.y.x + self.x.z * other.z.x + self.x.w * other.w.x,
                y: self.x.x * other.x.y + self.x.y * other.y.y + self.x.z * other.z.y + self.x.w * other.w.y,
                z: self.x.x * other.x.z + self.x.y * other.y.z + self.x.z * other.z.z + self.x.w * other.w.z,
                w: self.x.x * other.x.w + self.x.y * other.y.w + self.x.z * other.z.w + self.x.w * other.w.w,
            },
            y: Vec4 {
                x: self.y.x * other.x.x + self.y.y * other.y.x + self.y.z * other.z.x + self.y.w * other.w.x,
                y: self.y.x * other.x.y + self.y.y * other.y.y + self.y.z * other.z.y + self.y.w * other.w.y,
                z: self.y.x * other.x.z + self.y.y * other.y.z + self.y.z * other.z.z + self.y.w * other.w.z,
                w: self.y.x * other.x.w + self.y.y * other.y.w + self.y.z * other.z.w + self.y.w * other.w.w,
            },
            z: Vec4 {
                x: self.z.x * other.x.x + self.z.y * other.y.x + self.z.z * other.z.x + self.z.w * other.w.x,
                y: self.z.x * other.x.y + self.z.y * other.y.y + self.z.z * other.z.y + self.z.w * other.w.y,
                z: self.z.x * other.x.z + self.z.y * other.y.z + self.z.z * other.z.z + self.z.w * other.w.z,
                w: self.z.x * other.x.w + self.z.y * other.y.w + self.z.z * other.z.w + self.z.w * other.w.w,
            },
            w: Vec4 {
                x: self.w.x * other.x.x + self.w.y * other.y.x + self.w.z * other.z.x + self.w.w * other.w.x,
                y: self.w.x * other.x.y + self.w.y * other.y.y + self.w.z * other.z.y + self.w.w * other.w.y,
                z: self.w.x * other.x.z + self.w.y * other.y.z + self.w.z * other.z.z + self.w.w * other.w.z,
                w: self.w.x * other.x.w + self.w.y * other.y.w + self.w.z * other.z.w + self.w.w * other.w.w,
            },
        }
    }
}

// matrix / scalar
impl<T: Copy + Div<T,Output=T>> Div<T> for Mat4x4<T> {
    type Output = Mat4x4<T>;
    fn div(self,other: T) -> Mat4x4<T> {
        Mat4x4 {
            x: Vec4 { x: self.x.x / other,y: self.x.y / other,z: self.x.z / other,w: self.x.w / other, },
            y: Vec4 { x: self.y.x / other,y: self.y.y / other,z: self.y.z / other,w: self.y.w / other, },
            z: Vec4 { x: self.z.x / other,y: self.z.y / other,z: self.z.z / other,w: self.z.w / other, },
            w: Vec4 { x: self.w.x / other,y: self.w.y / other,z: self.w.z / other,w: self.w.w / other, },
        }
    }
}

// matrix += matrix
impl<T> AddAssign<Mat4x4<T>> for Mat4x4<T> where Vec4<T>: AddAssign<Vec4<T>> {
    fn add_assign(&mut self,other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

// matrix -= matrix
impl<T> SubAssign<Mat4x4<T>> for Mat4x4<T> where Vec4<T>: SubAssign<Vec4<T>> {
    fn sub_assign(&mut self,other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

// matrix *= scalar
impl<T> MulAssign<T> for Mat4x4<T> where Vec4<T>: MulAssign<T> {
    fn mul_assign(&mut self,other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
        self.w *= other;
    }
}

// matrix *= matrix
impl<T: Copy + Mul<T,Output=T> + Add<T,Output=T>> MulAssign<Mat4x4<T>> for Mat4x4<T> {
    fn mul_assign(&mut self,other: Mat4x4<T>) {
        let xx = self.x.x * other.x.x + self.x.y * other.y.x + self.x.z * other.z.x + self.x.w * other.w.x;
        let xy = self.x.x * other.x.y + self.x.y * other.y.y + self.x.z * other.z.y + self.x.w * other.w.y;
        let xz = self.x.x * other.x.z + self.x.y * other.y.z + self.x.z * other.z.z + self.x.w * other.w.z;
        let xw = self.x.x * other.x.w + self.x.y * other.y.w + self.x.z * other.z.w + self.x.w * other.w.w;
        self.x = Vec4 { x: xx,y: xy,z: xz,w: xw, };
        let yx = self.y.x * other.x.x + self.y.y * other.y.x + self.y.z * other.z.x + self.y.w * other.w.x;
        let yy = self.y.x * other.x.y + self.y.y * other.y.y + self.y.z * other.z.y + self.y.w * other.w.y;
        let yz = self.y.x * other.x.z + self.y.y * other.y.z + self.y.z * other.z.z + self.y.w * other.w.z;
        let yw = self.y.x * other.x.w + self.y.y * other.y.w + self.y.z * other.z.w + self.y.w * other.w.w;
        self.y = Vec4 { x: yx,y: yy,z: yz,w: yw, };
        let zx = self.z.x * other.x.x + self.z.y * other.y.x + self.z.z * other.z.x + self.z.w * other.w.x;
        let zy = self.z.x * other.x.y + self.z.y * other.y.y + self.z.z * other.z.y + self.z.w * other.w.y;
        let zz = self.z.x * other.x.z + self.z.y * other.y.z + self.z.z * other.z.z + self.z.w * other.w.z;
        let zw = self.z.x * other.x.w + self.z.y * other.y.w + self.z.z * other.z.w + self.z.w * other.w.w;
        self.z = Vec4 { x: zx,y: zy,z: zz,w: zw, };
        let wx = self.w.x * other.x.x + self.w.y * other.y.x + self.w.z * other.z.x + self.w.w * other.w.x;
        let wy = self.w.x * other.x.y + self.w.y * other.y.y + self.w.z * other.z.y + self.w.w * other.w.y;
        let wz = self.w.x * other.x.z + self.w.y * other.y.z + self.w.z * other.z.z + self.w.w * other.w.z;
        let ww = self.w.x * other.x.w + self.w.y * other.y.w + self.w.z * other.z.w + self.w.w * other.w.w;
        self.w = Vec4 { x: wx,y: wy,z: wz,w: ww, };
    }
}

// matrix /= scalar
impl<T: Copy + DivAssign<T>> DivAssign<T> for Mat4x4<T> {
    fn div_assign(&mut self,other: T) {
        self.x.x /= other;
        self.x.y /= other;
        self.x.z /= other;
        self.x.w /= other;
        self.y.x /= other;
        self.y.y /= other;
        self.y.z /= other;
        self.y.w /= other;
        self.z.x /= other;
        self.z.y /= other;
        self.z.z /= other;
        self.z.w /= other;
        self.w.x /= other;
        self.w.y /= other;
        self.w.z /= other;
        self.w.w /= other;
    }
}

// -matrix
impl<T: Neg<Output=T>> Neg for Mat4x4<T> {
    type Output = Mat4x4<T>;
    fn neg(self) -> Mat4x4<T> {
        Mat4x4 {
            x: Vec4 { x: -self.x.x,y: -self.x.y,z: -self.x.z,w: -self.x.w, },
            y: Vec4 { x: -self.y.x,y: -self.y.y,z: -self.y.z,w: -self.y.w, },
            z: Vec4 { x: -self.z.x,y: -self.z.y,z: -self.z.z,w: -self.z.w, },
            w: Vec4 { x: -self.w.x,y: -self.w.y,z: -self.w.z,w: -self.w.w, },
        }
    }
}
