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

use GenMat;

use vec::vec::{Vector2, Vector3, Vector4};

use std::mem;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Rem, Sub};

use basenum::{ApproxEq, BaseFloat};
use num::Zero;
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use rand::{Rand, Rng};

macro_rules! mul_v_unrolled {
    ($m: ident, $v: ident, Vector2, Vector2) => {
        Vector2::new(
            $m[0].x * $v.x + $m[1].x * $v.y,
            $m[0].y * $v.x + $m[1].y * $v.y,
        )
    };
    ($m: ident, $v: ident, Vector2, Vector3) => {
        Vector2::new(
            $m[0].x * $v.x + $m[1].x * $v.y + $m[2].x * $v.z,
            $m[0].y * $v.x + $m[1].y * $v.y + $m[2].y * $v.z,
        )
    };
    ($m: ident, $v: ident, Vector2, Vector4) => {
        Vector2::new(
            $m[0].x * $v.x + $m[1].x * $v.y + $m[2].x * $v.z + $m[3].x * $v.w,
            $m[0].y * $v.x + $m[1].y * $v.y + $m[2].y * $v.z + $m[3].y * $v.w,
        )
    };
    ($m: ident, $v: ident, Vector3, Vector2) => {
        Vector3::new(
            $m[0].x * $v.x + $m[1].x * $v.y,
            $m[0].y * $v.x + $m[1].y * $v.y,
            $m[0].z * $v.x + $m[1].z * $v.y,
        )
    };
    ($m: ident, $v: ident, Vector3, Vector3) => {
        Vector3::new(
            $m[0].x * $v.x + $m[1].x * $v.y + $m[2].x * $v.z,
            $m[0].y * $v.x + $m[1].y * $v.y + $m[2].y * $v.z,
            $m[0].z * $v.x + $m[1].z * $v.y + $m[2].z * $v.z,
        )
    };
    ($m: ident, $v: ident, Vector3, Vector4) => {
        Vector3::new(
            $m[0].x * $v.x + $m[1].x * $v.y + $m[2].x * $v.z + $m[3].x * $v.w,
            $m[0].y * $v.x + $m[1].y * $v.y + $m[2].y * $v.z + $m[3].y * $v.w,
            $m[0].z * $v.x + $m[1].z * $v.y + $m[2].z * $v.z + $m[3].z * $v.w,
        )
    };
    ($m: ident, $v: ident, Vector4, Vector2) => {
        Vector4::new(
            $m[0].x * $v.x + $m[1].x * $v.y,
            $m[0].y * $v.x + $m[1].y * $v.y,
            $m[0].z * $v.x + $m[1].z * $v.y,
            $m[0].w * $v.x + $m[1].w * $v.y,
        )
    };
    ($m: ident, $v: ident, Vector4, Vector3) => {
        Vector4::new(
            $m[0].x * $v.x + $m[1].x * $v.y + $m[2].x * $v.z,
            $m[0].y * $v.x + $m[1].y * $v.y + $m[2].y * $v.z,
            $m[0].z * $v.x + $m[1].z * $v.y + $m[2].z * $v.z,
            $m[0].w * $v.x + $m[1].w * $v.y + $m[2].w * $v.z,
        )
    };
    ($m: ident, $v: ident, Vector4, Vector4) => {
        Vector4::new(
            $m[0].x * $v.x + $m[1].x * $v.y + $m[2].x * $v.z + $m[3].x * $v.w,
            $m[0].y * $v.x + $m[1].y * $v.y + $m[2].y * $v.z + $m[3].y * $v.w,
            $m[0].z * $v.x + $m[1].z * $v.y + $m[2].z * $v.z + $m[3].z * $v.w,
            $m[0].w * $v.x + $m[1].w * $v.y + $m[2].w * $v.z + $m[3].w * $v.w,
        )
    };
}

macro_rules! mul_m_unrolled {
    ($lm: ident, $rm: ident, Matrix2) => {
        Matrix2::new($lm.mul_v(&$rm.c0), $lm.mul_v(&$rm.c1))
    };
    ($lm: ident, $rm: ident, Matrix3) => {
        Matrix3::new($lm.mul_v(&$rm.c0), $lm.mul_v(&$rm.c1), $lm.mul_v(&$rm.c2))
    };
    ($lm: ident, $rm: ident, Matrix4) => {
        Matrix4::new(
            $lm.mul_v(&$rm.c0),
            $lm.mul_v(&$rm.c1),
            $lm.mul_v(&$rm.c2),
            $lm.mul_v(&$rm.c3),
        )
    };
}

