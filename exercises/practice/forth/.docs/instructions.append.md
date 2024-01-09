# Instructions append

Note the additional test case in `tests/alloc-attack.rs`. It tests against
algorithmically inefficient implementations. Because of that, it usually times
out online instead of outright failing, leading to a less helpful error message.
