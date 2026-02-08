gaviota-sys
===========

Low level Rust bindings for [libgtb](https://github.com/michiguel/Gaviota-Tablebases), a library for Gaviota tablebase probing.

[![Test](https://github.com/niklasf/rust-gaviota-sys/workflows/Test/badge.svg)](https://github.com/niklasf/rust-gaviota-sys/actions)
[![crates.io](https://img.shields.io/crates/v/gaviota-sys.svg)](https://crates.io/crates/gaviota-sys)
[![docs.rs](https://docs.rs/gaviota-sys/badge.svg)](https://docs.rs/gaviota-sys)

Disclaimer
----------

These low level bindings are sound if and only if the underlying C library is
sound. Checksums of tablebase files should be verified before use.

Documentation
-------------

* [Generated API reference](https://docs.rs/gaviota-sys)
* [`gtb-probe.h`](https://github.com/michiguel/Gaviota-Tablebases/blob/master/gtb-probe.h)

License
-------

View `Gaviota-Tablebases/license.txt` for the licenses of the various
components (essentially MIT + Zlib).
