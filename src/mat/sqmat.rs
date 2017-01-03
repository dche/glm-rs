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

use basenum::BaseFloat;
use vec::vec::{ Vector2, Vector3, Vector4 };
use super::traits::{ GenMat, GenSquareMat };
use super::mat::*;
use num::One;

impl<T: BaseFloat> One for Matrix2<T> {
    #[inline]
    fn one() -> Matrix2<T> {
        let y = T::one();
        let l = T::zero();
        Matrix2::new(
            Vector2::new(y, l),
            Vector2::new(l, y)
        )
    }
}

impl<T: BaseFloat> GenSquareMat<T, Vector2<T>> for Matrix2<T> {
    #[inline(always)]
    fn determinant(&self) -> T {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
    #[inline]
    fn inverse(&self) -> Option<Matrix2<T>> {
        let det = self.determinant();
        let ling = T::zero();
        if det.is_approx_eq(&ling) {
            None
        } else {
            let inv_det = det.recip();
            let m = Matrix2::new(
                Vector2::new(self[1][1] * inv_det, -self[0][1] * inv_det),
                Vector2::new(-self[1][0] * inv_det, self[0][0] * inv_det)
            );
            Some(m)
        }
    }
}

impl<T: BaseFloat> One for Matrix3<T> {
    #[inline]
    fn one() -> Matrix3<T> {
        let y = T::one();
        let l = T::zero();
        Matrix3::new(
            Vector3::new(y, l, l),
            Vector3::new(l, y, l),
            Vector3::new(l, l, y)
        )
    }
}

impl<T: BaseFloat> GenSquareMat<T, Vector3<T>> for Matrix3<T> {
    #[inline]
    fn determinant(&self) -> T {
        self[0][0] * (self[1][1] * self[2][2] - self[2][1] * self[1][2]) -
        self[1][0] * (self[0][1] * self[2][2] - self[2][1] * self[0][2]) +
        self[2][0] * (self[0][1] * self[1][2] - self[1][1] * self[0][2])
    }
    #[inline]
    fn inverse(&self) -> Option<Matrix3<T>> {
        let det = self.determinant();
        let ling = T::zero();
        if det.is_approx_eq(&ling) {
            None
        } else {
            let inv_det = det.recip();
            let r11 = self[1][1] * self[2][2] - self[2][1] * self[1][2];
            let r12 = self[2][0] * self[1][2] - self[1][0] * self[2][2];
            let r13 = self[1][0] * self[2][1] - self[2][0] * self[1][1];
            let r21 = self[2][1] * self[0][2] - self[0][1] * self[2][2];
            let r22 = self[0][0] * self[2][2] - self[2][0] * self[0][2];
            let r23 = self[2][0] * self[0][1] - self[0][0] * self[2][1];
            let r31 = self[0][1] * self[1][2] - self[1][1] * self[0][2];
            let r32 = self[1][0] * self[0][2] - self[0][0] * self[1][2];
            let r33 = self[0][0] * self[1][1] - self[1][0] * self[0][1];
            let m = Matrix3::new(
                Vector3::new(r11 * inv_det, r21 * inv_det, r31 * inv_det),
                Vector3::new(r12 * inv_det, r22 * inv_det, r32 * inv_det),
                Vector3::new(r13 * inv_det, r23 * inv_det, r33 * inv_det)
            );
            Some(m)
        }
    }
}

impl<T: BaseFloat> One for Matrix4<T> {
    #[inline]
    fn one() -> Matrix4<T> {
        let y = T::one();
        let l = T::zero();
        Matrix4::new(
            Vector4::new(y, l, l, l),
            Vector4::new(l, y, l, l),
            Vector4::new(l, l, y, l),
            Vector4::new(l, l, l, y)
        )
    }
}

impl<T: BaseFloat> GenSquareMat<T, Vector4<T>> for Matrix4<T> {
    #[inline]
    fn determinant(&self) -> T {
        self[0][0] * (
            self[1][1] * self[2][2] * self[3][3] +
            self[2][1] * self[3][2] * self[1][3] +
            self[3][1] * self[1][2] * self[2][3] -
            self[3][1] * self[2][2] * self[1][3] -
            self[1][1] * self[3][2] * self[2][3] -
            self[2][1] * self[1][2] * self[3][3]
        ) -
        self[1][0] * (
            self[0][1] * self[2][2] * self[3][3] +
            self[2][1] * self[3][2] * self[0][3] +
            self[3][1] * self[0][2] * self[2][3] -
            self[3][1] * self[2][2] * self[0][3] -
            self[0][1] * self[3][2] * self[2][3] -
            self[2][1] * self[0][2] * self[3][3]
        ) +
        self[2][0] * (
            self[0][1] * self[1][2] * self[3][3] +
            self[1][1] * self[3][2] * self[0][3] +
            self[3][1] * self[0][2] * self[1][3] -
            self[3][1] * self[1][2] * self[0][3] -
            self[0][1] * self[3][2] * self[1][3] -
            self[1][1] * self[0][2] * self[3][3]
        ) -
        self[3][0] * (
            self[0][1] * self[1][2] * self[2][3] +
            self[1][1] * self[2][2] * self[0][3] +
            self[2][1] * self[0][2] * self[1][3] -
            self[2][1] * self[1][2] * self[0][3] -
            self[0][1] * self[2][2] * self[1][3] -
            self[1][1] * self[0][2] * self[2][3]
        )
    }
    #[inline]
    fn inverse(&self) -> Option<Matrix4<T>> {
        let det = self.determinant();
        let ling = T::zero();
        if det.is_approx_eq(&ling) {
            None
        } else {
            let inv_det = det.recip();
            let tr = self.transpose();
            let cf = |i, j| -> T {
                let mat = match i {
                    0 => Matrix3::new(
                        tr.c1.truncate(j),
                        tr.c2.truncate(j),
                        tr.c3.truncate(j)
                    ),
                    1 => Matrix3::new(
                        tr.c0.truncate(j),
                        tr.c2.truncate(j),
                        tr.c3.truncate(j)
                    ),
                    2 => Matrix3::new(
                        tr.c0.truncate(j),
                        tr.c1.truncate(j),
                        tr.c3.truncate(j)
                    ),
                    3 => Matrix3::new(
                        tr.c0.truncate(j),
                        tr.c1.truncate(j),
                        tr.c2.truncate(j)
                    ),
                    _ => unreachable!(),
                };
                let d = mat.determinant() * inv_det;
                if (i + j) & 1 == 1 {
                    -d
                } else {
                    d
                }
            };
            let m = Matrix4::new(
                Vector4::new(cf(0, 0), cf(0, 1), cf(0, 2), cf(0, 3)),
                Vector4::new(cf(1, 0), cf(1, 1), cf(1, 2), cf(1, 3)),
                Vector4::new(cf(2, 0), cf(2, 1), cf(2, 2), cf(2, 3)),
                Vector4::new(cf(3, 0), cf(3, 1), cf(3, 2), cf(3, 3))
            );
            Some(m)
        }
    }
}

#[cfg(test)]
mod test {

