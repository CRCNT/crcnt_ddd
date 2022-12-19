# Macros for CRCNT DDD

As a macro lib, the `proc_macro` should be set to `true` int the `Cargo.toml`

And the macro lib export nothing other than the `macro`. the `macro` can't be
invoked in the same lib, so we need another lib to test them. That comes 
the `macros` and the `macros_test` libs.
