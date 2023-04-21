use utf8_read::{Char, Reader};

/// A structure that buffers chars from an input file
///
/// The internal buffer allows characters to be peeked. `N` times forward
/// where `N` is the chosen size of the buffer.
pub struct BufCharacterReader<'a> {
	/// The size of the desired buffer, the size specified here
	/// corresponds to the amount of chars that you can peek forward by.
	pub buffer_size: usize,
	// The current position of the iterator in the character stream.
	pub index: usize,
	// The current position of the buffer in the character stream.
	pub buf_index: usize,
	// The backing vector that caches `N` future values.
	pub buffer: Vec<char>,
	// The reader for parsing utf-8 characters.
	reader: Reader<&'a mut dyn std::io::Read>,
	// A flag to indicate if the buffer has seen the end of input.
	buf_end: bool,
}

impl<'a> BufCharacterReader<'a> {
	/// Creates a new `BufCharacterReader` instance given a structure that implements [`std::io::Read`]
	/// and a size for the internal buffer
	pub fn new(readable: &'a mut dyn std::io::Read, buffer_size: usize) -> Self {
		let mut reader = Self {
			buffer_size,
			buffer: vec![' '; buffer_size],
			reader: Reader::new(readable),
			index: 0,
			buf_index: 0,
			buf_end: false,
		};

		reader.fill_buffer();

		reader
	}
}

impl BufCharacterReader<'_> {
	fn fill_buffer(&mut self) {
		if self.index >= self.buf_index {
			// If our current index is greater than or equal to the buffer index and we've
			// hit the end for the buffer return `None` to signal the end-of-input.
			if self.buf_end {
				return;
			}

			// Keep track of how many characters are added to the buffer.
			let mut i = 0;

			// Try to fill the entire buffer with new values if possible.
			for _ in 0..self.buffer_size {
				let next_ch = self.reader.next_char();

				// `Char::NoData` indicates we've hit the end.
				if let Ok(Char::NoData) = next_ch {
					self.buf_end = true;
					break;
				}

				let Ok(Char::Char(next_ch)) = next_ch else {
					panic!("Malformed character")
				};

				self.buffer[i] = next_ch;
				i += 1;
			}

			// Update the buffer index
			self.buf_index += i;
		}
	}
}

impl Iterator for BufCharacterReader<'_> {
	type Item = char;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.buffer.is_empty() {
			Some(self.buffer.remove(0))
		} else {
			self.reader.next_char().ok().and_then(|ch| match ch {
				Char::Char(ch) => Some(ch),
				_ => None,
			})
		}
	}
}

impl BufCharacterReader<'_> {
	/// Get a reference to the next character without consuming it.
	///
	/// Returns [`None`] when end-of-input is hit.
	pub fn peek_n(&mut self, n: usize) -> Vec<&char> {
		while self.buffer.len() < n {
			let Ok(Char::Char(ch)) = self.reader.next_char() else {
				self.buf_end = true;
				break;
			};

			if self.buffer.len() == self.buffer_size {
				self.buffer.pop();
			}

			self.buffer.push(ch);
		}

		self.buffer.iter().take(n).collect()
	}

	pub fn peek(&mut self) -> Option<&char> {
		self.peek_n(1).first().copied()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::shared::Result;

	#[test]
	fn test_buffered_chars() -> Result<()> {
		let mut input = "let a = b + 2".as_bytes();

		let mut buffered_chars = BufCharacterReader::new(&mut input, 10);

		assert_eq!(buffered_chars.peek(), Some(&'l'));
		assert_eq!(buffered_chars.next(), Some('l'));
		assert_eq!(buffered_chars.peek(), Some(&'e'));
		assert_eq!(buffered_chars.next(), Some('e'));
		assert_eq!(buffered_chars.next(), Some('t'));
		assert_eq!(buffered_chars.next(), Some(' '));
		assert_eq!(buffered_chars.next(), Some('a'));
		assert_eq!(buffered_chars.next(), Some(' '));
		assert_eq!(buffered_chars.next(), Some('='));
		assert_eq!(buffered_chars.next(), Some(' '));
		assert_eq!(buffered_chars.next(), Some('b'));
		assert_eq!(buffered_chars.next(), Some(' '));
		assert_eq!(buffered_chars.next(), Some('+'));
		assert_eq!(buffered_chars.next(), Some(' '));
		assert_eq!(buffered_chars.next(), Some('2'));
		assert_eq!(buffered_chars.peek(), None);

		let mut input = "test = 1 + 278".as_bytes();

		let mut buffered_chars = BufCharacterReader::new(&mut input, 10);

		assert_eq!(buffered_chars.peek(), Some(&'t'));
		assert_eq!(buffered_chars.next(), Some('t'));
		assert_eq!(buffered_chars.peek(), Some(&'e'));
		assert_eq!(buffered_chars.next(), Some('e'));
		assert_eq!(buffered_chars.next(), Some('s'));
		assert_eq!(buffered_chars.next(), Some('t'));
		assert_eq!(buffered_chars.next(), Some(' '));
		assert_eq!(buffered_chars.next(), Some('='));
		assert_eq!(buffered_chars.next(), Some(' '));
		assert_eq!(buffered_chars.next(), Some('1'));
		assert_eq!(buffered_chars.next(), Some(' '));
		assert_eq!(buffered_chars.next(), Some('+'));
		assert_eq!(buffered_chars.peek(), Some(&' '));
		assert_eq!(buffered_chars.next(), Some(' '));
		assert_eq!(buffered_chars.next(), Some('2'));
		assert_eq!(buffered_chars.next(), Some('7'));
		assert_eq!(buffered_chars.next(), Some('8'));
		assert_eq!(buffered_chars.peek(), None);

		let mut input = "278".as_bytes();

		let mut buffered_chars = BufCharacterReader::new(&mut input, 10);

		assert_eq!(buffered_chars.peek(), Some(&'2'));
		assert_eq!(buffered_chars.next(), Some('2'));
		assert_eq!(buffered_chars.next(), Some('7'));
		assert_eq!(buffered_chars.next(), Some('8'));
		assert_eq!(buffered_chars.peek(), None);

		Ok(())
	}
}