    use basenum::*;
    use mat::traits::GenSquareMat;
    use mat::mat::*;
    use mat::ctor::*;
    use num::{ One, Zero };

    #[test]
    fn test_determinant() {
        let m2 = mat2(4., 5., 6., 7.);
        assert_eq!(m2.determinant(), -2.);
        assert_eq!(Mat3::one().determinant(), 1.);
        let m4 = mat4(
            1., 0., 4., 0.,
            2., 1., 2., 1.,
            3., 2., 3., 1.,
            4., 3., 0., 0.
        );
        assert_eq!(m4.determinant(), -7.);
        assert_eq!((m4 * m4).determinant(), 49.);
        assert_eq!(Mat4::one().determinant(), 1.);
    }

    #[test]
    fn test_inverse_mat2() {
        let yi = Mat2::one();
        assert!(yi.inverse().is_some());
        assert!(DMat2::zero().inverse().is_none());
        let mat = mat2(1., 3., 2., 4.);
        let inv = mat.inverse().unwrap();
        assert_close_to!(mat * inv, yi, 0.000001);
        assert_close_to!(inv * mat, yi, 0.000001);
        let m2 = mat2(1., 2., 2., 4.);
        assert!(m2.inverse().is_none());
    }

    #[test]
    fn test_inverse_mat3() {
        let yi = Mat3::one();
        assert!(yi.inverse().is_some());
        assert_eq!(yi.inverse().unwrap(), yi);
        assert!(DMat3::zero().inverse().is_none());
        let mat = mat3(5., 7., 11., -6., 9., 2., 1., 13., 0.);
        let inv = mat.inverse().unwrap();
        assert_close_to!(mat * inv, yi, 0.000001);
        assert_close_to!(inv * mat, yi, 0.000001);
    }

    #[test]
    fn test_inverse_mat4() {
        let mat = mat4(
            1., 0., 4., 0.,
            2., 1., 2., 1.,
            3., 2., 3., 1.,
            4., 3., 0., 0.
        );
        let invm = mat4(
            3./7., 12./7., -12./7., 4./7.,
            -4./7., -16./7., 16./7., -3./7.,
            1./7., -3./7., 3./7., -1./7.,
            -4./7., 5./7., 2./7., -3./7.
        );
        assert_approx_eq!(mat.inverse().unwrap(), invm);
        assert_close_to!(mat.inverse().unwrap().inverse().unwrap(), mat, 0.000001);
        assert!(Mat4::one().inverse().is_some());
    }
}
