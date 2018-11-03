# Defaults

The files in this directory are used in appropriate places for defaults. They
are included at build-time in the compiled executable: if you change one, you
will need to recompile to see those changes reflected in the output.

Using external files makes it easier to ensure that formatting etc is correct,
without needing to worry about proper escaping within a Rust source file.
