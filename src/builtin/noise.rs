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

// The GLSL Specification, ch 8.11, Noise Functions.

//////////////////////////////////////////////////////////////////////////////
// Translated directly from g-truc (Christophe Riccio)'s original implementation.
//////////////////////////////////////////////////////////////////////////////
// OpenGL Mathematics (glm.g-truc.net)
//
// Copyright (c) 2005 - 2015 G-Truc Creation (www.g-truc.net)
//
// MIT License
//////////////////////////////////////////////////////////////////////////////

#![allow(non_snake_case)]

use traits::GenType;
use cast::to_vec4;
use vec::vec::{ Vec2, Vec3, Vec4, vec2, vec3, vec4 };
use super::common::*;
use super::geom::dot;
use super::vecrel::lessThan;
use num::{ One, Zero };

#[allow(non_snake_case)]
pub fn grad4(j: f32, ip: Vec4) -> Vec4 {
    let mut pXYZ = floor(fract(vec3(j, j, j) * ip.truncate(3)) * 7.) * ip[2] - 1.;
    let pW = 1.5 - dot(abs(pXYZ), Vec3::one());
    let s = to_vec4(lessThan(vec4(pXYZ.x, pXYZ.y, pXYZ.z, pW), Vec4::zero()));
    pXYZ = pXYZ + (s.truncate(3) * 2. - 1.) * s.w;
    vec4(pXYZ.x, pXYZ.y, pXYZ.z, pW)
}

#[inline(always)]
pub fn mod289<T: GenType>(x: T) -> T {
    x - floor(x * (1. / 289.)) * 289.
}

#[inline(always)]
pub fn permute<T: GenType>(x: T) -> T {
    mod289((x * 34. + 1.) * x)
}

#[inline(always)]
pub fn taylor_inv_sqrt<T: GenType>(x: T) -> T {
    -x * 0.85373472095314 + 1.79284291400159
}

pub trait NoiseImpl {
    fn noise1(self) -> f32;
}

impl NoiseImpl for f32 {
    #[inline]
    fn noise1(self) -> f32 {
        vec2(self, 0.).noise1()
    }
}

impl NoiseImpl for Vec2 {
    fn noise1(self) -> f32 {
        let yi = Vec2::one();
        let C = vec4(
             0.211324865405187,     //  (3.0 -  sqrt(3.0)) / 6.0
             0.366025403784439,     //  0.5 * (sqrt(3.0)  - 1.0)
            -0.577350269189626,     // -1.0 + 2.0 * C.x
             0.024390243902439      //  1.0 / 41.0
        );
        // first corner
        let mut i = floor(self + dot(self, yi * C.y));
        let x0 = self - i + dot(i, yi * C.x);
        // Other corners
        //i1.x = step( x0.y, x0.x ); // x0.x > x0.y ? 1.0 : 0.0
        //i1.y = 1.0 - i1.x;
        let i1 = if x0.x > x0.y { vec2(1., 0.) } else { vec2(0., 1.) };

        // x0 = x0 - 0.0 + 0.0 * C.xx ;
        // x1 = x0 - i1 + 1.0 * C.xx ;
        // x2 = x0 - 1.0 + 2.0 * C.xx ;
        let mut x12 = vec4(x0.x, x0.y, x0.x, x0.y) + vec4(C.x, C.x, C.z, C.z);
        x12 = vec4(x12.x - i1.x, x12.y - i1.y, x12.z, x12.w);

        // Permutations
        i = mod_s(i, 289.);    // Avoid truncation effects in permutation
        let p = permute(permute(
            vec3(0., i1.y, 1.) + i.y) +
            vec3(0., i1.x, 1.) + i.x);

        let mut m = max_s(
            -vec3(
                dot(x0, x0),
                dot(vec2(x12.x, x12.y), vec2(x12.x, x12.y)),
                dot(vec2(x12.z, x12.w), vec2(x12.z, x12.w))
            ) + 0.5,
            0.
        );
        m = m * m;
        m = m * m;

        // Gradients: 41 points uniformly over a line, mapped onto a diamond.
        // The ring size 17*17 = 289 is close to a multiple of 41 (41*7 = 287)

        let x = fract(p * C.w) * 2. - 1.;
        let h = abs(x) - 0.5;
        let ox = floor(x + 0.5);
        let a0 = x - ox;

        // Normalise gradients implicitly by scaling m
        // Inlined for speed: m *= taylorInvSqrt( a0*a0 + h*h );
        m = m * ((a0 * a0 + h * h) * -0.85373472095314 + 1.79284291400159);

        // Compute final noise value at P
        let g = vec3(
            a0.x * x0.x + h.x * x0.y,
            //g.yz = a0.yz * x12.xz + h.yz * x12.yw;
            a0.y * x12.x + h.y * x12.y,
            a0.z * x12.z + h.z * x12.w
        );
        dot(m, g) * 130.
    }
}

