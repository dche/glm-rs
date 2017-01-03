//
// GLSL Mathematics for Rust.
//
// Copyright (c) 2015 The glm-rs authors.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

#![allow(unused_variables)]

//! GLSL mathematics for Rust programming language.
//!
//! *glm-rs* is yet another Rust math library for graphics applications.
//! Inspired by the great [GLM](glm.g-truc.net) library for C++, the goal is to
//! provide familiar math API to porgrammers who knows GLSL as well.
//!
//! ## Differences to GLSL specification
//!
//! Like *GLM*, following GLSL conventions is a strict policy of *glm-rs* too.
//! *glm* crate implements all GLSL data types, operators and
//! built-in functions. However, Rust is not a C-like language, and the
//! syntax/semantics distances from Rust to GLSL is way longer than from C++ to
//! GLSL.
//! This is the major reason of following feature and syntax differences to GLSL
//! specification,
//!
//! - Precision qualifiers is not supported,
//! - Half float type is not available, yet,
//! - There is no vector swizzle operators. For example, you can't do this,
//!
//!   ~~~ignore
//!   # use glm::*;
//!   let mut my_vec2 = my_vec4.wz;
//!   // and,
//!   my_vec2.yx = my_vec4.xx;
//!   ~~~
//!   Part of swizzle operators can be done but must be in a very tedious way
//!   at the moment.
//!   The plan is to implemente accessing swizzle operators *after* Rust macro
//!   supports concatenating identifiers.
//! - Because Rust does not support function name overloading, loads of
//!   convenient constructor functions can't be implemented. For example,
//!   you can't do this,
//!
//!   ~~~ignore
//!   let v2 = vec2(1., 2.);
//!   // compile error: this function takes 3 parameters but 2 parameters were supplied [E0061]
//!   let v3 = vec3(v2, 3.);
//!   ~~~
//!   This will be fixed in future version by introducing functions like
//!   ```no_run fn vec21(x: Vec2, y: f32) -> Vec3```, in which function names
//!   indicate the forms of result vectors.
//! - Also because of lacking of function name overloading, following built-in
//!   functions are added,
//!
//!   `atan2`, `mod_s`, `max_s`, `min_s`, `clamp_s`, `mix_s`, `mix_bool`, `step_s`,
//!   and `smoothstep_s`.
//!
//!   The suffix `_s` stands for *scalar*, which means this is a variant of
//!   original function that some parameters are specific to be scalar,
//!   instead a generic type.
//!
//!   See documentation of these functions for detail.
//! - Most explicit conversion functions, like `vec2`, `dmat4x3` etc., do not
//!   work, for the same reason as above.
//!   This is rather inconvenient, and the plan is introducing functions like
//!   `to_vec2`, `to_dmat4x3` in future version.
//! - No implicit conversion.
//! - Explicit type conversion function `bool` is renamed to `boolean`.
//! - Many explicit type conversion functions for vector types are introduced.
//!   In GLSL spec, these fucntions have the same name as vector type
//!   constructors, which is not allowed in Rust. The naming rule is to get a
//!   vector type conversion fucntion, adds a `to_` before the constructor
//!   function. For example,
//!
//!   ~~~
//!   # use glm::*;
//!   let v = to_vec2(1_f32);
//!   assert_eq!(v, vec2(1., 1.));
//!   ~~~
//! - Lots of convertion functions are still missing (e.g., from matrices to
//!   vectors). These functions are syntax sugar actually, and will be fixed
//!   along with the constructor issue in future version.
//! - GLSL uses out parameter for returning multiple results of functions. In
//!   Rust, we can do this by returning a tuple. Following functions'
//!   signatures are changed because of this,
//!
//!   - `modf`,
//!   - `frexp`,
//!   - `uaddCarry`,
//!   - `usubBorrow`,
//!   - `umulExtended`,
//!   - `imulExtended`
//!
//!   The rule of changing is the out parameter is the second member of the
//!   return tuple.
//! - Function parameters of matrix related functions (e.g., `inverse`,
//!   `cross`) are passed by reference, instead of by value as in the GLSL spec.
//!   For example,
//!
//!   ~~~
//!   # use glm::*;
//!   let m = mat2(1., 2., 3., 4.);
//!   // instead of `inverse(m)`,
//!   let inv = inverse(&m);
//!   ~~~
//! - Built-in function `mod` is renamed to `fmod`, because **mod** is a Rust
//!   keyword.
//!

extern crate rand;
extern crate num;
extern crate quickcheck;

pub use builtin::*;

pub use basenum::{
    Primitive, BaseNum, BaseInt, BaseFloat, SignedNum,
    ApproxEq, is_approx_eq, is_close_to
};

pub use traits::{
    GenNum, GenInt, GenFloat
};

pub use vec::traits::{
    GenVec, GenNumVec, GenFloatVec, GenBVec,
};

pub use vec::vec::{
    Vector2, Vector3, Vector4,
    BVec2, BVec3, BVec4, bvec2, bvec3, bvec4,
    IVec2, IVec3, IVec4, ivec2, ivec3, ivec4,
    UVec2, UVec3, UVec4, uvec2, uvec3, uvec4,
    Vec2, Vec3, Vec4, vec2, vec3, vec4,
    DVec2, DVec3, DVec4, dvec2, dvec3, dvec4,
};

// pub use vec::swizzle::{
//     Swizzle2, Swizzle3, Swizzle4,
// };

pub use mat::traits::{ GenMat, GenSquareMat };

pub use mat::mat::{
    Matrix2, Matrix3, Matrix4,
    Matrix3x2, Matrix4x2, Matrix2x3, Matrix4x3, Matrix2x4, Matrix3x4,
    Mat2, Mat3, Mat4,
    Mat3x2, Mat2x3, Mat4x2, Mat2x4, Mat4x3, Mat3x4,
    DMat2, DMat3, DMat4,
    DMat3x2, DMat2x3, DMat4x2, DMat2x4, DMat4x3, DMat3x4,
};

pub use mat::ctor::{
    mat2, mat3, mat4,
    mat3x2, mat2x3, mat4x2, mat2x4, mat4x3, mat3x4,
    dmat2, dmat3, dmat4,
    dmat3x2, dmat2x3, dmat4x2, dmat2x4, dmat4x3, dmat3x4,
};

pub use cast::{
    PrimCast,
    int, uint, float, double, boolean,
    to_ivec2, to_ivec3, to_ivec4,
    to_uvec2, to_uvec3, to_uvec4,
    to_vec2, to_vec3, to_vec4,
    to_dvec2, to_dvec3, to_dvec4,
    to_bvec2, to_bvec3, to_bvec4
};

#[macro_use]
mod basenum;
mod traits;
mod vec {
    pub mod traits;
    pub mod vec;
}
mod mat {
    pub mod traits;
    pub mod mat;
    pub mod ctor;
    pub mod sqmat;
}
mod cast;

pub mod builtin;
pub mod ext;
