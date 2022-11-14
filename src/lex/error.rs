pub struct InvalidChar;

impl Display for InvalidChar {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(f, "Invalid character")
	}
}
