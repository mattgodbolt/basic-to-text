pub mod error;
pub mod tokens;

use error::DecodeError;
use tokens::*;

/// Which BBC BASIC dialect to detokenise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasicVersion {
    Basic2,
    BasicV,
}

/// A parsed but still-tokenised BASIC line.
#[derive(Debug)]
pub struct BasicLine {
    pub line_number: u16,
    pub data: Vec<u8>,
}

/// Parse the binary program into a list of tokenised lines.
///
/// Format: repeating `0x0D <hi> <lo> <len> <data...>`, terminated by `0x0D 0xFF`.
pub fn read_lines(data: &[u8]) -> Result<Vec<BasicLine>, DecodeError> {
    let mut lines = Vec::new();
    let mut pos = 0;

    loop {
        if pos >= data.len() {
            return Err(DecodeError::UnexpectedEnd);
        }
        if data[pos] != 0x0D {
            return Err(DecodeError::BadLineStart(data[pos]));
        }
        pos += 1;

        if pos >= data.len() {
            return Err(DecodeError::UnexpectedEnd);
        }
        if data[pos] == 0xFF {
            break;
        }

        if pos + 2 >= data.len() {
            return Err(DecodeError::UnexpectedEnd);
        }

        let line_number = u16::from_be_bytes([data[pos], data[pos + 1]]);
        let length = data[pos + 2] as usize;
        pos -= 1; // back to the 0x0D for length calculation

        if pos + length > data.len() {
            return Err(DecodeError::UnexpectedEnd);
        }

        let line_data = data[pos + 4..pos + length].to_vec();
        lines.push(BasicLine {
            line_number,
            data: line_data,
        });
        pos += length;
    }

    Ok(lines)
}

/// Decode an encoded line number (3 bytes after the 0x8D marker).
///
/// Encoding scheme from the BBC BASIC source:
/// - byte 2 is XORed with 0x54, its bit 2 and bit 0 form the top 2 bits
///   of the high and low bytes respectively
/// - bytes 0 and 1 have bit 6 masked (subtract 0x40) to give 6 bits each
/// - The result is a 16-bit line number
pub fn decode_line_number(bytes: &[u8; 3]) -> Result<u16, DecodeError> {
    let b2 = bytes[2] ^ 0x54;
    let b0 = bytes[0];
    let b1 = bytes[1];

    // b2 bits: bit 2 => top bit of high byte, bit 0 => top bit of low byte
    let high = ((b2 & 0x04) << 4) | (b0 & 0x3F);
    let low = ((b2 & 0x01) << 6) | (b1 & 0x3F);

    // Bits 4 and 5 of b2 provide additional high bits
    let high = ((b2 & 0x30) << 2) | high;

    Ok(((high as u16) << 8) | low as u16)
}

