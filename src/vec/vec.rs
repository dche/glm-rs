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

use basenum::*;
use traits::*;
use super::traits::{ GenVec, GenNumVec, GenFloatVec, GenBVec };
use std::cmp::Eq;
use std::mem;
use std::ops::{
    Add, Mul, Sub, Neg, Div, Rem, Not, BitAnd, BitOr, BitXor, Shl, Shr,
    Index, IndexMut,
};
use rand::{ Rand, Rng };
use num::{ Float, One, Zero };
#[cfg(test)]
use quickcheck::{ Arbitrary, Gen };

// copied from `cgmath-rs/src/vector.rs`.
macro_rules! fold(
    ($m: ident, { $x: expr, $y: expr }) => {
        $x.$m($y)
    };
    ($m: ident, { $x: expr, $y: expr, $z: expr }) => {
        $x.$m($y).$m($z)
    };
    ($m: ident, { $x: expr, $y: expr, $z: expr, $w: expr }) => {
        $x.$m($y).$m($z).$m($w)
    };
);

macro_rules! def_genvec(
    (
        $t: ident,          // name of the type to be defined,
        $n: expr,           // dimension (2, 3, or 4),
        $($field: ident),+  // list of field names (e.g., "x, y, z").
    ) => {
        #[repr(C)]
        #[derive(Copy, Clone, PartialEq, Debug)]
        pub struct $t<T: Primitive> {
            $(pub $field: T),+
        }
        impl<T: Primitive> $t<T> {
            #[inline(always)]
            pub fn new($($field: T),+) -> $t<T> {
                $t { $($field: $field),+ }
            }
            #[inline(always)]
            pub fn from_array(ary: &[T; $n]) -> &$t<T> {
                let r: &$t<T> = unsafe { mem::transmute(ary) };
                r
            }
            #[inline(always)]
            pub fn from_array_mut(ary: &mut [T; $n]) -> &mut $t<T> {
                let r: &mut $t<T> = unsafe { mem::transmute(ary) };
                r
            }
            #[inline(always)]
            pub fn as_array(&self) -> &[T; $n] {
                let ary: &[T; $n] = unsafe { mem::transmute(self) };
                ary
            }
            #[inline(always)]
            pub fn as_array_mut(&mut self) -> &mut [T; $n] {
                let ary: &mut [T; $n] = unsafe { mem::transmute(self) };
                ary
            }
        }
        impl<T: Primitive> GenVec<T> for $t<T> {
            #[inline(always)]
            fn dim() -> usize { $n }
        }
        impl<T: Primitive> Index<usize> for $t<T> {
            type Output = T;
            #[inline(always)]
            fn index<'a>(&'a self, i: usize) -> &'a T {
                self.as_array().index(i)
            }
        }
        impl<T: Primitive> IndexMut<usize> for $t<T> {
            #[inline(always)]
            fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut T {
                self.as_array_mut().index_mut(i)
            }
        }
        impl<T: Primitive> Rand for $t<T> {
            #[inline]
            fn rand<R: Rng>(rng: &mut R) -> $t<T> {
                $t {$($field: rng.gen()),+}
            }
        }
        #[cfg(test)]
        impl<T: Primitive + Arbitrary> Arbitrary for $t<T> {
            fn arbitrary<G: Gen>(g: &mut G) -> $t<T> {
                // do not use `g.size()`.
                g.gen()
            }
        }
        impl Eq for $t<bool> {}
        impl GenBType for $t<bool> {}
        impl GenBVec for $t<bool> {
            #[inline(always)]
            fn all(&self) -> bool {
                $(self.$field) && +
            }
            #[inline(always)]
            fn any(&self) -> bool {
                $(self.$field) || +
            }
            #[inline(always)]
            fn not(&self) -> $t<bool> {
                $t::new($(!self.$field),+)
            }
        }
        impl<T: BaseNum> Add<$t<T>> for $t<T> {
            type Output = $t<T>;
            #[inline(always)]
            fn add(self, rhs: $t<T>) -> $t<T> {
                $t::new($(self.$field + rhs.$field),+)
            }
        }
        impl<T: BaseNum> Add<T> for $t<T> {
            type Output = $t<T>;
            #[inline(always)]
            fn add(self, rhs: T) -> $t<T> {
                $t::new($(self.$field + rhs),+)
            }
        }
        impl<T: BaseNum> Mul<$t<T>> for $t<T> {
            type Output = $t<T>;
            #[inline(always)]
            fn mul(self, rhs: $t<T>) -> $t<T> {
                $t::new($(self.$field * rhs.$field),+)
            }
        }
        impl<T: BaseNum> Mul<T> for $t<T> {
            type Output = $t<T>;
            #[inline(always)]
            fn mul(self, rhs: T) -> $t<T> {
                $t::new($(self.$field * rhs),+)
            }
        }
        impl<T: BaseNum> Div<$t<T>> for $t<T> {
            type Output = $t<T>;
            #[inline(always)]
            fn div(self, rhs: $t<T>) -> $t<T> {
                $t::new($(self.$field / rhs.$field),+)
            }
        }
        impl<T: BaseNum> Div<T> for $t<T> {
            type Output = $t<T>;
            #[inline(always)]
            fn div(self, rhs: T) -> $t<T> {
                $t::new($(self.$field / rhs),+)
            }
        }
        impl<T: BaseNum> Rem<$t<T>> for $t<T> {
            type Output = $t<T>;
            #[inline(always)]
            fn rem(self, rhs: $t<T>) -> $t<T> {
                $t::new($(self.$field % rhs.$field),+)
            }
        }
        impl<T: BaseNum> Rem<T> for $t<T> {
            type Output = $t<T>;
            #[inline(always)]
            fn rem(self, rhs: T) -> $t<T> {
                $t::new($(self.$field % rhs),+)
            }
        }
        impl<T: BaseNum> One for $t<T> {
            #[inline(always)]
            fn one() -> $t<T> {
                let y = T::one();
                $t { $($field: y),+ }
            }
        }
        impl<T: BaseNum> Zero for $t<T> {
            #[inline(always)]
            fn zero() -> $t<T> {
                let l = T::zero();
                $t { $($field: l),+ }
            }
            #[inline]
            fn is_zero(&self) -> bool {
                let l = T::zero();
                $(self.$field == l) && +
            }
        }
        impl<T: BaseNum> GenNum<T> for $t<T> {
            #[inline(always)]
            fn from_s(x: T) -> Self {
                $t { $($field: x),+ }
            }
            #[inline(always)]
            fn map<F: Fn(T) -> T>(self, f: F) -> $t<T> {
                $t::new($(f(self.$field)),+)
            }
            #[inline(always)]
            fn zip<F: Fn(T, T) -> T>(self, y: $t<T>, f: F) -> $t<T> {
                $t::new($(f(self.$field, y.$field)),+)
            }
            #[inline]
            fn split<F: Fn(T) -> (T, T)>(self, f: F) -> ($t<T>, $t<T>) {
                let ling = $t::<T>::zero();
                let mut a = ling;
                let mut b = ling;
                let dim = $t::<T>::dim();
                for i in 0..dim {
                    let (c, d) = f(self[i]);
                    a[i] = c;
                    b[i] = d;
                }
                (a, b)
            }
            #[inline]
            fn map2<F: Fn(T, T) -> (T, T)>(self, y: Self, f: F) -> (Self, Self) {
                let ling = Self::zero();
                let mut a = ling;
                let mut b = ling;
                let dim = Self::dim();
                for i in 0..dim {
                    let (c, d) = f(self[i], y[i]);
                    a[i] = c;
                    b[i] = d;
                }
                (a, b)
            }
        }
        impl<T: BaseNum> GenNumVec<T> for $t<T> {
            #[inline(always)]
            fn sum(&self) -> T {
                fold!(add, { $(self.$field),+ })
            }
            #[inline(always)]
            fn product(&self) -> T {
                fold!(mul, { $(self.$field),+ })
            }
            #[inline(always)]
            fn min(&self) -> T {
                fold!(min, { $(self.$field),+ })
            }
            #[inline(always)]
            fn max(&self) -> T {
                fold!(max, { $(self.$field),+ })
            }
        }
        impl<T: SignedNum + BaseNum> Neg for $t<T> {
            type Output = $t<T>;
            #[inline]
            fn neg(self) -> $t<T> {
                $t::new($(-(self.$field)),+)
            }
        }
        impl<T: SignedNum + BaseNum> Sub<$t<T>> for $t<T> {
            type Output = $t<T>;
            #[inline(always)]
            fn sub(self, rhs: $t<T>) -> $t<T> {
                $t::new($(self.$field - rhs.$field),+)
            }
        }
        impl<T: SignedNum + BaseNum> Sub<T> for $t<T> {
            type Output = $t<T>;
            #[inline(always)]
            fn sub(self, rhs: T) -> $t<T> {
                $t::new($(self.$field - rhs),+)
            }
        }
        impl<T: SignedNum + BaseNum> SignedNum for $t<T> {
            #[inline]
            fn abs(&self) -> $t<T> {
                $t::new($(self.$field.abs()),+)
            }
            #[inline]
            fn sign(&self) -> $t<T> {
                $t::new($(self.$field.sign()),+)
            }
        }
        impl<T: BaseInt> Eq for $t<T> {}
        impl<T: BaseInt> Not for $t<T> {
            type Output = Self;
            #[inline]
            fn not(self) -> Self {
                $t::new($(!self.$field),+)
            }
        }
        impl<T: BaseInt> BitAnd<T> for $t<T> {
            type Output = Self;
            #[inline]
            fn bitand(self, rhs: T) -> Self {
                $t::new($(self.$field & rhs),+)
            }
        }
        impl<T: BaseInt> BitAnd<$t<T>> for $t<T> {
            type Output = Self;
            #[inline]
            fn bitand(self, rhs: Self) -> Self {
                $t::new($(self.$field & rhs.$field),+)
            }
        }
        impl<T: BaseInt> BitOr<T> for $t<T> {
            type Output = Self;
            #[inline]
            fn bitor(self, rhs: T) -> Self {
                $t::new($(self.$field | rhs),+)
            }
        }
        impl<T: BaseInt> BitOr<$t<T>> for $t<T> {
            type Output = Self;
            #[inline]
            fn bitor(self, rhs: Self) -> Self {
                $t::new($(self.$field | rhs.$field),+)
            }
        }
        impl<T: BaseInt> BitXor<T> for $t<T> {
            type Output = Self;
            #[inline]
            fn bitxor(self, rhs: T) -> Self {
                $t::new($(self.$field ^ rhs),+)
            }
        }
        impl<T: BaseInt> BitXor<$t<T>> for $t<T> {
            type Output = Self;
            #[inline]
            fn bitxor(self, rhs: Self) -> Self {
                $t::new($(self.$field ^ rhs.$field),+)
            }
        }
        impl<T: BaseInt> Shl<usize> for $t<T> {
            type Output = Self;
            #[inline]
            fn shl(self, rhs: usize) -> Self {
                $t::new($(self.$field << rhs),+)
            }
        }
        impl<T: BaseInt> Shr<usize> for $t<T> {
            type Output = Self;
            #[inline]
            fn shr(self, rhs: usize) -> Self {
                $t::new($(self.$field >> rhs),+)
            }
        }
        impl<T: BaseInt> GenInt<T> for $t<T> {}
        impl GenIType for $t<i32> {}
        impl GenUType for $t<u32> {}
        impl<T: BaseFloat> ApproxEq for $t<T> {
            type BaseType = T;
            #[inline]
            fn is_close_to(&self, rhs: &$t<T>, max_diff: T) -> bool {
                $(self.$field.is_close_to(&rhs.$field, max_diff)) && +
            }
        }
        impl<T: BaseFloat> GenFloat<T> for $t<T> {
            fn fma(&self, b: &$t<T>, c: &$t<T>) -> $t<T> {
                $t::new($(Float::mul_add(self.$field, b.$field, c.$field)),+)
            }
        }
        impl<T: BaseFloat> GenFloatVec<T> for $t<T> {}
        impl GenType for $t<f32> {}
        impl GenDType for $t<f64> {}
    }
);

