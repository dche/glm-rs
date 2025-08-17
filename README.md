# GLSL Mathematics for Rust.

[![Build Status](https://travis-ci.org/dche/glm-rs.png?branch=master)](https://travis-ci.org/dche/glm-rs)

Another Rust mathematics library for graphics applications.

Inspired by the great **[GLM](glm.g-truc.net)** library for C++, `glm-rs`
implements the data types, operators and built-in functions defined in GLSL
specification to help graphics programmers who are familiar with GLSL, so they
do not need to learn more than one math API.

Because of the not so small syntax/semantics difference between Rust and GLSL,
some parts of GLSL are missing or changed, some functions are added to
complete the functionalities. See the [glm-rs documentation](http://dche.github.io/glm-rs/) for full list
of differences.

The project is in beta status until the major version reaches to `1`.

## Thanks

- [GLM](glm.g-truc.net) by G-Truc (Christophe Riccio). The implementation of
  noise functions are directly translated from GLM.
- Creators of the [Rust](http://rust-lang.org) for the language.

## v0.3 (2025-08-17) Note

This version is a quick fix to let the crate work with new versions of Rust,
with following PRs & issues resolved.

- Support Rust edition 2021 (#26, #30).
- Manually merged PR #29.
- `quickcheck` is a dev-dependency now (#25).
- Marked pack/unpack functions `unsafe` (#27).
- Constructor functions (e.g., `vec3(..)`, `mat4x3(...)`) are `const` (#31).

### Breaking Change

- `GenNum` dropped `Rand` trait, which was removed in new versions of `rand` crate.

  Creating scalar and vector structs randomly with default distribution and range
  has few meanings in practice. So following statement does not compile anymore,

  ```rust
  let _ = Vec3::rand();
  ```

### Future Plan

- [ ] `no_std` support.
- [ ] SIMD support.
- [ ] Quaternion.
- [ ] Re-design the generic scalar/vector types using the new Rust language features.

## License

The MIT License (MIT)