impl NoiseImpl for Vec3 {
    fn noise1(self) -> f32 {
        let yi = Vec3::one();
        let C = vec2(1./6., 1./3.);
        let D = vec4(0., 0.5, 1., 2.);

        // First corner
        let mut i = floor(self + dot(self, yi * C.y));
        let x0 = self - i + dot(i, yi * C.x);

        // Other corners
        let g = step(vec3(x0.y, x0.z, x0.x), x0);
        let l = yi - g;
        let i1 = min(g, vec3(l.z, l.x, l.y));
        let i2 = max(g, vec3(l.z, l.x, l.y));

        // x0 = x0 - 0.0 + 0.0 * C.xxx;
        // x1 = x0 - i1  + 1.0 * C.xxx;
        // x2 = x0 - i2  + 2.0 * C.xxx;
        // x3 = x0 - 1.0 + 3.0 * C.xxx;
        let x1 = x0 - i1 + C.x;
        let x2 = x0 - i2 + C.y;     // 2.0*C.x = 1/3 = C.y
        let x3 = x0 - D.y;          // -1.0+3.0*C.x = -0.5 = -D.y

        // Permutations
        i = mod289(i);
        let p: Vec4 = permute(permute(permute(
            vec4(0., i1.z, i2.z, 1.) + i.z) +
            vec4(0., i1.y, i2.y, 1.) + i.y) +
            vec4(0., i1.x, i2.x, 1.) + i.x);

        // Gradients: 7x7 points over a square, mapped onto an octahedron.
        // The ring size 17*17 = 289 is close to a multiple of 49 (49*6 = 294)
        let n_ = 0.142857142857_f32;    // 1.0 / 7.0;
        let ns = vec3(D.w, D.y, D.z) * n_ - vec3(D.x, D.z, D.x);

        let j = p - floor(p * ns.z * ns.z) * 49.;   // mod(p,7*7)

        let x_ = floor(j * ns.z);
        let y_ = floor(j - x_ * 7.);    // mod(j, N)

        let x = x_ * ns.x + ns.y;
        let y = y_ * ns.x + ns.y;
        let h = Vec4::one() - abs(x) - abs(y);

        let b0 = vec4(x.x, x.y, y.x, y.y);
        let b1 = vec4(x.z, x.w, y.z, y.w);

        // vec4 s0 = vec4(lessThan(b0,0.0))*2.0 - 1.0;
        // vec4 s1 = vec4(lessThan(b1,0.0))*2.0 - 1.0;
        let s0 = floor(b0) * 2. + 1.;
        let s1 = floor(b1) * 2. + 1.;
        let sh = -step(h, Vec4::zero());

        let a0 = vec4(b0.x, b0.z, b0.y, b0.w) + vec4(s0.x, s0.z, s0.y, s0.w) * vec4(sh.x, sh.x, sh.y, sh.y);
        let a1 = vec4(b1.x, b1.z, b1.y, b1.w) + vec4(s1.x, s1.z, s1.y, s1.w) * vec4(sh.z, sh.z, sh.w, sh.w);

        let mut p0 = vec3(a0.x, a0.y, h.x);
        let mut p1 = vec3(a0.z, a0.w, h.y);
        let mut p2 = vec3(a1.x, a1.y, h.z);
        let mut p3 = vec3(a1.z, a1.w, h.w);

        // Normalise gradients
        let norm = taylor_inv_sqrt(vec4(
            dot(p0, p0), dot(p1, p1), dot(p2, p2), dot(p3, p3)
        ));
        p0 = p0 * norm.x;
        p1 = p1 * norm.y;
        p2 = p2 * norm.z;
        p3 = p3 * norm.w;

        // Mix final noise value
        let mut m = max_s(-vec4(dot(x0, x0), dot(x1, x1), dot(x2, x2), dot(x3, x3)) + 0.6, 0.);
        m = m * m;
        42. * dot(m * m, vec4(dot(p0, x0), dot(p1, x1), dot(p2, x2), dot(p3, x3)))
    }
}