macro_rules! transpose_unrolled {
    ($m: ident, Vector2, Vector2) => {
        Matrix2::new(
            Vector2::new($m[0][0], $m[1][0]),
            Vector2::new($m[0][1], $m[1][1]),
        )
    };
    ($m: ident, Vector2, Vector3) => {
        Matrix2x3::new(
            Vector3::new($m[0][0], $m[1][0], $m[2][0]),
            Vector3::new($m[0][1], $m[1][1], $m[2][1]),
        )
    };
    ($m: ident, Vector2, Vector4) => {
        Matrix2x4::new(
            Vector4::new($m[0][0], $m[1][0], $m[2][0], $m[3][0]),
            Vector4::new($m[0][1], $m[1][1], $m[2][1], $m[3][1]),
        )
    };
    ($m: ident, Vector3, Vector2) => {
        Matrix3x2::new(
            Vector2::new($m[0][0], $m[1][0]),
            Vector2::new($m[0][1], $m[1][1]),
            Vector2::new($m[0][2], $m[1][2]),
        )
    };
    ($m: ident, Vector3, Vector3) => {
        Matrix3::new(
            Vector3::new($m[0][0], $m[1][0], $m[2][0]),
            Vector3::new($m[0][1], $m[1][1], $m[2][1]),
            Vector3::new($m[0][2], $m[1][2], $m[2][2]),
        )
    };
    ($m: ident, Vector3, Vector4) => {
        Matrix3x4::new(
            Vector4::new($m[0][0], $m[1][0], $m[2][0], $m[3][0]),
            Vector4::new($m[0][1], $m[1][1], $m[2][1], $m[3][1]),
            Vector4::new($m[0][2], $m[1][2], $m[2][2], $m[3][2]),
        )
    };
    ($m: ident, Vector4, Vector2) => {
        Matrix4x2::new(
            Vector2::new($m[0][0], $m[1][0]),
            Vector2::new($m[0][1], $m[1][1]),
            Vector2::new($m[0][2], $m[1][2]),
            Vector2::new($m[0][3], $m[1][3]),
        )
    };
    ($m: ident, Vector4, Vector3) => {
        Matrix4x3::new(
            Vector3::new($m[0][0], $m[1][0], $m[2][0]),
            Vector3::new($m[0][1], $m[1][1], $m[2][1]),
            Vector3::new($m[0][2], $m[1][2], $m[2][2]),
            Vector3::new($m[0][3], $m[1][3], $m[2][3]),
        )
    };
    ($m: ident, Vector4, Vector4) => {
        Matrix4::new(
            Vector4::new($m[0][0], $m[1][0], $m[2][0], $m[3][0]),
            Vector4::new($m[0][1], $m[1][1], $m[2][1], $m[3][1]),
            Vector4::new($m[0][2], $m[1][2], $m[2][2], $m[3][2]),
            Vector4::new($m[0][3], $m[1][3], $m[2][3], $m[3][3]),
        )
    };
}

macro_rules! def_matrix {
    ($({
        $t: ident,          // type to be defined,
        $ct: ident,         // type of column vector,
        $($field: ident), + // name of columen vectors.
    }), +) => {
        $(
            #[repr(C)]
            #[derive(Copy, Clone, PartialEq, Debug)]
            pub struct $t<T: BaseFloat> {
                $(pub $field: $ct<T>), +
            }
        )+
    }
}

def_matrix! {
    { Matrix2,   Vector2, c0, c1 },
    { Matrix3x2, Vector2, c0, c1, c2 },
    { Matrix4x2, Vector2, c0, c1, c2, c3 },

    { Matrix2x3, Vector3, c0, c1 },
    { Matrix3,   Vector3, c0, c1, c2 },
    { Matrix4x3, Vector3, c0, c1, c2, c3 },

    { Matrix2x4, Vector4, c0, c1 },
    { Matrix3x4, Vector4, c0, c1, c2 },
    { Matrix4,   Vector4, c0, c1, c2, c3 }
}

