echo "#![feature(print_internals)]" > ./examples/test-expand.rs
echo "#![feature(structural_match)]" >> ./examples/test-expand.rs
echo "#![feature(core_intrinsics)]" >> ./examples/test-expand.rs
echo "#![feature(panic_internals)]" >> ./examples/test-expand.rs
echo "#![feature(rustc_attrs)]" >> ./examples/test-expand.rs
echo "#![feature(alloc)]" >> ./examples/test-expand.rs
echo "#![feature(fmt_helpers_for_derive)]" >> ./examples/test-expand.rs
echo "#![allow(warnings, unused)]" >> ./examples/test-expand.rs
cargo expand --example test >> ./examples/test-expand.rs
echo "extern crate alloc;" >> ./examples/test-expand.rs