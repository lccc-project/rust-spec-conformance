# Rust Specification Conformance Test Suite

This is a conformance test suite for the Rust Specification. It is primarily intended for use by the lccc project, though it is intended to be useful for any Rust compiler.

The Suite is intended to be comprehensive, but not exhaustive - it will test both success and failure conditions in the language and standard library implementation, but will not test every possible rust program, or obscure feature interactions. It also does not test QoI feature like diagnostics. An implementation that wishes to check precise diagnostics would need to devise its own test suite 

## Adding new tests

### Test Metadata

Test Metadata can be added by a line comment (exactly 2 slashes) starting with `@` followed by a keyword, and arbitrary information following. The keyword and `@` may be separated by whitespace, but the `@` must immediate follow the `//` of the line comment. Metadata is only recognized if occurs at the start of the file, only preceeded by the optional hashbang line, metadata lines, and other line comments (lines that are entirely consisting of whitespace, including empty lines, are ignored). Note that while the first line starting with `#![` are not considered hashbang lines by Rust, the tool will ignore these lines anyways. This is both an oversimplification, and deliberate behaviour for tests that use the hashbang.

The following keywords are recognized (unrecognized keywords are accepted but will issue a warning):
* `run-pass`, `run-fail`, `compile-only`, `compile-pass`: These keywords dictate the mode the test uses. One of these must appear (if it is absent, a warning is issued, and compile-only mode is used)
* `reference`: This keyword lets you indicate which reference rule-id is tested by the test. The directive is ignored without warning, but might be used by external tools (like mdbook-spec).

## Copyright Notice

Copyright (c) 2024 LCCC Project Contributors

This project, including all tests, is licensed under the MIT, Apache 2.0, and 2 Clause BSD plus Patent Licenses. You may, at your option, use this project under the terms of any of these licenses. Copies of the licenses are included with the repo. The above Copyright Notice must be maintained in all cases.

As per the Apache 2.0 License, unless you indicate otherwise, any contribution intentionally submitted by you for inclusion in this repository implicitly includes permission to the LCCC Project to license the contribution as above. For these purposes contribution means a submission of code, which is subject to copyright, that can be included automatically in the repository or through minor manual intervention (such as manual merging or rebasing), and includes but is not limited to Github Pull Requests.