macro_rules! impl_matrix {
    ($({
        $t: ident,          // type to impl (e.g., Matrix3),
        $ct: ident,         // type of column vector (e.g., Vec2),
        $rt: ident,         // type of row vector,
        $tr: ident,         // type of transpose matrix,
        $om: ident,         // the product of multiplying transpose matrix,
        $cn: expr,          // number of columns, i.e., the dimension of $rt,
        $($field: ident), + // fields for repeating reference columns,
    }), +) => {
        $(
            impl<T: BaseFloat> $t<T> {
                #[inline(always)]
                pub fn new($($field: $ct<T>), +) -> $t<T> {
                    $t { $($field: $field), + }
                }
                #[inline(always)]
                pub fn from_array(ary: &[$ct<T>; $cn]) -> &$t<T> {
                    let m: &Self = unsafe { mem::transmute(ary) };
                    m
                }
                #[inline(always)]
                pub fn from_array_mut(ary: &mut [$ct<T>; $cn]) -> &mut $t<T> {
                    let m: &mut Self = unsafe { mem::transmute(ary) };
                    m
                }
                #[inline(always)]
                pub fn as_array(&self) -> &[$ct<T>; $cn] {
                    let ary: &[$ct<T>; $cn] = unsafe { mem::transmute(self) };
                    ary
                }
                #[inline(always)]
                pub fn as_array_mut(&mut self) -> &mut [$ct<T>; $cn] {
                    let ary: &mut[$ct<T>; $cn] = unsafe { mem::transmute(self) };
                    ary
                }
                #[inline(always)]
                pub fn add_s(&self, rhs: T) -> $t<T> {
                    $t::new($(self.$field + rhs), +)
                }
                #[inline(always)]
                pub fn add_m(&self, rhs: &$t<T>) -> $t<T> {
                    $t::new($(self.$field + rhs.$field), +)
                }
                #[inline(always)]
                pub fn sub_s(&self, rhs: T) -> $t<T> {
                    $t::new($(self.$field - rhs), +)
                }
                #[inline(always)]
                pub fn sub_m(&self, rhs: &$t<T>) -> $t<T> {
                    $t::new($(self.$field - rhs.$field), +)
                }
                #[inline(always)]
                pub fn div_m(&self, rhs: &$t<T>) -> $t<T> {
                    $t::new($(self.$field / rhs.$field), +)
                }
                #[inline(always)]
                pub fn div_s(&self, rhs: T) -> $t<T> {
                    $t::new($(self.$field / rhs), +)
                }
                #[inline(always)]
                pub fn rem_m(&self, rhs: &$t<T>) -> $t<T> {
                    $t::new($(self.$field % rhs.$field), +)
                }
                #[inline(always)]
                pub fn rem_s(&self, rhs: T) -> $t<T> {
                    $t::new($(self.$field % rhs), +)
                }
                #[inline(always)]
                pub fn mul_s(&self, rhs: T) -> $t<T> {
                    $t::new($(self.$field * rhs), +)
                }
                #[inline(always)]
                pub fn mul_v(&self, rhs: &$rt<T>) -> $ct<T> {
                    mul_v_unrolled! { self, rhs, $ct, $rt }
                }
                #[inline(always)]
                pub fn mul_m(&self, rhs: &$tr<T>) -> $om<T> {
                    mul_m_unrolled! { self, rhs, $om }
                }
                #[inline(always)]
                pub fn neg_m(&self) -> $t<T> {
                    $t::new($(self.$field.neg()), +)
                }
            }
            impl<T: BaseFloat> Index<usize> for $t<T> {
                type Output = $ct<T>;
                #[inline(always)]
                fn index<'a>(&'a self, i: usize) -> &'a $ct<T> {
                    self.as_array().index(i)
                }
            }
            impl<T: BaseFloat> IndexMut<usize> for $t<T> {
                #[inline(always)]
                fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut $ct<T> {
                    self.as_array_mut().index_mut(i)
                }
            }
            impl<T: BaseFloat> Rand for $t<T> {
                #[inline]
                fn rand<R: Rng>(rng: &mut R) -> $t<T> {
                    $t {$($field: rng.gen()),+}
                }
            }
            #[cfg(test)]
            impl<T: BaseFloat + Arbitrary> Arbitrary for $t<T>
            where T::FromStrRadixErr: 'static {
                #[inline]
                fn arbitrary<G: Gen>(g: &mut G) -> $t<T> {
                    $t { $($field: $ct::<T>::arbitrary(g)),+ }
                }
            }
            impl<T: BaseFloat> Add<T> for $t<T> {
                type Output = $t<T>;
                #[inline(always)]
                fn add(self, rhs: T) -> $t<T> {
                    self.add_s(rhs)
                }
            }
            impl<T: BaseFloat> ApproxEq for $t<T> {
                type BaseType = T;
                #[inline]
                fn is_close_to(&self, rhs: &$t<T>, max_diff: T) -> bool {
                    $(self.$field.is_close_to(&rhs.$field, max_diff)) && +
                }
            }
            impl<T: BaseFloat> Add<$t<T>> for $t<T> {
                type Output = $t<T>;
                #[inline(always)]
                fn add(self, rhs: $t<T>) -> $t<T> {
                    self.add_m(&rhs)
                }
            }
            impl<T: BaseFloat> Sub<T> for $t<T> {
                type Output = $t<T>;
                #[inline(always)]
                fn sub(self, rhs: T) -> $t<T> {
                    self.sub_s(rhs)
                }
            }
            impl<T: BaseFloat> Sub<$t<T>> for $t<T> {
                type Output = $t<T>;
                #[inline(always)]
                fn sub(self, rhs: $t<T>) -> $t<T> {
                    self.sub_m(&rhs)
                }
            }
            impl<T: BaseFloat> Div<T> for $t<T> {
                type Output = $t<T>;
                #[inline(always)]
                fn div(self, rhs: T) -> $t<T> {
                    self.div_s(rhs)
                }
            }
            impl<T: BaseFloat> Div<$t<T>> for $t<T> {
                type Output = $t<T>;
                #[inline(always)]
                fn div(self, rhs: $t<T>) -> $t<T> {
                    self.div_m(&rhs)
                }
            }
            impl<T: BaseFloat> Rem<T> for $t<T> {
                type Output = $t<T>;
                #[inline(always)]
                fn rem(self, rhs: T) -> $t<T> {
                    self.rem_s(rhs)
                }
            }
            impl<T: BaseFloat> Rem<$t<T>> for $t<T> {
                type Output = $t<T>;
                #[inline(always)]
                fn rem(self, rhs: $t<T>) -> $t<T> {
                    self.rem_m(&rhs)
                }
            }
            impl<T: BaseFloat> Neg for $t<T> {
                type Output = $t<T>;
                #[inline(always)]
                fn neg(self) -> $t<T> {
                    self.neg_m()
                }
            }
            impl<T: BaseFloat> Mul<T> for $t<T> {
                type Output = $t<T>;
                #[inline(always)]
                fn mul(self, rhs: T) -> $t<T> {
                    self.mul_s(rhs)
                }
            }
            impl<T: BaseFloat> Mul<$rt<T>> for $t<T> {
                type Output = $ct<T>;
                #[inline(always)]
                fn mul(self, rhs: $rt<T>) -> $ct<T> {
                    self.mul_v(&rhs)
                }
            }
            impl<T: BaseFloat> Mul<$tr<T>> for $t<T> {
                type Output = $om<T>;
                #[inline(always)]
                fn mul(self, rhs: $tr<T>) -> $om<T> {
                    self.mul_m(&rhs)
                }
            }
            impl<T: BaseFloat> Zero for $t<T> {
                #[inline(always)]
                fn zero() -> $t<T> {
                    $t { $($field: $ct::<T>::zero()), + }
                }
                #[inline(always)]
                fn is_zero(&self) -> bool {
                    $(self.$field.is_zero()) && +
                }
            }
            impl<T: BaseFloat> GenMat<T, $ct<T>> for $t<T> {
                type R = $rt<T>;
                type Transpose = $tr<T>;
                #[inline]
                fn transpose(&self) -> $tr<T> {
                    transpose_unrolled!(self, $ct, $rt)
                }
                #[inline(always)]
                fn mul_c(&self, rhs: &$t<T>) -> $t<T> {
                    $t::new($(self.$field * rhs.$field), +)
                }
            }
       )+
    }
}

