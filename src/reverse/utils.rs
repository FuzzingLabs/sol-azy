use std::fmt::Write as _;

/// Formats a byte slice into a Rust-style byte string literal (`b"..."`).
///
/// Printable ASCII characters (including spaces) are rendered as-is.
/// Non-printable or non-ASCII bytes are rendered using hexadecimal escapes (`\xNN`).
///
/// # Arguments
///
/// * `slice` - The byte slice to format.
///
/// # Returns
///
/// A `String` formatted as a byte string literal with proper escaping.
/// 
pub fn format_bytes(slice: &[u8]) -> String {
    let mut bytes_repr = String::from("b\"");

    // Render printable ASCII as-is, otherwise use hex escape
    for &b in slice {
        if b.is_ascii_graphic() || b == b' ' {
            bytes_repr.push(b as char);
        } else {
            // Add hexadecimal escape sequence for non-printable bytes
            write!(&mut bytes_repr, "\\x{:02x}", b).unwrap();
        }
    }

    bytes_repr.push('"');
    bytes_repr
}
