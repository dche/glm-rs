
#![feature(core, test)]

extern crate test;
extern crate rand;
extern crate glm;

use glm::*;
use test::Bencher;
use std::ops::Mul;
use rand::{ IsaacRng, Rng };

#[path="common/macros.rs"]
#[macro_use] mod macros;

bench_binop!(_mat2_mul_m_byref, Mat2, Mat2, mul_m);
bench_binop!(_mat3_mul_m_byref, Mat3, Mat3, mul_m);
bench_binop!(_mat4_mul_m_byref, Mat4, Mat4, mul_m);

bench_binop_deref!(_mat2_mul_m, Mat2, Mat2, mul);
bench_binop_deref!(_mat3_mul_m, Mat3, Mat3, mul);
bench_binop_deref!(_mat4_mul_m, Mat4, Mat4, mul);

bench_binop_deref!(_mat2_mul_v, Mat2, Vec2, mul);
bench_binop_deref!(_mat3_mul_v, Mat3, Vec3, mul);
bench_binop_deref!(_mat4_mul_v, Mat4, Vec4, mul);

bench_uniop!(_mat2_transpose, Mat2, transpose);
bench_uniop!(_mat3_transpose, Mat3, transpose);
bench_uniop!(_mat4_transpose, Mat4, transpose);

bench_uniop!(_mat2_determinant, Mat2, determinant);
bench_uniop!(_mat3_determinant, Mat3, determinant);
bench_uniop!(_mat4_determinant, Mat4, determinant);

bench_uniop!(_mat2_inverse, Mat2, inverse);
bench_uniop!(_mat3_inverse, Mat3, inverse);
bench_uniop!(_mat4_inverse, Mat4, inverse);
