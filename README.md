xmath [![crates-i][]][crates-a] [![travis-i][]][travis-a]
========
[Rust][] port of Microsoft [DirectXMath][]. [Documentation][doc]

```toml
[dependencies]
xmath = "0.2"
```

Glium support
--------

**xmath** supports [glium][] out of the box.

```toml
[dependencies]
xmath = { version = "0.2", features = ["glium-support"] }
```

--------

[BSD 2-Clause](LICENSE.md)

[Rust]: http://rust-lang.org
[DirectXMath]: https://msdn.microsoft.com/en-us/library/windows/desktop/hh437833(v=vs.85).aspx
[doc]: https://simnalamburt.github.io/xmath/
[glium]: https://github.com/tomaka/glium

[crates-i]: https://img.shields.io/crates/v/xmath.svg
[crates-a]: https://crates.io/crates/xmath
[travis-i]: https://travis-ci.org/simnalamburt/xmath.svg?branch=master
[travis-a]: https://travis-ci.org/simnalamburt/xmath
