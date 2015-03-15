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

use basenum::Primitive;
use super::vec::{ Vector2, Vector3, Vector4 };

macro_rules! def_swizzle2 {
    (
        {$($f1: ident, $field1: ident),+},
        {$($f2: ident, {$($field2: ident),+}),+}
    ) => {
        pub trait Swizzle2<T: Primitive> {
            fn x(&self) -> T;
            fn y(&self) -> T;
            $(
                #[inline(always)]
                fn $f1(&self) -> T { self.$field1() }
            )+
            $(
                #[inline(always)]
                fn $f2(&self) -> Vector2<T> {
                    Vector2::new($(self.$field2()),+)
                }
            )+
        }
    }
}

macro_rules! gen_swizzle2 {
    (
        {$($fx: ident),+},
        {$($fy: ident),+},
        {
            $({$($f2x: ident),+}, {$($f2y: ident),+}),+
        }
    ) => {
        def_swizzle2! {
            { $($fx, x),+, $($fy, y),+ },
            {
                $(
                    $(
                        concat_idents!($f2x, $f2y), { $f2x, $f2y }
                    ),+
                )+
            }
        }
    }
}

gen_swizzle2! {
    { r, u },
    { g, v },
    {
        { x, y }, { x, y },
        { r, g }, { r, g },
        { u, v }, { u, v }
    }
}

macro_rules! def_swizzle3 {
    () => {
        pub trait Swizzle3<T: Primitive>: Swizzle2<T> {
            fn z(&self) -> T;
        }
    }
}

macro_rules! def_swizzle4 {
    () => {
        pub trait Swizzle4<T: Primitive>: Swizzle3<T> {
            fn w(&self) -> T;
        }
    }
}

def_swizzle3! {}
def_swizzle4! {}

macro_rules! impl_swizzle2 {
    ($($v: ident),+) => {
        $(
            impl<T: Primitive> Swizzle2<T> for $v<T> {
                #[inline(always)]
                fn x(&self) -> T { self.x }
                #[inline(always)]
                fn y(&self) -> T { self.y }
            }
        )+
    }
}

macro_rules! impl_swizzle3 {
    ($($v: ident),+) => {
        $(
            impl<T: Primitive> Swizzle3<T> for $v<T> {
                #[inline(always)]
                fn z(&self) -> T { self.z }
            }
        )+
    }
}

impl_swizzle2! { Vector2, Vector3, Vector4 }
impl_swizzle3! { Vector3, Vector4 }

impl<T: Primitive> Swizzle4<T> for Vector4<T> {
    #[inline(always)]
    fn w(&self) -> T { self.w }
}

#[cfg(test)]
mod test {

    use vec::vec::*;
    use super::*;

    #[test]
    fn test_swizzle2() {
        let v = ivec4(1, 2, 3, 4);
        assert_eq!(v.xy(), ivec2(1, 2));
        assert_eq!(v.ww(), ivec2(4, 4));
        assert_eq!(v.zy(), ivec2(3, 2));
        assert_eq!(v.gb(), ivec2(2, 3));
        assert_eq!(v.ts(), ivec2(4, 3));
        assert_eq!(v.vv(), ivec2(2, 2));
        assert_eq!(v.bb(), v.ss());
    }

    #[test]
    fn test_swizzle3() {
        let v = vec3(0., 1., 2.);

    }

    #[test]
    fn test_swizzle4() {
        let v = uvec4(0, 7, 5, 2);

    }
}