def_genvec! { Vector2, 2, x, y }
def_genvec! { Vector3, 3, x, y, z }
def_genvec! { Vector4, 4, x, y, z, w }

impl<T: Primitive> Vector2<T> {
    /// Extends _self_ to a `Vector3` by appending `z`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use glm::*;
    ///
    /// let v2 = vec2(1., 2.);
    /// let v3 = vec3(1., 2., 3.);
    /// assert_eq!(v2.extend(3.), v3);
    /// ```
    #[inline]
    pub fn extend(&self, z: T) -> Vector3<T> {
        Vector3 { x: self.x, y: self.y, z: z }
    }
}

impl<T: Primitive> Vector3<T> {
    /// Extends _self_ to a `Vector4` by appending `w`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use glm::*;
    ///
    /// let v3 = vec3(1., 2., 3.);
    /// let v4 = vec4(1., 2., 3., 4.);
    /// assert_eq!(v3.extend(4.), v4);
    /// ```
    #[inline]
    pub fn extend(&self, w: T) -> Vector4<T> {
        Vector4 { x: self.x, y: self.y, z: self.z, w: w }
    }

    /// Truncates _self_ to a `Vector2` by remove the `i`<sub>th</sub> element.
    ///
    /// Parameter `i` is `0` based index.
    ///
    /// # Panic
    ///
    /// It is a panic if i is larger than `2`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use glm::*;
    ///
    /// let v3 = vec3(1., 2., 3.);
    /// let v2 = vec2(1., 3.);
    /// assert_eq!(v3.truncate(1), v2);
    /// ```
    #[inline]
    pub fn truncate(&self, i: usize) -> Vector2<T> {
        match i {
            0 => Vector2::new(self.y, self.z),
            1 => Vector2::new(self.x, self.z),
            2 => Vector2::new(self.x, self.y),
            _ => panic!("parameter i is out of range [{:?} > 2].", i)
        }
    }
}

