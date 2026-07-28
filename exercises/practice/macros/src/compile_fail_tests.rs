// To activate a doctest locally, remove ",ignore" from the code block.

/// ```compile_fail,ignore
/// use macros::hashmap;
/// use std::collections::HashMap;
///
/// // using only commas is invalid
/// let _hm: HashMap<_, _> = hashmap!('a', 1);
/// ```
const _COMMA_SEPARATOR: () = ();

/// ```compile_fail,ignore
/// use macros::hashmap;
/// use std::collections::HashMap;
///
/// // a single trailing comma is okay, but two is not
/// let _hm: HashMap<_, _> = hashmap!('a' => 2, ,);
/// ```
const _DOUBLE_TRAILING_COMMAS: () = ();

/// ```compile_fail,ignore
/// use macros::hashmap;
/// use std::collections::HashMap;
///
/// // a single random comma is not valid
/// let _hm: HashMap<(), ()> = hashmap!(,);
/// ```
const _ONLY_COMMA: () = ();

/// ```compile_fail,ignore
/// use macros::hashmap;
/// use std::collections::HashMap;
///
/// // a single argument is invalid
/// let _hm: HashMap<_, _> = hashmap!('a');
/// ```
const _SINGLE_ARGUMENT: () = ();

/// ```compile_fail,ignore
/// use macros::hashmap;
/// use std::collections::HashMap;
///
/// // three arguments are invalid
/// hashmap!('a' => 1, 'b');
/// ```
const _TRIPLE_ARGUMENTS: () = ();

/// ```compile_fail,ignore
/// use macros::hashmap;
/// use std::collections::HashMap;
///
/// // a single random arrow is not valid
/// let _hm: HashMap<(), ()> = hashmap!(=>);
/// ```
const _ONLY_ARROW: () = ();

/// ```compile_fail,ignore
/// use macros::hashmap;
/// use std::collections::HashMap;
///
/// // a trailing => isn't valid either
/// hashmap!('a' => 2, =>);
/// ```
const _TRAILING_ARROW: () = ();

/// ```compile_fail,ignore
/// use macros::hashmap;
/// use std::collections::HashMap;
///
/// // leading commas are not valid
/// let _hm: HashMap<_, _> = hashmap!(, 'a' => 2);
/// ```
const _LEADING_COMMA: () = ();

/// ```compile_fail,ignore
/// use macros::hashmap;
/// use std::collections::HashMap;
///
/// // Key value pairs must be separated by commas
/// let _hm: HashMap<_, _> = hashmap!('a' => 1 'b' => 2);
/// ```
const _MISSING_COMMA: () = ();

/// ```compile_fail,ignore
/// use macros::hashmap;
/// use std::collections::HashMap;
///
/// // an argument should come between each pair of commas
/// let _hm: HashMap<_, _> = hashmap!('a' => 1, , 'b' => 2);
/// ```
const _MISSING_ARGUMENT: () = ();
