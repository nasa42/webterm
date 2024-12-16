use crate::models::webterm_error::WebtermError;
use flate2::write::{DeflateDecoder, DeflateEncoder};
use flate2::Compression;
use std::io::Write;

pub fn compress(data: &[u8]) -> Result<Vec<u8>, WebtermError> {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(data)
        .map_err(|e| WebtermError::RuntimeError(format!("Compression failed: {}", e)))?;
    encoder
        .finish()
        .map_err(|e| WebtermError::RuntimeError(format!("Compression finalization failed: {}", e)))
}

pub fn decompress(data: &[u8]) -> Result<Vec<u8>, WebtermError> {
    let mut decoder = DeflateDecoder::new(Vec::new());
    decoder
        .write_all(data)
        .map_err(|e| WebtermError::RuntimeError(format!("Decompression failed: {}", e)))?;
    decoder.finish().map_err(|e| {
        WebtermError::RuntimeError(format!("Decompression finalization failed: {}", e))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test vectors shared between Rust and TypeScript implementations
    const COMPATIBILITY_TEST_CASES: &[(&[u8], &[u8])] = &[
        (b"", &[3, 0]),
        (
            b"Hello, world!",
            &[
                243, 72, 205, 201, 201, 215, 81, 40, 207, 47, 202, 73, 81, 4, 0,
            ],
        ),
        (b"AAAAAAAA", &[115, 116, 132, 0, 0]),
    ];

    #[test]
    fn test_compression_compatibility() {
        for (input, expected) in COMPATIBILITY_TEST_CASES {
            let compressed = compress(input).unwrap();
            assert_eq!(
                &compressed, expected,
                "Compression mismatch for input: {:?}\nGot:     {:?}\nExpected: {:?}",
                input, &compressed, expected
            );

            let decompressed = decompress(&compressed).unwrap();
            assert_eq!(
                &decompressed, input,
                "Decompression mismatch for input: {:?}",
                input
            );
        }
    }

    #[test]
    fn print_compression_outputs() {
        let inputs: &[&[u8]] = &[b"", b"Hello World!", b"AAAAAA"];

        for input in inputs {
            let compressed = compress(input).unwrap();
            println!("Input: {:?}", input);
            println!("Compressed: {:?}\n", compressed);
        }
    }
}
