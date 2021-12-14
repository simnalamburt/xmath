xmath [![crates-i][]][crates-a]
========
[Rust][] port of Microsoft [DirectXMath][]. [Documentation][doc]

```toml
[dependencies]
xmath = "0.2"
```

- [x] Optional [glium][] support
- [x] Out of the box [glium_text][] support

### Glium support

**xmath** supports [glium][] out of the box, but because of its compilation time
it is disabled by default. To use xmath with glium, you can enable it in
`Cargo.toml`.

```toml
[dependencies]
xmath = { version = "0.2", features = ["glium-support"] }
```

```rust
let uniforms = uniform! {
    matrix: Matrix::orthographic(width as f32/10.0, height as f32/10.0, 0.0, 1.0)
};

target.draw(&unit.vb, &unit.ib, program, uniforms, &draw_parameters);
```

--------

[BSD 2-Clause](LICENSE.md)

[Rust]: http://rust-lang.org
[DirectXMath]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh437833(v=vs.85).aspx
[doc]: https://simnalamburt.github.io/xmath/
[glium]: https://github.com/tomaka/glium
[glium_text]: https://github.com/tomaka/glium_text

[crates-i]: https://img.shields.io/crates/v/xmath.svg
[crates-a]: https://crates.io/crates/xmath
