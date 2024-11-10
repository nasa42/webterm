pub fn format_pty_output(bytes: &[u8]) -> String {
    let mut result = String::new();

    for chunk in bytes.utf8_chunks() {
        if !chunk.valid().is_empty() {
            result.push_str(&process_string(chunk.valid()));
        }

        if !chunk.invalid().is_empty() {
            for &byte in chunk.invalid() {
                result.push_str(&format!("\\x{:02x}", byte));
            }
        }
    }

    result
}

fn process_string(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '\x08' => "^H".to_string(),
            '\x0A' => "\n".to_string(),
            '\x0C' => "^L".to_string(),
            '\x0D' => "\r".to_string(),
            '\x1B' => "^[ ".to_string(),
            '\x7F' => "^?".to_string(),
            c if c.is_control() => {
                // Represent other control characters as ^<char>
                let caret_notation = (c as u8 + 0x40) as char;
                format!("^{}", caret_notation)
            }
            _ => c.to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_pty_output() {
        let bytes = vec![
            0x08, 0x0A, 0x0C, 0x0D, 0x1B, 0x7F, b'A', b'B', b'C', 0xc0, 0xaf, 0xe2, 0x98,
            0x83, // Unicode character '☃' (Snowman)
        ];
        let expected = "^H\n^L\r^[ ^?ABC\\xc0\\xaf☃".to_string();
        assert_eq!(format_pty_output(&bytes), expected);
    }
}
