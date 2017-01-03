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

// The GLSL Specification, ch 8.8, Integer Functions.

use basenum::BaseInt;
use traits::{ GenNum, GenInt, GenIType, GenUType };
use vec::vec::{ UVec2, UVec3, UVec4, IVec2, IVec3, IVec4 };
use std::mem;

// used by `findLSB` and `findMSB`.
pub trait IntIntRel<I: BaseInt, T: GenIType>: GenInt<I> {
    fn map_int<F: Fn(I) -> i32>(&self, f: F) -> T;
}

macro_rules! impl_IntIntRel_for_int {
    ($($t: ident),+) => {
        $(
            impl IntIntRel<i32, $t> for $t {
                #[inline(always)]
                fn map_int<F: Fn(i32) -> i32>(&self, f: F) -> $t {
                    self.map(f)
                }
            }
        )+
    }
}

impl_IntIntRel_for_int! { i32, IVec2, IVec3, IVec4 }

impl IntIntRel<u32, i32> for u32 {
    #[inline(always)]
    fn map_int<F: Fn(u32) -> i32>(&self, f: F) -> i32 {
        f(*self)
    }
}

macro_rules! impl_IntIntRel_for_uint {
    ($({ $ut: ident, $it: ident, $($field: ident),+ }),+) => {
        $(
            impl IntIntRel<u32, $it> for $ut {
                #[inline(always)]
                fn map_int<F: Fn(u32) -> i32>(&self, f: F) -> $it {
                    $it { $($field: f(self.$field)),+ }
                }
            }
        )+
    }
}

impl_IntIntRel_for_uint! {
    { UVec2, IVec2, x, y },
    { UVec3, IVec3, x, y, z },
    { UVec4, IVec4, x, y, z, w }
}

/// Adds 32-bit unsigned integer `x` and `y`, returning the sum modulus
/// *2<sup>32</sup>* and the carry bit.
///
/// Carry is set to `0` if the sum was less than *2<sup>32</sup>*, or to `1`
/// otherwise.
///
/// # Note
///
/// In GLSL, the carry bit is returned via the output parameter `carry`.
///
/// # Example
///
/// ```
/// use glm::{ uvec2, uaddCarry };
///
/// let v = uvec2(0xFFFFFFFE, 0);
/// assert_eq!(uaddCarry(v, uvec2(3, 3)), (uvec2(1, 3), uvec2(1, 0)))
/// ```
#[inline]
#[allow(non_snake_case)]
pub fn uaddCarry<T: GenUType>(x: T, y: T) -> (T, T) {
    x.map2(y, |i, j| -> (u32, u32) {
        match i.checked_add(j) {
            Some(s) => (s, 0),
            None    => (i - (0xFFFFFFFF - j + 1), 1),
        }
    })
}

/// Subtracts the 32-bit unsigned integer `y` from `x`, returning the
/// difference and the borrow bit.
///
/// Returns the difference if it is non-negative, or *2<sup>32</sup>* plus the
/// difference otherwise.
///
/// The borrow bit is set to `0` if` x ≥ y`, or to `1` otherwise.
///
/// # Example
///
/// ```
/// use glm::{ usubBorrow, uvec2 };
///
/// let uv1 = uvec2(16, 17);
/// let uv2 = uvec2(17, 16);
/// assert_eq!(usubBorrow(uv1, uv2), (uvec2(0xFFFFFFFE, 1), uvec2(1, 0)));
/// ```
#[inline]
#[allow(non_snake_case)]
pub fn usubBorrow<T: GenUType>(x: T, y: T) -> (T, T) {
    x.map2(y, |i, j| -> (u32, u32) {
        if i >= j {
            (i - j, 0)
        } else {
            (0xFFFFFFFF - j + i, 1)
        }
    })
}

/// Multiplies 32-bit unsigned integers `x` and `y`, producing a 64-bit
/// result.
///
/// The 32 least-significant bits are returned in `lsb`.
///
/// The 32 most-significant bits are returned in `msb`.
#[allow(non_snake_case)]
pub fn umulExtended<T: GenUType>(x: T, y: T) -> (T, T) {
    x.map2(y, |i, j| -> (u32, u32) {
        let ei = i as u64;
        let ej = j as u64;
        let p = ei * ej;
        ((p >> 32) as u32, p as u32)
    })
}

/// Multiplies 32-bit integers `x` and `y`, producing a 64-bit result.
///
/// The 32 least-significant bits are returned in `lsb`.
///
/// The 32 most-significant bits are returned in `msb`.
#[allow(non_snake_case)]
pub fn imulExtended<T: GenIType>(x: T, y: T) -> (T, T) {
    x.map2(y, |i, j| -> (i32, i32) {
        let ei = i as i64;
        let ej = j as i64;
        let p = ei * ej;
        ((p >> 32) as i32, p as i32)
    })
}

