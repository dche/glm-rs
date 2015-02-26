
#![feature(core, test)]

extern crate test;
extern crate rand;
extern crate glm;

use glm::*;
use test::Bencher;
use std::ops::{ Add, Mul, Sub, Div };
use rand::{ IsaacRng, Rng };

#[path="common/macros.rs"]
#[macro_use] mod macros;

bench_binop_deref!(_vec2_add_v, Vec2, Vec2, add);
bench_binop_deref!(_vec3_add_v, Vec3, Vec3, add);
bench_binop_deref!(_vec4_add_v, Vec4, Vec4, add);

bench_binop_deref!(_dvec2_add_v, DVec2, DVec2, add);
bench_binop_deref!(_dvec3_add_v, DVec3, DVec3, add);
bench_binop_deref!(_dvec4_add_v, DVec4, DVec4, add);

bench_binop_deref!(_vec2_sub_v, Vec2, Vec2, sub);
bench_binop_deref!(_vec3_sub_v, Vec3, Vec3, sub);
bench_binop_deref!(_vec4_sub_v, Vec4, Vec4, sub);

bench_binop_deref!(_vec2_mul_v, Vec2, Vec2, mul);
bench_binop_deref!(_vec3_mul_v, Vec3, Vec3, mul);
bench_binop_deref!(_vec4_mul_v, Vec4, Vec4, mul);

bench_binop_deref!(_vec2_div_v, Vec2, Vec2, div);
bench_binop_deref!(_vec3_div_v, Vec3, Vec3, div);
bench_binop_deref!(_vec4_div_v, Vec4, Vec4, div);

bench_binop_deref!(_vec2_add_s, Vec2, f32, add);
bench_binop_deref!(_vec3_add_s, Vec3, f32, add);
bench_binop_deref!(_vec4_add_s, Vec4, f32, add);

bench_binop_deref!(_vec2_sub_s, Vec2, f32, sub);
bench_binop_deref!(_vec3_sub_s, Vec3, f32, sub);
bench_binop_deref!(_vec4_sub_s, Vec4, f32, sub);

bench_binop_deref!(_vec2_mul_s, Vec2, f32, mul);
bench_binop_deref!(_vec3_mul_s, Vec3, f32, mul);
bench_binop_deref!(_vec4_mul_s, Vec4, f32, mul);

bench_binop_deref!(_vec2_div_s, Vec2, f32, div);
bench_binop_deref!(_vec3_div_s, Vec3, f32, div);
bench_binop_deref!(_vec4_div_s, Vec4, f32, div);
