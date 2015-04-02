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

//! Functions that extend the functionalities of GLSL data types.
//!
//! # Note
//!
//! - Unlike built-in functions, extension functions are not re-exported to
//!   the root module of `glm`. You have to `use glm::ext`.
//! - Also unlike the built-in functions, the naming of extension functions
//!   follows the Rust convention, instead of the GLSL's. That is, snake case
//!   is used.
//! - The parameters of matrix related functions are passed by reference.

pub use self::trig::*;
pub use self::exp::*;
pub use self::common::*;
pub use self::geom::*;
pub use self::matrix::*;
pub use self::consts::{
    Consts,
    epsilon,
    pi,
    tau,
    root_pi,
    half_pi,
    one_third_pi,
    quarter_pi,
    one_over_pi,
    one_over_tau,
    two_over_pi,
    four_over_pi,
    two_over_root_pi,
    one_over_root_two,
    root_half_pi,
    root_tau,
    root_ln_four,
    e,
    euler,
    root_two,
    root_three,
    root_five,
    ln_two,
    ln_ten,
    ln_ln_two,
    one_third,
    two_thirds,
    golden_ratio
};

mod trig;
mod exp;
mod common;
mod geom;
mod matrix;
pub mod consts;
