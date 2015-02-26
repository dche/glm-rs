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

// The GLSL Specification, ch 8.4, Floating-Point Pack and Unpack Functions.

use vec::vec::*;
use super::common::{ clamp_s, round };
use std::mem;

/// First, converts each component of the normalized floating-point value `v`
/// into 16-bit integer values. Then, the results are packed into the
/// returned 32-bit unsigned integer.
///
/// The conversion for component `c` of `v` to fixed point is done as follows:
/// ```round(clamp(c, 0, 1) * 65535.0)```
///
/// The first component of the vector will be written to the least significant
/// bits of the output; the last component will be written to the most
/// significant bits.
///
/// # Example
///
/// ```
///
/// ```
#[inline]
#[allow(non_snake_case)]
pub fn packUnorm2x16(v: Vec2) -> u32 {
    let us = round(clamp_s(v, 0., 1.) * 65535.);
    let pack: [u16; 2] = [us.y as u16, us.x as u16];
    let r: &u32 = unsafe { mem::transmute(&pack) };
    *r
}

/// First, unpacks a single 32-bit unsigned integer `p` into a pair of 16-bit
/// unsigned integers. Then, each component is converted to a normalized
/// floating-point value to generate the returned two-component vector.
///
/// The conversion for unpacked fixed-point value `f` to floating point is done
/// as follows: `f / 65535.0`.
///
/// The first component of the returned vector will be extracted from the least
/// significant bits of the input; the last component will be extracted from
/// the most significant bits.
///
/// # Example
///
/// ```
///
/// ```
#[inline]
#[allow(non_snake_case)]
pub fn unpackUnorm2x16(p: u32) -> Vec2 {
    let unpack: &[u16; 2] = unsafe { mem::transmute(&p) };
    let v = vec2(unpack[1] as f32, unpack[0] as f32);
    // v / 65535.
    v * 1.5259021896696421759365224689097e-5
}

/// First, converts each component of the normalized floating-point value `v`
/// into 8-bit integer values. Then, the results are packed into the
/// returned 32-bit unsigned integer.
///
/// The conversion for component `c` of `v` to fixed point is done as follows:
/// ```round(clamp(c, 0, 1) * 255.0)```
///
/// The first component of the vector will be written to the least significant
/// bits of the output; the last component will be written to the most
/// significant bits.
///
/// # Example
///
/// ```
///
/// ```
#[inline]
#[allow(non_snake_case)]
pub fn packUnorm4x8(v: Vec4) -> u32 {
    let us = round(clamp_s(v, 0., 1.) * 255.);
    let pack: [u8; 4] = [us.w as u8, us.z as u8, us.y as u8, us.x as u8];
    let r: &u32 = unsafe { mem::transmute(&pack) };
    *r
}

/// First, unpacks a single 32-bit unsigned integer `p` into four 8-bit unsigned
/// integers. Then, each component is converted to a normalized floating-point
/// value to generate the returned four-component vector.
///
/// The conversion for unpacked fixed-point value `f` to floating point is done
/// as follows: `f / 255.0`.
///
/// The first component of the returned vector will be extracted from the least
/// significant bits of the input; the last component will be extracted from
/// the most significant bits.
///
/// # Example
///
/// ```
///
/// ```
#[inline]
#[allow(non_snake_case)]
pub fn unpackUnorm4x8(p: u32) -> Vec4 {
    let unpack: &[u8; 4] = unsafe { mem::transmute(&p) };
    let v =
        vec4(
            unpack[3] as f32,
            unpack[2] as f32,
            unpack[1] as f32,
            unpack[0] as f32
        );
    // v / 255.
    v * 0.0039215686274509803921568627451
}

/// First, converts each component of the normalized floating-point value `v`
/// into 16-bit integer values. Then, the results are packed into the
/// returned 32-bit unsigned integer.
///
/// The conversion for component `c` of `v` to fixed point is done as follows:
/// ```round(clamp(c, -1, 1) * 32767.0)```
///
/// The first component of the vector will be written to the least significant
/// bits of the output; the last component will be written to the most
/// significant bits.
///
/// # Example
///
/// ```
///
/// ```
#[inline]
#[allow(non_snake_case)]
pub fn packSnorm2x16(v: Vec2) -> u32 {
    let is = round(clamp_s(v, -1., 1.) * 32767.);
    let pack: [i16; 2] = [is.y as i16, is.x as i16];
    let r: &u32 = unsafe { mem::transmute(&pack) };
    *r
}