impl_matrix! {
    { Matrix2,   Vector2, Vector2, Matrix2,   Matrix2, 2, c0, c1 },
    { Matrix3x2, Vector2, Vector3, Matrix2x3, Matrix2, 3, c0, c1, c2 },
    { Matrix4x2, Vector2, Vector4, Matrix2x4, Matrix2, 4, c0, c1, c2, c3 },

    { Matrix2x3, Vector3, Vector2, Matrix3x2, Matrix3, 2, c0, c1 },
    { Matrix3,   Vector3, Vector3, Matrix3,   Matrix3, 3, c0, c1, c2 },
    { Matrix4x3, Vector3, Vector4, Matrix3x4, Matrix3, 4, c0, c1, c2, c3 },

    { Matrix2x4, Vector4, Vector2, Matrix4x2, Matrix4, 2, c0, c1 },
    { Matrix3x4, Vector4, Vector3, Matrix4x3, Matrix4, 3, c0, c1, c2 },
    { Matrix4,   Vector4, Vector4, Matrix4,   Matrix4, 4, c0, c1, c2, c3 }
}

macro_rules! impl_mul(
    ($({
        $t: ident, $rhs: ident, $output: ident, $($field: ident), +
    }), +) => {
        $(
            impl<T: BaseFloat> Mul<$rhs<T>> for $t<T> {
                type Output = $output<T>;
                #[inline(always)]
                fn mul(self, rhs: $rhs<T>) -> $output<T> {
                    $output::new(
                        $(self.mul_v(&rhs.$field)), +
                    )
                }
            }
        )+
    };
);