/// Detokenise a single line's data bytes into a string.
pub fn detokenise_line(data: &[u8], version: BasicVersion) -> Result<String, DecodeError> {
    let mut result = String::new();
    let mut i = 0;

    while i < data.len() {
        let b = data[i];

        match b {
            // Regular ASCII characters
            0x00..=0x7E => {
                result.push(char::from(b));
                i += 1;
            }

            // OTHERWISE in BASIC V, literal in BASIC 2
            0x7F => {
                match version {
                    BasicVersion::BasicV => result.push_str("OTHERWISE"),
                    BasicVersion::Basic2 => result.push(char::from(b)),
                }
                i += 1;
            }

            // Line number marker
            0x8D => {
                if i + 3 >= data.len() {
                    return Err(DecodeError::UnexpectedEnd);
                }
                let line_num = decode_line_number(&[data[i + 1], data[i + 2], data[i + 3]])?;
                result.push_str(&line_num.to_string());
                i += 4;
            }

            // Extended token prefixes (BASIC V) or single tokens (BASIC 2)
            0xC6..=0xC8 => match version {
                BasicVersion::BasicV => {
                    i += 1;
                    if i >= data.len() {
                        return Err(DecodeError::UnexpectedEnd);
                    }
                    let sub = data[i];
                    let idx = (sub as usize).wrapping_sub(0x8E);
                    let name = match b {
                        0xC6 => ESCFN_TOKENS
                            .get(idx)
                            .copied()
                            .ok_or(DecodeError::InvalidExtendedToken { prefix: b, sub })?,
                        0xC7 => ESCCOM_TOKENS
                            .get(idx)
                            .copied()
                            .ok_or(DecodeError::InvalidExtendedToken { prefix: b, sub })?,
                        0xC8 => ESCSTMT_TOKENS
                            .get(idx)
                            .copied()
                            .ok_or(DecodeError::InvalidExtendedToken { prefix: b, sub })?,
                        _ => unreachable!(),
                    };
                    result.push_str(name);
                    i += 1;
                }
                BasicVersion::Basic2 => {
                    let idx = (b as usize) - 0xC6;
                    if let Some(name) = BASIC2_TOKENS_C6_CE[idx] {
                        result.push_str(name);
                    }
                    i += 1;
                }
            },

            // In BASIC 2, 0xC9..=0xCE are single-byte tokens (LIST, NEW, OLD, etc.)
            0xC9..=0xCE if version == BasicVersion::Basic2 => {
                let idx = (b as usize) - 0xC6;
                if let Some(name) = BASIC2_TOKENS_C6_CE[idx] {
                    result.push_str(name);
                }
                i += 1;
            }

            // REM: emit token then copy rest of line literally
            0xF4 => {
                result.push_str("REM");
                i += 1;
                while i < data.len() {
                    result.push(char::from(data[i]));
                    i += 1;
                }
            }

            // DATA: emit token then copy rest literally (may contain high bytes)
            0xDC => {
                result.push_str("DATA");
                i += 1;
                while i < data.len() {
                    result.push(char::from(data[i]));
                    i += 1;
                }
            }

            // All other tokens
            _ => {
                let idx = (b as usize) - 0x7F;
                if let Some(name) = BASE_TOKENS[idx] {
                    result.push_str(name);
                } else {
                    // Shouldn't happen for well-formed data
                    result.push(char::from(b));
                }
                i += 1;
            }
        }
    }

    Ok(result)
}

