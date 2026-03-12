/// Base tokens shared between BASIC 2 and BASIC V.
/// Indexed by `(byte - 0x7F)`. `None` entries are handled specially:
/// - Index for 0x8D (line number marker)
/// - Indices for 0xC6, 0xC7, 0xC8 (extended token prefixes in BASIC V)
pub const BASE_TOKENS: [Option<&str>; 129] = [
    // 0x7F
    Some("OTHERWISE"),
    // 0x80..=0x8F
    Some("AND"),    // 80
    Some("DIV"),    // 81
    Some("EOR"),    // 82
    Some("MOD"),    // 83
    Some("OR"),     // 84
    Some("ERROR"),  // 85
    Some("LINE"),   // 86
    Some("OFF"),    // 87
    Some("STEP"),   // 88
    Some("SPC"),    // 89
    Some("TAB("),   // 8A
    Some("ELSE"),   // 8B
    Some("THEN"),   // 8C
    None,           // 8D - line number marker
    Some("OPENIN"), // 8E
    Some("PTR"),    // 8F
    // 0x90..=0x9F
    Some("PAGE"),  // 90
    Some("TIME"),  // 91
    Some("LOMEM"), // 92
    Some("HIMEM"), // 93
    Some("ABS"),   // 94
    Some("ACS"),   // 95
    Some("ADVAL"), // 96
    Some("ASC"),   // 97
    Some("ASN"),   // 98
    Some("ATN"),   // 99
    Some("BGET"),  // 9A
    Some("COS"),   // 9B
    Some("COUNT"), // 9C
    Some("DEG"),   // 9D
    Some("ERL"),   // 9E
    Some("ERR"),   // 9F
    // 0xA0..=0xAF
    Some("EVAL"),    // A0
    Some("EXP"),     // A1
    Some("EXT"),     // A2
    Some("FALSE"),   // A3
    Some("FN"),      // A4
    Some("GET"),     // A5
    Some("INKEY"),   // A6
    Some("INSTR("),  // A7
    Some("INT"),     // A8
    Some("LEN"),     // A9
    Some("LN"),      // AA
    Some("LOG"),     // AB
    Some("NOT"),     // AC
    Some("OPENUP"),  // AD
    Some("OPENOUT"), // AE
    Some("PI"),      // AF
    // 0xB0..=0xBF
    Some("POINT("), // B0
    Some("POS"),    // B1
    Some("RAD"),    // B2
    Some("RND"),    // B3
    Some("SGN"),    // B4
    Some("SIN"),    // B5
    Some("SQR"),    // B6
    Some("TAN"),    // B7
    Some("TO"),     // B8
    Some("TRUE"),   // B9
    Some("USR"),    // BA
    Some("VAL"),    // BB
    Some("VPOS"),   // BC
    Some("CHR$"),   // BD
    Some("GET$"),   // BE
    Some("INKEY$"), // BF
    // 0xC0..=0xCF
    Some("LEFT$("),   // C0
    Some("MID$("),    // C1
    Some("RIGHT$("),  // C2
    Some("STR$"),     // C3
    Some("STRING$("), // C4
    Some("EOF"),      // C5
    None,             // C6 - ESCFN prefix in BASIC V
    None,             // C7 - ESCCOM prefix in BASIC V
    None,             // C8 - ESCSTMT prefix in BASIC V
    Some("WHEN"),     // C9
    Some("OF"),       // CA
    Some("ENDCASE"),  // CB
    Some("ELSE"),     // CC
    Some("ENDIF"),    // CD
    Some("ENDWHILE"), // CE
    Some("PTR"),      // CF
    // 0xD0..=0xDF
    Some("PAGE"),  // D0
    Some("TIME"),  // D1
    Some("LOMEM"), // D2
    Some("HIMEM"), // D3
    Some("SOUND"), // D4
    Some("BPUT"),  // D5
    Some("CALL"),  // D6
    Some("CHAIN"), // D7
    Some("CLEAR"), // D8
    Some("CLOSE"), // D9
    Some("CLG"),   // DA
    Some("CLS"),   // DB
    Some("DATA"),  // DC
    Some("DEF"),   // DD
    Some("DIM"),   // DE
    Some("DRAW"),  // DF
    // 0xE0..=0xEF
    Some("END"),      // E0
    Some("ENDPROC"),  // E1
    Some("ENVELOPE"), // E2
    Some("FOR"),      // E3
    Some("GOSUB"),    // E4
    Some("GOTO"),     // E5
    Some("GCOL"),     // E6
    Some("IF"),       // E7
    Some("INPUT"),    // E8
    Some("LET"),      // E9
    Some("LOCAL"),    // EA
    Some("MODE"),     // EB
    Some("MOVE"),     // EC
    Some("NEXT"),     // ED
    Some("ON"),       // EE
    Some("VDU"),      // EF
    // 0xF0..=0xFF
    Some("PLOT"),    // F0
    Some("PRINT"),   // F1
    Some("PROC"),    // F2
    Some("READ"),    // F3
    Some("REM"),     // F4
    Some("REPEAT"),  // F5
    Some("REPORT"),  // F6
    Some("RESTORE"), // F7
    Some("RETURN"),  // F8
    Some("RUN"),     // F9
    Some("STOP"),    // FA
    Some("COLOUR"),  // FB
    Some("TRACE"),   // FC
    Some("UNTIL"),   // FD
    Some("WIDTH"),   // FE
    Some("OSCLI"),   // FF
];