impl_mul! {
    { Matrix2x3, Matrix2,   Matrix2x3, c0, c1 },
    { Matrix2x4, Matrix2,   Matrix2x4, c0, c1 },

    { Matrix2,   Matrix3x2, Matrix3x2, c0, c1, c2 },
    { Matrix2x4, Matrix3x2, Matrix3x4, c0, c1, c2 },

    { Matrix2,   Matrix4x2, Matrix4x2, c0, c1, c2, c3 },
    { Matrix2x3, Matrix4x2, Matrix4x3, c0, c1, c2, c3 },

    { Matrix3,   Matrix2x3, Matrix2x3, c0, c1 },
    { Matrix3x4, Matrix2x3, Matrix2x4, c0, c1 },

    { Matrix3x2, Matrix3,   Matrix3x2, c0, c1, c2 },
    { Matrix3x4, Matrix3,   Matrix3x4, c0, c1, c2 },

    { Matrix3x2, Matrix4x3, Matrix4x2, c0, c1, c2, c3 },
    { Matrix3,   Matrix4x3, Matrix4x3, c0, c1, c2, c3 },

    { Matrix4x3, Matrix2x4, Matrix2x3, c0, c1 },
    { Matrix4,   Matrix2x4, Matrix2x4, c0, c1 },

    { Matrix4x2, Matrix3x4, Matrix3x2, c0, c1, c2 },
    { Matrix4,   Matrix3x4, Matrix3x4, c0, c1, c2 },

    { Matrix4x2, Matrix4,   Matrix4x2, c0, c1, c2, c3 },
    { Matrix4x3, Matrix4,   Matrix4x3, c0, c1, c2, c3 }
}

macro_rules! def_alias(
    (
        $({
            $a: ident,          // type alias (e.g., Mat2 for Matrix2<f32>),
            $t: ident,          // type to be aliased,
            $et: ty             // element type,
        }), +
    ) => {
        $(
            pub type $a = $t<$et>;
        )+
    }
);

def_alias! {
    { Mat2,   Matrix2,   f32 },
    { Mat3x2, Matrix3x2, f32 },
    { Mat4x2, Matrix4x2, f32 },

    { Mat2x3, Matrix2x3, f32 },
    { Mat3,   Matrix3,   f32 },
    { Mat4x3, Matrix4x3, f32 },

    { Mat2x4, Matrix2x4, f32 },
    { Mat3x4, Matrix3x4, f32 },
    { Mat4,   Matrix4,   f32 },

    { DMat2,   Matrix2,   f64 },
    { DMat3x2, Matrix3x2, f64 },
    { DMat4x2, Matrix4x2, f64 },

    { DMat2x3, Matrix2x3, f64 },
    { DMat3,   Matrix3,   f64 },
    { DMat4x3, Matrix4x3, f64 },

    { DMat2x4, Matrix2x4, f64 },
    { DMat3x4, Matrix3x4, f64 },
    { DMat4,   Matrix4,   f64 }
}

