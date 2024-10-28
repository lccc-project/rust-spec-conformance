//@ compile-fail
// Test to ensure an obviously ill-formed compile-fail test fails

core::compile_error!("Meta test succeed");