/// First, unpacks a single 32-bit unsigned integer `p` into two 16-bit signed
/// integers. Then, each component is converted to a normalized floating-point
/// value to generate the returned two-component vector.
///
/// The conversion for unpacked fixed-point value `f` to floating point is
/// done as follows: `clamp(f / 32767.0, -1, +1)`
///
/// The first component of the returned vector will be extracted from the
/// least significant bits of the input; the last component will be extracted
/// from the most significant bits.
///
/// # Example
///
/// ```
///
/// ```
#[inline]
#[allow(non_snake_case)]
pub fn unpackSnorm2x16(p: u32) -> Vec2 {
    let unpack: &[i16; 2] = unsafe { mem::transmute(&p) };
    let v = vec2(unpack[1] as f32, unpack[0] as f32);
    // v / 32767.
    clamp_s(v * 3.0518509475997192297128208258309e-5, -1., 1.)
}

/// First, converts each component of the normalized floating-point value `v`
/// into 8-bit integer values. Then, the results are packed into the
/// returned 32-bit unsigned integer.
///
/// The conversion for component `c` of `v` to fixed point is done as follows:
/// ```round(clamp(c, -1, 1) * 127.0)```
///
/// The first component of the vector will be written to the least significant
/// bits of the output; the last component will be written to the most
/// significant bits.
///
/// # Example
///
/// ```
///
/// ```
#[inline]
#[allow(non_snake_case)]
pub fn packSnorm4x8(v: Vec4) -> u32 {
    let is = round(clamp_s(v, -1., 1.) * 127.);
    let pack: [i8; 4] = [is.w as i8, is.z as i8, is.y as i8, is.x as i8];
    let r: &u32 = unsafe { mem::transmute(&pack) };
    *r
}

/// First, unpacks a single 32-bit unsigned integer `p` into four 8-bit signed
/// integers. Then, each component is converted to a normalized floating-point
/// value to generate the returned four-component vector.
///
/// The conversion for unpacked fixed-point value `f` to floating point is
/// done as follows: `clamp(f / 127.0, -1, +1)`
///
/// The first component of the returned vector will be extracted from the
/// least significant bits of the input; the last component will be extracted
/// from the most significant bits.
///
/// # Example
///
/// ```
///
/// ```
#[inline]
#[allow(non_snake_case)]
pub fn unpackSnorm4x8(p: u32) -> Vec4 {
    let unpack: &[i8; 4] = unsafe { mem::transmute(&p) };
    let v =
        vec4(
            unpack[3] as f32,
            unpack[2] as f32,
            unpack[1] as f32,
            unpack[0] as f32
        );
    // v / 127.
    clamp_s(v * 0.0078740157480315, -1., 1.)

}

/// Returns a double-precision value obtained by packing the components of `v`
/// into a 64-bit value.
///
/// If an IEEE 754 Iinf or **NaN** is created, it will not signal, and the
/// resulting floating point value is unspecified. Otherwise, the bit-level
/// representation of `v` is preserved. The first vector component specifies
/// the 32 least significant bits; the second component specifies the 32 most
/// significant bits.
///
/// # Example
///
/// ```
///
/// ```
#[allow(non_snake_case)]
#[inline(always)]
pub fn packDouble2x32(v: UVec2) -> f64 {
    let f: &f64 = unsafe { mem::transmute(&v) };
    *f
}

/// Returns a two-component unsigned integer vector representation of `v`.
/// The bit-level representation of `v` is preserved.
///
/// The first component of the vector contains the 32 least significant bits
/// of the double; the second component consists the 32 most significant bits.
///
/// # Example
///
/// ```
///
/// ```
#[allow(non_snake_case)]
#[inline(always)]
pub fn unpackDouble2x32(v: f64) -> UVec2 {
    let uv: &UVec2 = unsafe { mem::transmute(&v) };
    *uv
}
