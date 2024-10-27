#![cfg(any())]
// ^ The above specifically is being tested as the very start of the file, don't add anything before it, or reorder the directives with it.
//@ compile-only
//@ reference: input.shebang.inner-attribute

// Deliberately produce a post-expansion error.
core::compile_error!("Bad Compiler, go read input.shebang.inner-attribute");
