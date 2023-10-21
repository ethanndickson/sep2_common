# IEEE 2030.5 Common (Smart Energy Profile 2.0) (SEP2)

`sep2_common` is a (WIP) Rust library for developing IEEE 2030.5 compliant servers & clients.

It can be used with [`sep2_client`](https://github.com/ethanndickson/IEEE-2030.5-Client), which provides an interface for developing IEEE 2030.5 clients.

# Contents

`common` - Implementation of IEEE 2030.5 data types

`common-derive` - Rust Procedural Macros providing common interfaces on IEEE 2030.5 data types 

`resources` - IEEE 2030.5 associated resources & documentation published `2018-03-01`


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
- [ ] Correctness / Spec Adherence Tests 
### Future
- [ ] EXI Serialising & Deserialising all resources (Requires a Rust EXI Library)
### Broken
- pubsub::NotificationList: Serializing + Deserializing
