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
use traits::GenFloat;
use vec::traits::GenFloatVec;

pub trait Consts<T: BaseFloat>: GenFloat<T> {
    fn pi() -> Self;
    fn tau() -> Self;
    fn root_pi() -> Self;
    fn half_pi() -> Self;
    fn one_third_pi() -> Self;
    fn quarter_pi() -> Self;
    fn one_over_pi() -> Self;
    fn one_over_tau() -> Self;
    fn two_over_pi() -> Self;
    fn four_over_pi() -> Self;
    fn two_over_root_pi() -> Self;
    fn one_over_root_two() -> Self;
    fn root_half_pi() -> Self;
    fn root_tau() -> Self;
    fn root_ln_four() -> Self;
    fn e() -> Self;
    fn euler() -> Self;
    fn root_two() -> Self;
    fn root_three() -> Self;
    fn root_five() -> Self;
    fn ln_two() -> Self;
    fn ln_ten() -> Self;
    fn ln_ln_two() -> Self;
    fn one_third() -> Self;
    fn two_thirds() -> Self;
    fn golden_ratio() -> Self;
}

macro_rules! impl_Consts_for {
    ($($bt: ident),+) => {
        $(
            impl<T> Consts<$bt> for T where T: GenFloat<$bt> {
                #[inline(always)]
                fn pi() -> T {
                    T::from_s(3.14159265358979323846264338327950288)
                }
                #[inline(always)]
                fn tau() -> T {
                    T::from_s(6.28318530717958647692528676655900576)
                }
                #[inline(always)]
                fn root_pi() -> T {
                    T::from_s(1.772453850905516027)
                }
                #[inline(always)]
                fn half_pi() -> T {
                    T::from_s(1.57079632679489661923132169163975144)
                }
                #[inline(always)]
                fn one_third_pi() -> T {
                    T::from_s(1.04719755119659774615421446109316763)
                }
                #[inline(always)]
                fn quarter_pi() -> T {
                    T::from_s(0.785398163397448309615660845819875721)
                }
                #[inline(always)]
                fn one_over_pi() -> T {
                    T::from_s(0.318309886183790671537767526745028724)
                }
                #[inline(always)]
                fn one_over_tau() -> T {
                    T::from_s(0.159154943091895335768883763372514362)
                }
                #[inline(always)]
                fn two_over_pi() -> T {
                    T::from_s(0.636619772367581343075535053490057448)
                }
                #[inline(always)]
                fn four_over_pi() -> T {
                    T::from_s(1.273239544735162686151070106980114898)
                }
                #[inline(always)]
                fn two_over_root_pi() -> T {
                    T::from_s(1.12837916709551257389615890312154517)
                }
                #[inline(always)]
                fn one_over_root_two() -> T {
                    T::from_s(0.707106781186547524400844362104849039)
                }
                #[inline(always)]
                fn root_half_pi() -> T {
                    T::from_s(1.253314137315500251)
                }
                #[inline(always)]
                fn root_tau() -> T {
                    T::from_s(2.506628274631000502)
                }
                #[inline(always)]
                fn root_ln_four() -> T {
                    T::from_s(1.17741002251547469)
                }
                #[inline(always)]
                fn e() -> T {
                    T::from_s(2.71828182845904523536028747135266250)
                }
                #[inline(always)]
                fn euler() -> T {
                    T::from_s(0.577215664901532860606)
                }
                #[inline(always)]
                fn root_two() -> T {
                    T::from_s(1.41421356237309504880168872420969808)
                }
                #[inline(always)]
                fn root_three() -> T {
                    T::from_s(1.73205080756887729352744634150587236)
                }
                #[inline(always)]
                fn root_five() -> T {
                    T::from_s(2.23606797749978969640917366873127623)
                }
                #[inline(always)]
                fn ln_two() -> T {
                    T::from_s(0.693147180559945309417232121458176568)
                }
                #[inline(always)]
                fn ln_ten() -> T {
                    T::from_s(2.30258509299404568401799145468436421)
                }
                #[inline(always)]
                fn ln_ln_two() -> T {
                    T::from_s(-0.3665129205816643)
                }
                #[inline(always)]
                fn one_third() -> T {
                    T::from_s(0.3333333333333333333333333333333333333333)
                }
                #[inline(always)]
                fn two_thirds() -> T {
                    T::from_s(0.666666666666666666666666666666666666667)
                }
                fn golden_ratio() -> T {
                    T::from_s(1.61803398874989484820458683436563811)
                }
            }
        )+
    }
}

