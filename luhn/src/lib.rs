/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.len() < 2 {
        return false;
    }
    todo!("bytes to digits?")
}