/// Decode an entire BBC BASIC binary program to text.
pub fn decode(
    data: &[u8],
    version: BasicVersion,
    show_line_numbers: bool,
) -> Result<String, DecodeError> {
    let lines = read_lines(data)?;
    let mut output = String::new();

    for line in &lines {
        if show_line_numbers {
            output.push_str(&format!("{:5} ", line.line_number));
        }
        output.push_str(&detokenise_line(&line.data, version)?);
        output.push('\n');
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines_empty() {
        let data = [0x0D, 0xFF];
        let lines = read_lines(&data).unwrap();
        assert!(lines.is_empty());
    }

    #[test]
    fn test_read_lines() {
        // Line 10: single byte 0x41 ('A')
        // Format: 0x0D <hi> <lo> <len> <data...>
        // Length includes from 0x0D to end of line data
        let data = [
            0x0D, 0x00, 0x0A, 0x05, 0x41, // line 10, length 5, data 'A'
            0x0D, 0xFF, // end
        ];
        let lines = read_lines(&data).unwrap();
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].line_number, 10);
        assert_eq!(lines[0].data, vec![0x41]);
    }

    #[test]
    fn test_read_lines_bad_start() {
        let data = [0x00, 0xFF];
        assert!(matches!(
            read_lines(&data),
            Err(DecodeError::BadLineStart(0x00))
        ));
    }

    #[test]
    fn test_decode_line_number() {
        // Line 139: the Python code doesn't handle this, but the encoding is:
        // Top bits come from byte[2] XOR 0x54
        // Let's verify with a known value.
        // Line 0 encodes as: low=0, high=0
        //   b0 = 0x00 | 0x40 = 0x40, b1 = 0x00 | 0x40 = 0x40
        //   b2: bits for high bit 6 of high=0, high bit 6 of low=0 => 0x00 XOR 0x54 = 0x54
        let line_0 = decode_line_number(&[0x40, 0x40, 0x54]).unwrap();
        assert_eq!(line_0, 0);

        // Line 1: low=1, high=0
        //   b0 = 0x40, b1 = 0x01 | 0x40 = 0x41, b2 = 0x54
        let line_1 = decode_line_number(&[0x40, 0x41, 0x54]).unwrap();
        assert_eq!(line_1, 1);

        // Line 256: high=1, low=0
        //   b0 = 0x01 | 0x40 = 0x41, b1 = 0x40, b2 = 0x54
        let line_256 = decode_line_number(&[0x41, 0x40, 0x54]).unwrap();
        assert_eq!(line_256, 256);
    }

    #[test]
    fn test_detokenise_simple() {
        // PRINT is 0xF1
        let data = [0xF1];
        assert_eq!(
            detokenise_line(&data, BasicVersion::BasicV).unwrap(),
            "PRINT"
        );
    }

    #[test]
    fn test_detokenise_rem() {
        // REM followed by literal text (including high bytes)
        let data = [0xF4, b'h', b'e', b'l', b'l', b'o'];
        assert_eq!(
            detokenise_line(&data, BasicVersion::BasicV).unwrap(),
            "REMhello"
        );
    }

    #[test]
    fn test_detokenise_goto_line() {
        // GOTO (0xE5) + encoded line number for line 0
        let data = [0xE5, 0x8D, 0x40, 0x40, 0x54];
        assert_eq!(
            detokenise_line(&data, BasicVersion::BasicV).unwrap(),
            "GOTO0"
        );
    }

    #[test]
    fn test_detokenise_extended_tokens() {
        // CASE is ESCSTMT (0xC8) + 0x8E
        let data = [0xC8, 0x8E];
        assert_eq!(
            detokenise_line(&data, BasicVersion::BasicV).unwrap(),
            "CASE"
        );

        // SYS is ESCSTMT (0xC8) + 0x99
        let data = [0xC8, 0x99];
        assert_eq!(detokenise_line(&data, BasicVersion::BasicV).unwrap(), "SYS");
    }

    #[test]
    fn test_basic2_tokens() {
        // In BASIC 2, 0xC6 = AUTO (not a prefix)
        let data = [0xC6];
        assert_eq!(
            detokenise_line(&data, BasicVersion::Basic2).unwrap(),
            "AUTO"
        );

        // In BASIC 2, 0xC7 = DELETE
        let data = [0xC7];
        assert_eq!(
            detokenise_line(&data, BasicVersion::Basic2).unwrap(),
            "DELETE"
        );
    }

    #[test]
    fn test_quoted_strings() {
        // BBC BASIC tokenises keywords even inside strings, so they get expanded
        let data = [0xF1, b'"', 0xF1, b'"'];
        let result = detokenise_line(&data, BasicVersion::BasicV).unwrap();
        // 0xF1 inside quotes is PRINT token, expanded back to text
        assert_eq!(result, "PRINT\"PRINT\"");
    }

    #[test]
    fn test_decode_full_program() {
        // A simple program: 10 PRINT "HI"
        let data = [
            0x0D, 0x00, 0x0A, 0x09, // line 10, length 9
            0xF1, b'"', b'H', b'I', b'"', // PRINT "HI"
            0x0D, 0xFF, // end
        ];

        let result = decode(&data, BasicVersion::BasicV, false).unwrap();
        assert_eq!(result, "PRINT\"HI\"\n");

        let result = decode(&data, BasicVersion::BasicV, true).unwrap();
        assert_eq!(result, "   10 PRINT\"HI\"\n");
    }
}
