use basic_to_text::{BasicVersion, decode};

/// Helper to build a minimal BBC BASIC binary program from line entries.
/// Each entry is (line_number, raw_tokenised_data).
fn build_program(lines: &[(u16, &[u8])]) -> Vec<u8> {
    let mut data = Vec::new();
    for &(line_num, line_data) in lines {
        let length = 4 + line_data.len(); // 0x0D + hi + lo + len + data
        data.push(0x0D);
        data.push((line_num >> 8) as u8);
        data.push((line_num & 0xFF) as u8);
        data.push(length as u8);
        data.extend_from_slice(line_data);
    }
    data.push(0x0D);
    data.push(0xFF);
    data
}

#[test]
fn test_simple_print() {
    let program = build_program(&[(10, b"\xF1\"Hello\"")]);
    let result = decode(&program, BasicVersion::BasicV, false).unwrap();
    assert_eq!(result, "PRINT\"Hello\"\n");
}

#[test]
fn test_with_line_numbers() {
    let program = build_program(&[(10, b"\xF1\"A\""), (20, b"\xE0")]);
    let result = decode(&program, BasicVersion::BasicV, true).unwrap();
    assert_eq!(result, "   10 PRINT\"A\"\n   20 END\n");
}

#[test]
fn test_goto_with_encoded_line() {
    // GOTO (0xE5) + line number marker (0x8D) + encoded line 10
    // Line 10: high=0, low=10
    //   b0 = 0x00 | 0x40 = 0x40
    //   b1 = 0x0A | 0x40 = 0x4A
    //   b2 = 0x00 ^ 0x54 = 0x54
    let program = build_program(&[(100, &[0xE5, 0x8D, 0x40, 0x4A, 0x54])]);
    let result = decode(&program, BasicVersion::BasicV, false).unwrap();
    assert_eq!(result, "GOTO10\n");
}

#[test]
fn test_gosub_with_encoded_line() {
    // GOSUB (0xE4) + line number marker (0x8D) + encoded line 256
    // Line 256: high=1, low=0
    //   b0 = 0x01 | 0x40 = 0x41
    //   b1 = 0x00 | 0x40 = 0x40
    //   b2 = 0x00 ^ 0x54 = 0x54
    let program = build_program(&[(50, &[0xE4, 0x8D, 0x41, 0x40, 0x54])]);
    let result = decode(&program, BasicVersion::BasicV, false).unwrap();
    assert_eq!(result, "GOSUB256\n");
}

#[test]
fn test_rem_preserves_literal() {
    // REM (0xF4) + literal bytes including high byte
    let program = build_program(&[(10, &[0xF4, b' ', b'h', b'i', 0x80])]);
    let result = decode(&program, BasicVersion::BasicV, false).unwrap();
    assert_eq!(result, "REM hi\u{80}\n");
}

#[test]
fn test_basic_v_extended_tokens() {
    // WHILE is ESCSTMT (0xC8) + 0x95
    let program = build_program(&[(10, &[0xC8, 0x95, b' ', b'x'])]);
    let result = decode(&program, BasicVersion::BasicV, false).unwrap();
    assert_eq!(result, "WHILE x\n");
}

#[test]
fn test_basic2_single_byte_tokens() {
    // In BASIC 2, 0xC6 = AUTO, 0xC9 = LIST
    let program = build_program(&[(10, &[0xC6]), (20, &[0xC9])]);
    let result = decode(&program, BasicVersion::Basic2, false).unwrap();
    assert_eq!(result, "AUTO\nLIST\n");
}

#[test]
fn test_quoted_string_expands_tokens() {
    // BBC BASIC tokenises keywords even inside strings
    let program = build_program(&[(10, &[0xF1, b'"', 0xF1, 0xE5, b'"'])]);
    let result = decode(&program, BasicVersion::BasicV, false).unwrap();
    // Tokens inside quotes are expanded back to text
    assert_eq!(result, "PRINT\"PRINTGOTO\"\n");
}

#[test]
fn test_empty_program() {
    let program = build_program(&[]);
    let result = decode(&program, BasicVersion::BasicV, false).unwrap();
    assert_eq!(result, "");
}

#[test]
fn test_multi_line_program() {
    let program = build_program(&[
        (10, &[0xF4, b' ', b'S', b't', b'a', b'r', b't']), // REM Start
        (20, b"\xF1\"OK\""),                               // PRINT"OK"
        (30, &[0xE0]),                                     // END
    ]);
    let result = decode(&program, BasicVersion::BasicV, true).unwrap();
    assert_eq!(result, "   10 REM Start\n   20 PRINT\"OK\"\n   30 END\n");
}
