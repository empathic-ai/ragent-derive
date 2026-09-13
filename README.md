# Ragent derive macros

`Task` generates an implementation of the consuming crate's Ragent task trait.
It resolves renamed dependencies with `proc-macro-crate` and parses tokens
without a string round trip. Ragent Core remains the owner of the runtime trait;
the macro crate does not need to compile it as a host dependency.

Validate changes with the application/provider consumers as well as this crate.
The macro's generated trait implementation and the runtime's bounds must agree.
