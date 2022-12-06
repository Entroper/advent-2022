pub fn InvalidInputError(message: &str) -> std::io::Error {
	std::io::Error::new(std::io::ErrorKind::InvalidInput, message)
}
