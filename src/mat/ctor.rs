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

use super::mat::*;
use vec::vec::{ vec2, vec3, vec4, dvec2, dvec3, dvec4 };

#[inline]
pub fn mat2(
    m11: f32, m21: f32,
    m12: f32, m22: f32
) -> Mat2 {
    Matrix2 {
        c0: vec2(m11, m21),
        c1: vec2(m12, m22)
    }
}

#[inline]
pub fn mat3x2(
    m11: f32, m21: f32,
    m12: f32, m22: f32,
    m13: f32, m23: f32
) -> Mat3x2 {
    Matrix3x2 {
        c0: vec2(m11, m21),
        c1: vec2(m12, m22),
        c2: vec2(m13, m23)
    }
}

#[inline]
pub fn mat4x2(
    m11: f32, m21: f32,
    m12: f32, m22: f32,
    m13: f32, m23: f32,
    m14: f32, m24: f32
) -> Mat4x2 {
    Matrix4x2 {
        c0: vec2(m11, m21),
        c1: vec2(m12, m22),
        c2: vec2(m13, m23),
        c3: vec2(m14, m24)
    }
}

#[inline]
pub fn mat3(
    m11: f32, m21: f32, m31: f32,
    m12: f32, m22: f32, m32: f32,
    m13: f32, m23: f32, m33: f32
) -> Mat3 {
    Matrix3 {
        c0: vec3(m11, m21, m31),
        c1: vec3(m12, m22, m32),
        c2: vec3(m13, m23, m33)
    }
}

#[inline]
pub fn mat2x3(
    m11: f32, m21: f32, m31: f32,
    m12: f32, m22: f32, m32: f32
) -> Mat2x3 {
    Matrix2x3 {
        c0: vec3(m11, m21, m31),
        c1: vec3(m12, m22, m32)
    }
}

#[inline]
pub fn mat4x3(
    m11: f32, m21: f32, m31: f32,
    m12: f32, m22: f32, m32: f32,
    m13: f32, m23: f32, m33: f32,
    m14: f32, m24: f32, m34: f32
) -> Mat4x3 {
    Matrix4x3 {
        c0: vec3(m11, m21, m31),
        c1: vec3(m12, m22, m32),
        c2: vec3(m13, m23, m33),
        c3: vec3(m14, m24, m34)
    }
}

#[inline]
pub fn mat4(
    m11: f32, m21: f32, m31: f32, m41: f32,
    m12: f32, m22: f32, m32: f32, m42: f32,
    m13: f32, m23: f32, m33: f32, m43: f32,
    m14: f32, m24: f32, m34: f32, m44: f32
) -> Mat4 {
    Matrix4 {
        c0: vec4(m11, m21, m31, m41),
        c1: vec4(m12, m22, m32, m42),
        c2: vec4(m13, m23, m33, m43),
        c3: vec4(m14, m24, m34, m44)
    }
}

#[inline]
pub fn mat2x4(
    m11: f32, m21: f32, m31: f32, m41: f32,
    m12: f32, m22: f32, m32: f32, m42: f32,
) -> Mat2x4 {
    Matrix2x4 {
        c0: vec4(m11, m21, m31, m41),
        c1: vec4(m12, m22, m32, m42)
    }
}

#[inline]
pub fn mat3x4(
    m11: f32, m21: f32, m31: f32, m41: f32,
    m12: f32, m22: f32, m32: f32, m42: f32,
    m13: f32, m23: f32, m33: f32, m43: f32
) -> Mat3x4 {
    Matrix3x4 {
        c0: vec4(m11, m21, m31, m41),
        c1: vec4(m12, m22, m32, m42),
        c2: vec4(m13, m23, m33, m43)
    }
}

#[inline]
pub fn dmat2(
    m11: f64, m21: f64,
    m12: f64, m22: f64
) -> DMat2 {
    Matrix2 {
        c0: dvec2(m11, m21),
        c1: dvec2(m12, m22)
    }
}

#[inline]
pub fn dmat3x2(
    m11: f64, m21: f64,
    m12: f64, m22: f64,
    m13: f64, m23: f64
) -> DMat3x2 {
    Matrix3x2 {
        c0: dvec2(m11, m21),
        c1: dvec2(m12, m22),
        c2: dvec2(m13, m23)
    }
}

#[inline]
pub fn dmat4x2(
    m11: f64, m21: f64,
    m12: f64, m22: f64,
    m13: f64, m23: f64,
    m14: f64, m24: f64
) -> DMat4x2 {
    Matrix4x2 {
        c0: dvec2(m11, m21),
        c1: dvec2(m12, m22),
        c2: dvec2(m13, m23),
        c3: dvec2(m14, m24)
    }
}

#[inline]
pub fn dmat3(
    m11: f64, m21: f64, m31: f64,
    m12: f64, m22: f64, m32: f64,
    m13: f64, m23: f64, m33: f64
) -> DMat3 {
    Matrix3 {
        c0: dvec3(m11, m21, m31),
        c1: dvec3(m12, m22, m32),
        c2: dvec3(m13, m23, m33)
    }
}

#[inline]
pub fn dmat2x3(
    m11: f64, m21: f64, m31: f64,
    m12: f64, m22: f64, m32: f64
) -> DMat2x3 {
    Matrix2x3 {
        c0: dvec3(m11, m21, m31),
        c1: dvec3(m12, m22, m32)
    }
}

#[inline]
pub fn dmat4x3(
    m11: f64, m21: f64, m31: f64,
    m12: f64, m22: f64, m32: f64,
    m13: f64, m23: f64, m33: f64,
    m14: f64, m24: f64, m34: f64
) -> DMat4x3 {
    Matrix4x3 {
        c0: dvec3(m11, m21, m31),
        c1: dvec3(m12, m22, m32),
        c2: dvec3(m13, m23, m33),
        c3: dvec3(m14, m24, m34)
    }
}

#[inline]
pub fn dmat4(
    m11: f64, m21: f64, m31: f64, m41: f64,
    m12: f64, m22: f64, m32: f64, m42: f64,
    m13: f64, m23: f64, m33: f64, m43: f64,
    m14: f64, m24: f64, m34: f64, m44: f64
) -> DMat4 {
    Matrix4 {
        c0: dvec4(m11, m21, m31, m41),
        c1: dvec4(m12, m22, m32, m42),
        c2: dvec4(m13, m23, m33, m43),
        c3: dvec4(m14, m24, m34, m44)
    }
}

#[inline]
pub fn dmat2x4(
    m11: f64, m21: f64, m31: f64, m41: f64,
    m12: f64, m22: f64, m32: f64, m42: f64,
) -> DMat2x4 {
    Matrix2x4 {
        c0: dvec4(m11, m21, m31, m41),
        c1: dvec4(m12, m22, m32, m42)
    }
}

#[inline]
pub fn dmat3x4(
    m11: f64, m21: f64, m31: f64, m41: f64,
    m12: f64, m22: f64, m32: f64, m42: f64,
    m13: f64, m23: f64, m33: f64, m43: f64
) -> DMat3x4 {
    Matrix3x4 {
        c0: dvec4(m11, m21, m31, m41),
        c1: dvec4(m12, m22, m32, m42),
        c2: dvec4(m13, m23, m33, m43)
    }
}
