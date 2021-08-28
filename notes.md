Tried installing binaryen but cargo build took _ages_ and then failed to allocate memory. Yikes! The underlying C++ project is 50k lines of source.

Also these bindings don't really make it nice in Rust, the FFI is a nightmare

Also `cargo run` takes forever for minor changes

This is a bit shit
