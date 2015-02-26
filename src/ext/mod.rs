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

mod trig;
mod exp;
mod common;
mod geom;
mod matrix;
