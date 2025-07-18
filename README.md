# Rust Specification Conformance Test Suite

This is a conformance test suite for the Rust Specification. It is primarily intended for use by the lccc project, though it is intended to be useful for any Rust compiler.
This test suite is not an official test suite produced by the Rust Project or Rust Foundation, and is not endorsed or associated with either, but is intended to be correct (and eventually complete).

The Specification the suite is based on is the [Rust Reference](https://doc.rust-lang.org/nightly/reference), as it is turned into the Rust Specification by the efforts of the Rust Project Specification Team and the Rust Foundation.

The Suite is intended to be comprehensive, but not exhaustive - it will test both success and failure conditions in the language and standard library implementation, but will not test every possible rust program, or obscure feature interactions. It also does not test QoI feature like diagnostics (however, some tests may support limited error checking in the future, to ensure that an compile-fail test is done for the right reason). An implementation that wishes to check precise diagnostics would need to devise its own test suite 

## Running the suite

The project can be built and run on all tests via `cargo run`. By default, the suite uses the same compiler for building the test suite as running it. This can be overriden by setting the `TEST_RUN_RUSTC` environment variable. Currently only `rustc`-like CLIs are supported. In the future, support for a gcc-style CLI will be implemented.

## Adding new tests

### Test Metadata

Test Metadata can be added by a line comment (exactly 2 slashes) starting with `@` followed by a keyword, and arbitrary information following. The keyword and `@` may be separated by whitespace, but the `@` must immediate follow the `//` of the line comment. Metadata is only recognized if occurs at the start of the file, only preceeded by the optional hashbang line, metadata lines, and other line comments (lines that are entirely consisting of whitespace, including empty lines, are ignored). Note that while the first line starting with `#![` are not considered hashbang lines by Rust, the tool will ignore these lines anyways. This is both an oversimplification, and deliberate behaviour for tests that use the hashbang.

The following keywords are recognized (unrecognized keywords are accepted but will issue a warning):
* `run-pass`, `run-fail`, `compile-only`, `compile-pass`: These keywords dictate the mode the test uses. One of these must appear (if it is absent, a warning is issued, and compile-only mode is used)
* `reference`: This keyword lets you indicate which reference rule-id is tested by the test. The directive is ignored without warning, but might be used by external tools (like mdbook-spec).
* `skip`: Do not run this file as a test. 
* `edition`: Ensure the test is run with a specific edition. Else use the same edition as the test suite does.

### Structure

Structure should roughly follow the structure of the Rust specification, with folders added as necessary, using the identifiers from the reference as appropriate names. 
Standard library tests should generally follow the module structure. Do not duplicate tests between `core`, `alloc`, and `std` (test only the first module in which it appears).

Files in the `meta` folder are reserved for tests of the executor itself. Do not add semantically relevant tests to this section.

## Copyright Notice

Copyright (c) 2024 LCCC Project Contributors

This project, including all tests, is licensed under the MIT, Apache 2.0, and 2 Clause BSD plus Patent Licenses. You may, at your option, use this project under the terms of any of these licenses in any combination. Copies of the licenses are included with the repo. The above Copyright Notice must be maintained in all cases.

As per the Apache 2.0 License, unless you indicate otherwise, any contribution intentionally submitted by you for inclusion in this repository implicitly includes permission to the LCCC Project to license the contribution as above. For these purposes contribution means a submission of code, which is subject to copyright, that can be included automatically in the repository or through minor manual intervention (such as manual merging or rebasing), and includes but is not limited to Github Pull Requests.