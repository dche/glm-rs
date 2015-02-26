
#![feature(core, test)]

extern crate test;
extern crate rand;
extern crate glm;

use glm::*;
use glm::ext::*;
use test::Bencher;
use std::num::Float;
use rand::{ IsaacRng, Rng };

#[path="common/macros.rs"]
#[macro_use] mod macros;

bench_binop_deref!(_f32_pow_intrinsic, f32, f32, powf);
bench_binfn_deref!(_f32_pow, f32, f32, pow);
bench_binfn_deref!(_f32_powi, f32, i32, powi);

bench_uniop!(_f32_exp_intrinsic, f32, exp);
bench_unifn_deref!(_f32_exp, f32, exp);

bench_uniop!(_f32_sin_intrinsic, f32, sin);
bench_unifn_deref!(_f32_sin, f32, sin);

bench_unifn_deref!(_vec2_exp, Vec2, exp);
bench_unifn_deref!(_vec3_exp, Vec3, exp);
bench_unifn_deref!(_vec4_exp, Vec4, exp);

bench_unifn_deref!(_vec2_acos, Vec2, acos);
bench_unifn_deref!(_vec3_acos, Vec3, acos);
bench_unifn_deref!(_vec4_acos, Vec4, acos);

bench_binfn_deref!(_vec2_atan2, Vec2, Vec2, atan2);
bench_binfn_deref!(_vec3_atan2, Vec3, Vec3, atan2);
bench_binfn_deref!(_vec4_atan2, Vec4, Vec4, atan2);

bench_unifn_deref!(_vec2_sin, Vec2, sin);
bench_unifn_deref!(_vec3_sin, Vec3, sin);
bench_unifn_deref!(_vec4_sin, Vec4, sin);

bench_unifn_deref!(_dvec2_exp, DVec2, exp);
bench_unifn_deref!(_dvec3_exp, DVec3, exp);
bench_unifn_deref!(_dvec4_exp, DVec4, exp);

bench_unifn_deref!(_dvec2_sin, DVec2, sin);
bench_unifn_deref!(_dvec3_sin, DVec3, sin);
bench_unifn_deref!(_dvec4_sin, DVec4, sin);

bench_binfn_deref!(_vec2_dot, Vec2, Vec2, dot);
bench_binfn_deref!(_vec3_dot, Vec3, Vec3, dot);
bench_binfn_deref!(_vec4_dot, Vec4, Vec4, dot);

bench_unifn_deref!(_vec2_length, Vec2, length);
bench_unifn_deref!(_vec3_length, Vec3, length);
bench_unifn_deref!(_vec4_length, Vec4, length);

bench_unifn_deref!(_vec2_normalize, Vec2, normalize);
bench_unifn_deref!(_vec3_normalize, Vec3, normalize);
bench_unifn_deref!(_vec4_normalize, Vec4, normalize);

bench_binfn_deref!(_vec2_distance, Vec2, Vec2, distance);
bench_binfn_deref!(_vec3_distance, Vec3, Vec3, distance);
bench_binfn_deref!(_vec4_distance, Vec4, Vec4, distance);

bench_binfn_deref!(_cross, Vec3, Vec3, cross);
