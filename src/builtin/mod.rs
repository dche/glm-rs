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

//!
//! Built-in funcions defined in GLSL specification chapter 8.
//!
//!

pub use self::trig::{
    radians, degrees,
    sin, cos, tan, asin, acos, atan2, atan,
    sinh, cosh, tanh, asinh, acosh, atanh,
};

pub use self::exp::{
    pow, exp, log, exp2, log2, sqrt, inversesqrt
};

pub use self::common::{
    abs, sign,
    floor, trunc, round, roundEven, ceil, fract, fmod, mod_s, modf,
    min, min_s, max, max_s, clamp, clamp_s,
    mix, mix_s, mix_bool, step, step_s, smoothstep, smoothstep_s,
    isnan, isinf,
    floatBitsToInt, floatBitsToUint, intBitsToFloat, uintBitsToFloat,
    fma,
    frexp, ldexp,
};

pub use self::pack::{
    packUnorm2x16, packUnorm4x8, packSnorm2x16, packSnorm4x8,
    unpackUnorm2x16, unpackUnorm4x8, unpackSnorm2x16, unpackSnorm4x8,
    packDouble2x32, unpackDouble2x32,
};

pub use self::geom::{
    dot, length, distance, normalize, faceforward, reflect, refract, cross,
};

pub use self::matrix::{
    matrixCompMult, outerProduct, transpose, determinant, inverse,
};

pub use self::vecrel::{
    lessThan, lessThanEqual, greaterThan, greaterThanEqual, equal, notEqual,
    all, any, not,
};

pub use self::integer::{
    uaddCarry, usubBorrow, umulExtended, imulExtended,
    bitfieldExtract, bitfieldInsert, bitfieldReverse, bitCount,
    findLSB, findMSB,
};

pub use self::noise::{
    noise1, noise2, noise3, noise4,
};

mod trig;
mod exp;
mod common;
mod pack;
mod geom;
mod matrix;
mod vecrel;
mod integer;
mod noise;
