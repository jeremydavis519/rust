//@normalize-stderr-test: "unsafe \{ libc::abort\(\) \}|crate::intrinsics::abort\(\);" -> "ABORT();"
//@normalize-stderr-test: "\| +\^+" -> "| ^"
//@normalize-stderr-test: "\n  +[0-9]+:[^\n]+" -> "$1"
//@normalize-stderr-test: "\n at [^\n]+" -> "$1"
//@error-in-other-file: aborted execution
#![feature(never_type)]

#[allow(deprecated, invalid_value)]
fn main() {
    let _ = unsafe { std::mem::uninitialized::<!>() };
}
