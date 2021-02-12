## Edge cases

Handling edge cases nicely while creating CSV makes the world a better place.
The following is the 'standard' way to escape the data:

- Fields containing line breaks, double-quotes, and commas should be enclosed in double-quotes.

- If double-quotes are used to enclose fields, then a double-quote appearing inside a field must be escaped by preceding it with another double-quote.

## Details Galore

Remember how we said a formal specification exists?
[RFC 4180](https://tools.ietf.org/html/rfc4180) has all your details.
Enjoy!
