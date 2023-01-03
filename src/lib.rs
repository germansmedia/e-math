#![feature(const_trait_impl)]
#![feature(const_fn_floating_point_arithmetic)]

mod zero;
pub use zero::*;

mod rational;
pub use rational::*;

mod complex;
pub use complex::*;

mod vector;
pub use vector::*;

mod vec2;
pub use vec2::*;

mod vec3;
pub use vec3::*;

mod vec4;
pub use vec4::*;

mod matrix;
pub use matrix::*;

mod mat2x2;
pub use mat2x2::*;

mod mat3x3;
pub use mat3x3::*;

mod mat4x4;
pub use mat4x4::*;

mod quaternion;
pub use quaternion::*;

mod euler;
pub use euler::*;

mod pose;
pub use pose::*;

mod multivec2;
pub use multivec2::*;

mod multivec3;
pub use multivec3::*;

mod multivec4;
pub use multivec4::*;
