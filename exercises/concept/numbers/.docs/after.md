When working in Rust there are a few aspects to consider.
The key distinctions are the difference between integers (numbers with no digits after the decimal separator) and floating-point numbers (numbers with no digits after the decimal separator).
The other factor is the signedness of integers, with the option signed (positive and negative numbers, with half the integers range for negative and half for positive numbers) and unsigned (only positive numbers).

Numbers can be compared using the default comparison operators (`<`, `>`, `==`, etc.). These operators can be used in if statements to conditionally execute code.
When dealing with arithmetic between different numeric types in Rust, we need to make certain all the values are of the same type.
Casting does introduce the possibility of losing data when converting to a type of a smaller size or when casting a floating-point number to an integer.
