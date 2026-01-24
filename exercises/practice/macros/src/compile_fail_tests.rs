/// To activate a doctest locally, remove ",ignore" from the code block.
///
/// # Comma separator
///
/// ```compile_fail,ignore
/// use macros::hashmap;
/// use std::collections::HashMap;
///
/// // using only commas is invalid
/// let _hm: HashMap<_, _> = hashmap!('a', 1);
/// ```
///
/// # Double trailing commas
///
/// ```compile_fail,ignore
/// use macros::hashmap;
/// use std::collections::HashMap;
///
/// // a single trailing comma is okay, but two is not
/// let _hm: HashMap<_, _> = hashmap!('a' => 2, ,);
/// ```
///
/// # Only comma
///
/// ```compile_fail,ignore
/// use macros::hashmap;
/// use std::collections::HashMap;
///
/// // a single random comma is not valid
/// let _hm: HashMap<(), ()> = hashmap!(,);
/// ```
///
/// # Single argument
///
/// ```compile_fail,ignore
/// use macros::hashmap;
/// use std::collections::HashMap;
///
/// // a single argument is invalid
/// let _hm: HashMap<_, _> = hashmap!('a');
/// ```
///
/// # Triple arguments
///
/// ```compile_fail,ignore
/// use macros::hashmap;
/// use std::collections::HashMap;
///
/// // three arguments are invalid
/// hashmap!('a' => 1, 'b');
/// ```
///
/// # Only arrow
///
/// ```compile_fail,ignore
/// use macros::hashmap;
/// use std::collections::HashMap;
///
/// // a single random arrow is not valid
/// let _hm: HashMap<(), ()> = hashmap!(=>);
/// ```
///
/// # Trailing arrow
///
/// ```compile_fail,ignore
/// use macros::hashmap;
/// use std::collections::HashMap;
///
/// // a trailing => isn't valid either
/// hashmap!('a' => 2, =>);
/// ```
///
/// # Leading comma
///
/// ```compile_fail,ignore
/// use macros::hashmap;
/// use std::collections::HashMap;
///
/// // leading commas are not valid
/// let _hm: HashMap<_, _> = hashmap!(, 'a' => 2);
/// ```
///
/// # Missing comma
///
/// ```compile_fail,ignore
/// use macros::hashmap;
/// use std::collections::HashMap;
///
/// // Key value pairs must be separated by commas
/// let _hm: HashMap<_, _> = hashmap!('a' => 1 'b' => 2);
/// ```
///
/// # Missing argument
///
/// ```compile_fail,ignore
/// use macros::hashmap;
/// use std::collections::HashMap;
///
/// // an argument should come between each pair of commas
/// let _hm: HashMap<_, _> = hashmap!('a' => 1, , 'b' => 2);
/// ```
///
const _TESTS: () = ();
