# Instructions append

Different languages on Exercism have different expectations about what the result should be if the length of the substrings is zero.
On the Rust track, we don't have a test for that case, so you are free to do what you feel is most appropriate.

Consider the advantages and disadvantages of the following possibilities:
- Crash the program with `panic!`.
- Return a `Result::Err`. (not possible here, because the function signature is given)
- Return an empty vector.
- Return a vector containing as many empty strings as the length of the string "digits" **plus one**. (this has some nice mathematical properties!)
