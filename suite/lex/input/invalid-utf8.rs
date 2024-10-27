//@ compile-fail: This test contains invalid UTF-8 At the end of it
//@ reference: input.encoding.invalid
// There's not really much to say about this, except that it contains invalid UTF-8
// For anyone reading this, the file ends with `\xFF`, if you can't see it.
// Uh, please don't delete the `\xFF` byte, even accidentally
// In particular, if you are looking at this in VSC, don't save it, and on't run rustfmt on this

// `\xFF` byte immediately after this comment
/*interrupt the stream here*/
//ÿ