impl NoiseImpl for Vec4 {
    fn noise1(self) -> f32 {
        let yi = Vec4::one();
        let C = vec4(
             0.138196601125011,     // (5 - sqrt(5))/20  G4
             0.276393202250021,     // 2 * G4
             0.414589803375032,     // 3 * G4
            -0.447213595499958      // -1 + 4 * G4
        );
        // (sqrt(5) - 1)/4 = F4, used once below
        let F4: f32 = 0.309016994374947451;

        // First corner
        let mut i = floor(self + dot(self, yi * F4));
        let x0 = self - i + dot(i, yi * C.x);

        // Other corners

        // Rank sorting originally contributed by Bill Licea-Kane, AMD (formerly ATI)
        let isX = step(vec3(x0.y, x0.z, x0.w), vec3(x0.x, x0.x, x0.x));
        let isYZ = step(vec3(x0.z, x0.w, x0.w), vec3(x0.y, x0.y, x0.z));

        // i0.x = dot(isX, vec3(1.0));
        // i0.x = isX.x + isX.y + isX.z;
        // i0.yzw = static_cast<T>(1) - isX;
        let mut i0 = vec4(isX.x + isX.y + isX.z, 1. - isX.x, 1. - isX.y, 1. - isX.z);
        //  i0.y += dot(isYZ.xy, vec2(1.0));
        i0.y += isYZ.x + isYZ.y;
        //i0.zw += 1.0 - tvec2<T, P>(isYZ.x, isYZ.y);
        i0.z += 1. - isYZ.x;
        i0.w += 1. - isYZ.y;
        i0.z += isYZ.z;
        i0.w += 1. - isYZ.z;

        // i0 now contains the unique values 0,1,2,3 in each channel
        let i3 = clamp_s(i0, 0., 1.);
        let i2 = clamp_s(i0 - 1., 0., 1.);
        let i1 = clamp_s(i0 - 2., 0., 1.);

        //  x0 = x0 - 0.0 + 0.0 * C.xxxx
        //  x1 = x0 - i1  + 0.0 * C.xxxx
        //  x2 = x0 - i2  + 0.0 * C.xxxx
        //  x3 = x0 - i3  + 0.0 * C.xxxx
        //  x4 = x0 - 1.0 + 4.0 * C.xxxx
        let x1 = x0 - i1 + C.x;
        let x2 = x0 - i2 + C.y;
        let x3 = x0 - i3 + C.z;
        let x4 = x0 + C.w;

        // Permutations
        i = mod_s(i, 289.);
        let j0 = permute(permute(permute(permute(i.w) + i.z) + i.y) + i.x);
        let j1 = permute(permute(permute(permute(
            vec4(i1.w, i2.w, i3.w, 1.) + i.w) +
            vec4(i1.z, i2.z, i3.z, 1.) + i.z) +
            vec4(i1.y, i2.y, i3.y, 1.) + i.y) +
            vec4(i1.x, i2.x, i3.x, 1.) + i.x);

        // Gradients: 7x7x6 points over a cube, mapped onto a 4-cross polytope
        // 7*7*6 = 294, which is close to the ring size 17*17 = 289.
        let ip = vec4(1./294., 1./49., 1./7., 0.);

        let mut p0 = grad4(j0,   ip);
        let mut p1 = grad4(j1.x, ip);
        let mut p2 = grad4(j1.y, ip);
        let mut p3 = grad4(j1.z, ip);
        let mut p4 = grad4(j1.w, ip);

        // Normalise gradients
        let norm =
            taylor_inv_sqrt(vec4(dot(p0, p0), dot(p1, p1), dot(p2, p2), dot(p3, p3)));
        p0 = p0 * norm.x;
        p1 = p1 * norm.y;
        p2 = p2 * norm.z;
        p3 = p3 * norm.w;
        p4 = p4 * taylor_inv_sqrt(dot(p4, p4));

        // Mix contributions from the five corners
        let mut m0 = max_s(vec3(0.6, 0.6, 0.6) - vec3(dot(x0, x0), dot(x1, x1), dot(x2, x2)), 0.);
        let mut m1 = max_s(vec2(0.6, 0.6) - vec2(dot(x3, x3), dot(x4, x4)), 0.);
        m0 = m0 * m0;
        m1 = m1 * m1;

        dot(m0 * m0, vec3(dot(p0, x0), dot(p1, x1), dot(p2, x2))) +
        dot(m1 * m1, vec2(dot(p3, x3), dot (p4, x4))) * 49.
    }
}

/// Returns a 1D noise value based on the input value `x`.
#[inline]
pub fn noise1<T: GenType + NoiseImpl>(x: T) -> f32 {
    x.noise1()
}

/// Returns a 2D noise value based on the input value `x`.
#[inline]
pub fn noise2<T: GenType + NoiseImpl>(x: T) -> Vec2 {
    vec2(x.noise1(), (-x).noise1())
}

/// Returns a 3D noise value based on the input value `x`.
#[inline]
pub fn noise3<T: GenType + NoiseImpl>(x: T) -> Vec3 {
    let yi = T::one();
    vec3((x - yi).noise1(), x.noise1(), (x + yi).noise1())
}

/// Returns a 4D noise value based on the input value `x`.
#[inline]
pub fn noise4<T: GenType + NoiseImpl>(x: T) -> Vec4 {
    let yi = T::one();
    vec4(
        (x - yi).noise1(),
        x.noise1(),
        (x + yi).noise1(),
        (x + yi + yi).noise1()
    )
}

#[cfg(test)]
mod test {

}
