
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

# Thanks

- [GLM](glm.g-truc.net) by G-Truc (Christophe Riccio). The implementation of
  noise functions are directly translated from GLM.
- Creators of the [Rust](http://rust-lang.org) for the language.

# License

The MIT License (MIT)
