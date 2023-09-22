# IEEE 2030.5 Common

`common` - Implementation of IEEE 2030.5 data types

`common-derive` - Rust Procedural Macros providing common interfaces on IEEE 2030.5 data types 

`resources` - IEEE 2030.5 associated resources & documentation published `2018-03-01`


# Progress
- [x] Rust Types for all IEEE 2030.5 Resources, Types & Primitives.
- [x] XML Serialising & Deserialising all resources using [SEPSerde](https://github.com/ethanndickson/yaserde)
- [x] Rust enums for all applicable integer types.
- [x] [Bitflags](https://github.dev/bitflags/bitflags) for all `HexBinary\d+` bitmaps
- [x] Rust Traits for all re-used IEEE 2030.5 base types
  - [x] Common Interface for manipulating List Resources
- [x] Rust Procedural Macros to derive these traits on the appropriate types. 
- [x] [De]?serialising sanity tests (auto-generated)
- [x] All Function Set List Ordering
- [ ] Correctness / Spec Adherence Tests 