impl<T: BaseFloat> Matrix2<T> {
    /// Extends _self_ to a `Matrix3x2` by appending the column vector `z`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use glm::*;
    ///
    /// let m2 = mat2(1., 2., 3., 4.);
    /// let m3x2 = mat3x2(1., 2., 3., 4., 0., 0.);
    /// assert_eq!(m2.extend(vec2(0., 0.)), m3x2);
    /// ```
    #[inline]
    pub fn extend(&self, z: Vector2<T>) -> Matrix3x2<T> {
        Matrix3x2::new(self[0], self[1], z)
    }
}

impl<T: BaseFloat> Matrix3<T> {
    /// Extends _self_ to a `Matrix4x3` by appending the column vector `w`.
    #[inline]
    pub fn extend(&self, w: Vector3<T>) -> Matrix4x3<T> {
        Matrix4x3::new(self[0], self[1], self[2], w)
    }
}

fn combine_vec<T: BaseFloat>(a: &Vector3<T>, b: &Vector3<T>, a_scale: T, b_scale: T) -> Vector3<T> {
    *a * a_scale + *b * b_scale
}

fn scale_vec<T: BaseFloat>(v: &Vector3<T>, desired_length: T) -> Vector3<T> {
    *v * desired_length / crate::length(*v)
}

impl<T: BaseFloat> Matrix4<T> {
    /// Extracts scale, orientation, translation, skew and perspective in this order.
    pub fn decompose(
        &self,
    ) -> Option<(Vector3<T>, Vector4<T>, Vector3<T>, Vector3<T>, Vector4<T>)> {
        let mut matrix = self.clone();

        if matrix[3][3].is_approx_eq(&T::zero()) {
            return None;
        }

        for i in 0..4 {
            for j in 0..4 {
                matrix[i][j] = matrix[i][j] / matrix[3][3];
            }
        }

        let mut perspective_matrix = matrix;
        for i in 0..3 {
            perspective_matrix[i][3] = T::zero();
        }
        perspective_matrix[3][3] = T::one();

        // Why is this marked "fixme" in the original..?
        if crate::determinant(&perspective_matrix).is_approx_eq(&T::zero()) {
            return None;
        }

        // Isolate perspective
        let perspective = if !(matrix[0][3].is_approx_eq(&T::zero())
            || matrix[1][3].is_approx_eq(&T::zero())
            || matrix[2][3].is_approx_eq(&T::zero()))
        {
            // Rhs of the equation
            let rhs = Vector4::new(matrix[0][3], matrix[1][3], matrix[2][3], matrix[3][3]);

            // Solve the equation (easiest way but not optimal)
            let inverse_perspective_matrix = crate::inverse(&perspective_matrix);
            let transposed_inverse_perspective_matrix = inverse_perspective_matrix.transpose();

            // Clear perspective partition
            matrix[0][3] = T::zero();
            matrix[1][3] = T::zero();
            matrix[2][3] = T::zero();
            matrix[3][3] = T::one();

            transposed_inverse_perspective_matrix * rhs
        } else {
            // No perspective
            Vector4::new(T::zero(), T::zero(), T::zero(), T::one())
        };

        let translation = matrix[3].truncate(3);
        matrix[3] = Vector4::new(T::zero(), T::zero(), T::zero(), matrix[3].w);

        // Get scale and shear
        let mut row = [Vector3::<T>::zero(); 3];
        for i in 0..3 {
            for j in 0..3 {
                row[i][j] = matrix[i][j];
            }
        }

        let mut scale = Vector3::new(T::zero(), T::zero(), T::zero());
        let mut skew = Vector3::new(T::zero(), T::zero(), T::zero());

        // Get X scale and normalize 1st row
        scale.x = crate::length(row[0]);
        row[0] = scale_vec(&row[0], T::one());

        // Compute XY shear factor
        skew.z = crate::dot(row[0], row[1]);
        // Make 2nd row orthogonal to 1st
        row[1] = combine_vec(&row[1], &row[0], T::one(), -skew.z);

        // Get Y scale and normalize 2nd row
        scale.y = crate::length(row[1]);
        row[1] = scale_vec(&row[1], T::one());
        skew.z = skew.z / scale.y;

        // Compute XZ and YZ shears, orthogonalize 3rd row
        skew.y = crate::dot(row[0], row[2]);
        row[2] = combine_vec(&row[2], &row[0], T::one(), -skew.y);
        skew.x = crate::dot(row[1], row[2]);
        row[2] = combine_vec(&row[2], &row[1], T::one(), -skew.x);

        // Get Z scale and normalize 3rd row
        scale.z = crate::length(row[2]);
        row[2] = scale_vec(&row[2], T::one());

        skew.y = skew.y / scale.z;
        skew.x = skew.x / scale.z;

        // At this point, the matrix (in rows[]) is orthonormal.
        // Check for a coordinate system flip.  If the determinant is -1, then negate the matrix
        // and the scaling factors.
        let pdum3 = crate::cross(row[1], row[2]);
        if crate::dot(row[1], pdum3) < T::zero() {
            for i in 0..3 {
                scale[i] = scale[i] * -T::one();
                row[i] = row[i] * -T::one();
            }
        }

        // Get rotation
        let mut orientation = Vector4::new(T::zero(), T::zero(), T::zero(), T::zero());

        let mut root = row[0].x + row[1].y + row[2].z;
        let trace = root;

        if trace > T::zero() {
            root = (trace + T::one()).sqrt();
            orientation.w = T::from(0.5).unwrap() * root;
            root = T::from(0.5).unwrap() / root;
            orientation.x = root * (row[1].z - row[2].y);
            orientation.y = root * (row[2].x - row[0].z);
            orientation.z = root * (row[0].y - row[1].x);
        }
        // Enf if > 0
        else {
            let next = [1, 2, 0];

            let mut i = 0;

            if row[1].y > row[0].x {
                i = 1;
            }
            if row[2].z > row[i][i] {
                i = 2;
            }

            let j = next[i];
            let k = next[j];

            // TODO: Add GLM_FORCE_QUAT_DATA_WXYZ equivalent (off = 1)
            let off = 0;

            root = (row[i][i] - row[j][j] - row[k][k] + T::one()).sqrt();

            orientation[i + off] = T::from(0.5).unwrap() * root;
            root = T::from(0.5).unwrap() / root;
            orientation[j + off] = root * (row[i][j] + row[j][i]);
            orientation[k + off] = root * (row[i][k] + row[k][i]);
            orientation.w = root * (row[j][k] - row[k][j]);
        } // End if <= 0

        Some((scale, orientation, translation, skew, perspective))
    }
}