impl_Consts_for! { f32, f64 }

// NOTE: can't define these functions without the indirection (trait `Consts`),
//       because Rust does not support implicit conversion.

/// Returns the epsilon constant for floating point types.
#[inline(always)]
pub fn epsilon<F: BaseFloat, T: GenFloatVec<F>>() -> T {
    T::from_s(F::epsilon())
}

/// Returns the Archimedes' constant π.
///
/// # Example
///
/// ```rust
/// use glm::*;
/// use glm::ext::*;
///
/// assert_eq!(DVec4::pi(), dvec4(pi(), pi(), pi(), pi()));
#[inline(always)]
pub fn pi<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::pi()
}

/// Returns π * 2.
#[inline(always)]
pub fn tau<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::tau()
}

/// Returns square root of π.
#[inline(always)]
pub fn root_pi<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::root_pi()
}

/// Returns π / 2.
#[inline(always)]
pub fn half_pi<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::half_pi()
}

/// Returns π / 3.
#[inline(always)]
pub fn one_third_pi<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::one_third_pi()
}

/// Returns π / 4.
#[inline(always)]
pub fn quarter_pi<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::quarter_pi()
}

/// Returns 1 / π.
#[inline(always)]
pub fn one_over_pi<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::one_over_pi()
}

/// Returns 1 / tau.
#[inline(always)]
pub fn one_over_tau<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::one_over_tau()
}

/// Returns 2 / π.
#[inline(always)]
pub fn two_over_pi<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::two_over_pi()
}

/// Returns 4 / π.
#[inline(always)]
pub fn four_over_pi<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::four_over_pi()
}

/// Returns 2 / sqrt(π).
#[inline(always)]
pub fn two_over_root_pi<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::two_over_root_pi()
}

/// Returns 1 / sqrt(2).
#[inline(always)]
pub fn one_over_root_two<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::one_over_root_two()
}

/// Returns sqrt(π / 2).
#[inline(always)]
pub fn root_half_pi<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::root_half_pi()
}

/// Returns sqrt(tau).
#[inline(always)]
pub fn root_tau<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::root_tau()
}

/// Returns sqrt(ln(4)).
#[inline(always)]
pub fn root_ln_four<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::root_ln_four()
}

/// Returns e constant.
#[inline(always)]
pub fn e<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::e()
}

/// Returns Euler's constant.
#[inline(always)]
pub fn euler<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::euler()
}

/// Returns sqrt(2).
#[inline(always)]
pub fn root_two<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::root_two()
}

/// Returns sqrt(3).
#[inline(always)]
pub fn root_three<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::root_three()
}

/// Returns sqrt(5).
#[inline(always)]
pub fn root_five<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::root_five()
}

/// Returns ln(2).
#[inline(always)]
pub fn ln_two<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::ln_two()
}

/// Returns ln(10).
#[inline(always)]
pub fn ln_ten<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::ln_ten()
}

/// Returns ln(ln(2)).
#[inline(always)]
pub fn ln_ln_two<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::ln_ln_two()
}

/// Returns 1 / 3.
#[inline(always)]
pub fn one_third<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::one_third()
}

/// Returns 2 / 3.
#[inline(always)]
pub fn two_thirds<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::two_thirds()
}

/// Returns the golden ratio constant.
pub fn golden_ratio<F: BaseFloat, T: Consts<F>>() -> T {
    Consts::golden_ratio()
}