impl<T: Primitive> Vector4<T> {
    /// Truncates _self_ to a `Vector3` by remove the `i`<sub>th</sub> element.
    ///
    /// Parameter `i` is `0` based index.
    ///
    /// # Panic
    ///
    /// It is a panic if i is larger than `3`.
    #[inline]
    pub fn truncate(&self, i: usize) -> Vector3<T> {
        match i {
            0 => Vector3::new(self.y, self.z, self.w),
            1 => Vector3::new(self.x, self.z, self.w),
            2 => Vector3::new(self.x, self.y, self.w),
            3 => Vector3::new(self.x, self.y, self.z),
            _ => panic!("parameter i is out of range [{:?} > 3].", i)
        }
    }
}

macro_rules! def_alias(
    (
        $({
            $a: ident,          // type alias (e.g., Vec2 for Vector2<f32>),
            $t: ident,          // type to be aliased,
            $et: ty,            // element type,
            $ctor: ident,       // constructor name (e.g., vec3),
            $($field: ident),+  // fields,
        }),+
    ) => {
        $(
            pub type $a = $t<$et>;
            #[inline(always)]
            pub fn $ctor($($field: $et),+) -> $t<$et> {
                $t::new($($field),+)
            }
        )+
    }
);

def_alias! {
    { BVec2, Vector2, bool, bvec2, x, y },
    { BVec3, Vector3, bool, bvec3, x, y, z },
    { BVec4, Vector4, bool, bvec4, x, y, z, w },

    { Vec2, Vector2, f32, vec2, x, y },
    { Vec3, Vector3, f32, vec3, x, y, z },
    { Vec4, Vector4, f32, vec4, x, y, z, w },

    { DVec2, Vector2, f64, dvec2, x, y },
    { DVec3, Vector3, f64, dvec3, x, y, z },
    { DVec4, Vector4, f64, dvec4, x, y, z, w },

    { IVec2, Vector2, i32, ivec2, x, y },
    { IVec3, Vector3, i32, ivec3, x, y, z },
    { IVec4, Vector4, i32, ivec4, x, y, z, w },

    { UVec2, Vector2, u32, uvec2, x, y },
    { UVec3, Vector3, u32, uvec3, x, y, z },
    { UVec4, Vector4, u32, uvec4, x, y, z, w }
}