#[cfg(test)]
mod test {

    use mat::ctor::*;
    use vec::vec::*;

    use crate::is_close_to;

    #[test]
    fn test_index() {
        let m = mat4x3(1., 2., 3., 2., 4., 6., 3., 6., 9., 4., 8., 12.);
        assert_eq!(m[3], vec3(4., 8., 12.));
        assert_eq!(m[1], m.c1);
        assert_eq!(m[2][0], 3.);
        assert_eq!(m[0][1], 2.)
    }

    #[test]
    #[should_panic]
    fn test_index_bound_check() {
        let m2 = mat2(1., 2., 3., 4.);
        m2[2];
    }

    #[test]
    fn test_mul_v() {
        let m = mat3x2(1., 2., 3., 4., 5., 6.);
        let v = vec3(-2., 0., 2.);
        let p = vec2(8., 8.);
        assert_eq!(m * v, p);
    }

    #[test]
    fn test_decompose() {
        // Example model matrix
        #[rustfmt::skip]
        let mat = mat4(
            0.177637, 0., 0.582154, 0.,
            0., 0.608653, 0., 0.,
            -0.582154, 0., 0.177637, 0.,
            146.278, 0., -106.38, 1.
        );

        let (scale, orientation, translation, skew, perspective) = mat.decompose().unwrap();

        // Results obtained by running the original C++ version on the same input matrix
        assert_close_to!(scale, vec3(0.608653, 0.608653, 0.608653), 1e-5);
        assert_close_to!(orientation, vec4(0., -0.595041, 0., 0.803695), 1e-5);
        assert_close_to!(translation, vec3(146.278, 0., -106.38), 1e-5);
        assert_close_to!(skew, vec3(0., -2.44822e-08, 0.), 1e-5);
        assert_close_to!(perspective, vec4(0., 0., 0., 1.), 1e-5);
    }
}
