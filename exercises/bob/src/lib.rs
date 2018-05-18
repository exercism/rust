pub fn reply(message: &str) -> &str {
	if yelling(message) && asking(message) {
		return "Calm down, I know what I'm doing!"
	}
	if yelling(message) {
		return "Whoa, chill out!"
	}
	if asking(message) {
		return "Sure."
	}
	if silence(message) {
		return "Fine. Be that way!"
	}
	"Whatever."
}

fn yelling(s: &str) -> bool {
	s.chars().all(|c| !c.is_lowercase()) &&
		s.chars().any(|c| c.is_uppercase())
}

fn silence(s: &str) -> bool {
	s.trim().is_empty()
}

fn asking(s: &str) -> bool {
	s.trim_right().ends_with('?')
}