/// BBC BASIC 2 tokens for 0xC6..=0xCE (single-byte, no prefix).
/// These don't exist in BASIC V where 0xC6-0xC8 are prefix bytes.
pub const BASIC2_TOKENS_C6_CE: [Option<&str>; 9] = [
    Some("AUTO"),     // C6
    Some("DELETE"),   // C7
    Some("LOAD"),     // C8
    Some("LIST"),     // C9
    Some("NEW"),      // CA
    Some("OLD"),      // CB
    Some("RENUMBER"), // CC
    Some("SAVE"),     // CD
    Some("EDIT"),     // CE
];

/// BASIC V extended function tokens (prefix 0xC6), sub-byte indexed from 0x8E.
pub const ESCFN_TOKENS: [&str; 2] = [
    "SUM",  // 8E
    "BEAT", // 8F
];

/// BASIC V extended command tokens (prefix 0xC7), sub-byte indexed from 0x8E.
pub const ESCCOM_TOKENS: [&str; 18] = [
    "APPEND",   // 8E
    "AUTO",     // 8F
    "CRUNCH",   // 90
    "DELET",    // 91
    "EDIT",     // 92
    "HELP",     // 93
    "LIST",     // 94
    "LOAD",     // 95
    "LVAR",     // 96
    "NEW",      // 97
    "OLD",      // 98
    "RENUMBER", // 99
    "SAVE",     // 9A
    "TEXTLOAD", // 9B
    "TEXTSAVE", // 9C
    "TWIN",     // 9D
    "TWINO",    // 9E
    "INSTALL",  // 9F
];

/// BASIC V extended statement tokens (prefix 0xC8), sub-byte indexed from 0x8E.
pub const ESCSTMT_TOKENS: [&str; 22] = [
    "CASE",    // 8E
    "CIRCLE",  // 8F
    "FILL",    // 90
    "ORIGIN",  // 91
    "PSET",    // 92
    "RECT",    // 93
    "SWAP",    // 94
    "WHILE",   // 95
    "WAIT",    // 96
    "MOUSE",   // 97
    "QUIT",    // 98
    "SYS",     // 99
    "INSTALL", // 9A
    "LIBRARY", // 9B
    "TINT",    // 9C
    "ELLIPSE", // 9D
    "BEATS",   // 9E
    "TEMPO",   // 9F
    "VOICES",  // A0
    "VOICE",   // A1
    "STEREO",  // A2
    "OVERLAY", // A3
];
