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

use basenum::{ BaseNum, BaseFloat, Primitive };
use traits::{ GenBType, GenFloat, GenNum };
use std::ops::{ Index, IndexMut };

/// Generic vector type.
pub trait GenVec<T: Primitive>
: Copy
+ Index<usize, Output = T>
+ IndexMut<usize, Output = T>
{
    /// Returns the dimension of the vector.
    ///
    /// # Example
    ///
    /// ```
    /// use glm::GenVec;    // bring the method into scope.
    /// assert_eq!(glm::IVec4::dim(), 4);
    /// ```
    fn dim() -> usize;

    // XXX: swap(i, j)
}

/// Trait of all vector types that are GenNum.
pub trait GenNumVec<T: BaseNum>: GenNum<T> + GenVec<T> {

    /// Returns the sum of all components.
    //
    /// # Example
    ///
    /// ```
    /// use glm::GenNumVec;     // bring the method into scope.
    /// let v = glm::vec3(1., 2., 3.);
    /// assert_eq!(v.sum(), 6.0);
    /// ```
    fn sum(&self) -> T;

    /// Multiplies all components.
    ///
    /// # Example
    ///
    /// ```
    /// use glm::GenNumVec;     // bring the method into scope.
    /// let v = glm::vec3(2., 3., 4.);
    /// assert_eq!(v.product(), 24.);
    /// ```
    fn product(&self) -> T;

    /// Returns the minimal value of all components.
    ///
    /// # Example
    ///
    /// ```
    /// use glm::GenNumVec;     // bring the method into scope.
    /// let v = glm::vec3(1.0, 2.0, 3.0);
    /// assert_eq!(v.min(), 1.0);
    /// ```
    fn min(&self) -> T;

    /// Returns the maximal value of all components.
    ///
    /// # Example
    ///
    /// ```
    /// use glm::GenNumVec;     // bring the method into scope.
    ///
    /// let v = glm::vec3(1.0, 2.0, 3.0);
    /// assert_eq!(v.max(), 3.0);
    /// ```
    fn max(&self) -> T;
}

/// Generic type of vectors of float number.
pub trait GenFloatVec<T: BaseFloat>: GenNumVec<T> + GenFloat<T> {}

/// Generic boolean vector type.
pub trait GenBVec: GenVec<bool> + GenBType {

    /// Returns `true` if there is any component of the receiver is `true`.
    ///
    /// # Example
    ///
    /// ```
    /// use glm::{ GenBVec, bvec2 };
    ///
    /// assert!(bvec2(true, false).any());
    /// assert_eq!(bvec2(false, false).all(), false);
    /// ```
    fn any(&self) -> bool;

    /// Returns `true` if all components of the receiver are `true`.
    ///
    /// # Example
    ///
    /// ```
    /// use glm::{ GenBVec, bvec2 };
    ///
    /// assert_eq!(bvec2(true, false).all(), false);
    /// assert_eq!(bvec2(true, true).all(), true);
    /// ```
    fn all(&self) -> bool;

    /// Returns the component-wise logical complement of the receiver.
    ///
    /// # Example
    ///
    /// ```
    /// use glm::{ GenBVec, bvec2 };
    ///
    /// assert_eq!(bvec2(true, false).not(), bvec2(false, true));
    /// ```
    fn not(&self) -> Self;
}
