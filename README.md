# SEP2 Common Library

`sep2_common` is a (WIP) Rust library for developing IEEE 2030.5 compliant servers & clients.

It can be used with [`sep2_client`](https://github.com/ethanndickson/sep2_client), which provides a framework for developing IEEE 2030.5 clients.

[Documentation](https://ethanndickson.github.io/sep2_client_docs/sep2_common/)

# Contents

[`sep2_common`](sep2_common) - Implementation of IEEE 2030.5 data types

[`sep2_common_derive`](sep2_common_derive) - Rust Procedural Macros providing inheritance-based interfaces on IEEE 2030.5 data types 


### Core Features
- [x] Rust Types for practically all IEEE 2030.5 Resources, Types & Primitives.
- [x] XML Serialising & Deserialising all resources using [SEPSerde](https://github.com/ethanndickson/yaserde)
- [x] Rust enums for all applicable integer types.
- [x] [Bitflags](https://github.dev/bitflags/bitflags) for all `HexBinary\d+` bitmaps for improved ergonomics
- [x] Rust Traits for all re-used IEEE 2030.5 base types
  - [x] Common Interface for manipulating List Resources
- [x] Rust Procedural Macros to derive these traits on the appropriate types. 
- [x] [De]?serialising sanity tests (auto-generated)
- [x] Function Set List Ordering
- [x] CSIP-AUS Extension Resources & Attributes
- [ ] Correctness / Spec Adherence Tests 
### Future
- [ ] EXI Serialising & Deserialising all resources (Requires a Rust EXI Library)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.