#[cfg(test)]
mod test {

    use super::*;
    use num::One;
    use quickcheck::*;

    #[test]
    fn test_as_array() {
        fn prop(v3: Vec3) -> bool {
            let ary: &[f32; 3] = v3.as_array();
            ary[0] == v3.x &&
            ary[1] == v3.y &&
            ary[2] == v3.z
        }
        quickcheck(prop as fn(Vec3) -> bool);
    }

    #[test]
    fn test_as_array_mut() {
        fn prop(v2: DVec2) -> bool {
            let DVec2 { x, y } = v2;
            let mut v = v2;
            let ary: &mut [f64; 2] = v.as_array_mut();
            ary[0] = x + 1.;
            ary[1] = y * 2.;
            (x + 1.) == ary[0] &&
            (y * 2.) == ary[1]
        }
        quickcheck(prop as fn(DVec2) -> bool)
    }

    #[test]
    fn test_index() {
        fn prop(v3: Vec3) -> bool {
            v3[0] == v3.x &&
            v3[1] == v3.y &&
            v3[2] == v3.z
        }
        quickcheck(prop as fn(Vec3) -> bool);
    }

    #[test]
    fn test_index_mut() {
        fn prop(iv: IVec3) -> bool {
            let mut miv = iv;
            miv[0] = iv.x + 1;
            miv[1] = iv.y + 1;
            miv[2] = iv.z + 1;
            miv == iv + IVec3::one()
        }
        quickcheck(prop as fn(IVec3) -> bool);
    }
}
