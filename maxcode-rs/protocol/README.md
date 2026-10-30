# maxcode-protocol

This crate defines the "types" for the protocol used by Maxcode CLI, which includes both "internal types" for communication between `maxcode-core` and `maxcode-tui`, as well as "external types" used with `maxcode app-server`.

This crate should have minimal dependencies.

Ideally, we should avoid "material business logic" in this crate, as we can always introduce `Ext`-style traits to add functionality to types in other crates.
