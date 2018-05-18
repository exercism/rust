pub fn verse(n: i32) -> String {
	match n {
		0 => "No more bottles of beer on the wall, no more bottles of beer.\n\
			  Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
		1 => "1 bottle of beer on the wall, 1 bottle of beer.\n\
			  Take it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
		n if n <= 99 =>
			format!(
				"{n} bottles of beer on the wall, {n} bottles of beer.\n\
				Take one down and pass it around, {n_minus_1} bottle{s} of beer on the wall.\n",
				n=n,
				n_minus_1=n - 1,
				s=if n - 1 > 1 {"s"} else {""}),
		_ =>
			panic!(),
	}
}

pub fn sing(start: i32, end: i32) -> String {
	let mut string=String::new();
	for i in (end..start+1).rev() {
		string.push_str(&verse(i));
		string.push('\n');
	}
	string.pop();
	string
}