/// Extracts bits `[offset, offset + bits - 1]` from `value`, returning them in
/// the least significant bits of the result.
///
/// For unsigned data types, the most significant bits of the result will
/// be set to zero. For signed data types, the most significant bits will
/// be set to the value of bit offset + base – 1.
///
/// If `bits` is zero, the result will be zero. The result will be undefined
/// if `offset` or `bits` is negative, or if the sum of `offset` and `bits` is
/// greater than the number of bits used to store the operand.
///
/// # Example
///
/// ```
/// use glm::bitfieldExtract;
///
/// assert_eq!(bitfieldExtract(0xF000FFFF, 32, 12), 0);
/// assert_eq!(bitfieldExtract(0b11100011_u32, 1, 6), 0b110001);
/// ```
#[allow(non_snake_case)]
pub fn bitfieldExtract
<
I: BaseInt,
T: GenInt<I>
>(value: T, offset: usize, bits: usize) -> T {
    let ling = T::zero();
    if value.is_zero() || bits == 0 || offset + bits > 32 {
        ling
    } else {
        let mask = I::from((1_u32 << bits) - 1).unwrap();
        value.map(|i| -> I {
            (i >> offset) & mask
        })
    }
}

/// Returns the insertion the `bits` least-significant bits of `insert` into
/// `base`.
///
/// The result will have bits `[offset, offset + bits - 1]` taken from
/// bits `[0, bits – 1]` of `insert`, and all other bits taken directly from
/// the corresponding bits of `base`. If `bits` is zero, the result will
/// simply be `base`.
///
/// The result will be undefined if `offset` or `bits` is negative,
/// or if the sum of `offset` and `bits` is greater than the number of bits
/// used to store the operand.
///
/// # Example
///
/// ```
/// use glm::bitfieldInsert;
///
/// assert_eq!(bitfieldInsert(1_i32, 0xFF00FF00, 8, 20), 0xF00FF01);
/// ```
#[allow(non_snake_case)]
pub fn bitfieldInsert
<
I: BaseInt,
T: GenInt<I>
>(base: T, insert: T, offset: usize, bits: usize) -> T {
    if bits == 0 {
        base
    } else {
        let mask = I::from(((1_u32 << bits) - 1) << offset).unwrap();
        base.zip(insert, |i, j| -> I {
            (i & !mask) | (j & mask)
        })
    }
}

/// Returns the reversal of the bits of `value`.
///
/// The bit numbered n of the result will be taken from bit `(bits - 1) - n`
/// of `value`, where *bits* is the total number of bits used to represent
/// `value`.
///
/// # Example
///
/// ```
/// use glm::bitfieldReverse;
///
/// assert_eq!(bitfieldReverse(0xF30000F3), 0xCF0000CF);
/// ```
#[allow(non_snake_case)]
pub fn bitfieldReverse<I: BaseInt, T: GenInt<I>>(value: T) -> T {
    #[inline(always)]
    fn reverse_step(x: u32, mask: u32, shift: usize) -> u32 {
        ((x & mask) << shift) | ((x & !mask) >> shift)
    }
    value.map(|i| -> I {
        // reinterpret_cast
        let u: &u32 = unsafe { mem::transmute(&i) };
        let mut x = *u;
        x = reverse_step(x, 0x55555555, 1);
        x = reverse_step(x, 0x33333333, 2);
        x = reverse_step(x, 0x0F0F0F0F, 4);
        x = reverse_step(x, 0x00FF00FF, 8);
        x = reverse_step(x, 0x0000FFFF, 16);
        let r: &I = unsafe { mem::transmute(&x) };
        *r
    })
}

/// Returns the number of bits set to 1 in the binary representation of
/// `value`.
///
/// # Example
///
/// ```
/// use glm::{ bitCount, ivec2 };
///
/// let v = ivec2(0b01010101, 0);
/// assert_eq!(bitCount(v), ivec2(4, 0));
/// ```
#[allow(non_snake_case)]
pub fn bitCount<I: BaseInt, T: GenInt<I>>(value: T) -> T {
    value.map(|i| -> I {
        let c = I::from(i.count_ones()).unwrap();
        c
    })
}

/// Returns the bit number of the least significant bit set to 1 in the binary
/// representation of `value`.
///
/// If `value` is zero, `-1` will be returned.
///
/// # Example
///
/// ```
/// use glm::{ findLSB, ivec2, uvec2 };
///
/// assert_eq!(findLSB(0u32), -1);
/// let v = uvec2(0b0101000, 0x80000000);
/// assert_eq!(findLSB(v), ivec2(3, 31));
/// ```
#[allow(non_snake_case)]
pub fn findLSB<
B: BaseInt,
I: GenIType,
T: IntIntRel<B, I>
>(value: T) -> I {
    value.map_int(|i| -> i32 {
        if i.is_zero() {
            -1
        } else {
            i.trailing_zeros() as i32
        }
    })
}

/// Returns the bit number of the most significant bit in the binary
/// representation of `value`.
///
/// For positive integers, the result will be the bit number of the
/// most significant bit set to `1`.
///
/// For negative integers, the result will be the bit number of the most
/// significant bit set to `0`. For a value of zero or negative one, `-1` will
/// be returned.
///
/// # Example
///
/// ```
/// use glm::{ findMSB, ivec3 };
///
/// assert_eq!(findMSB(0_i32), -1);
/// assert_eq!(findMSB(ivec3(-1, -2, 0x7FFFFFFF)), ivec3(-1, 0, 30));
/// ```
#[allow(non_snake_case)]
pub fn findMSB<
B: BaseInt,
I: GenIType,
T: IntIntRel<B, I>
>(value: T) -> I {
    value.map_int(|i| -> i32 {
        let ling = B::zero();
        if i.is_zero() {
            -1
        } else if i < ling {
            31 - ((!i).leading_zeros() as i32)
        } else {
            31 - (i.leading_zeros() as i32)
        }
    })
}
