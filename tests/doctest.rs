// Note: this may show errors in intellisense until first build
// This generates output from `skeptic`, via `build.rs`,
// for automating tests of README.md code segments.
include!(concat!(env!("OUT_DIR"), "/skeptic-tests.rs"));
