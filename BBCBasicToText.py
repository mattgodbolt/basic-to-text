#!/usr/bin/env python3
#
# (c) 2007 Matt Godbolt.
# Use however you like, as long as you put credit where credit's due.
# Some information obtained from source code from RISC OS Open.
# v0.01 - first release.  Doesn't deal with GOTO line numbers.

import getopt
import re
import struct
import sys

# fmt: off
# The list of BBC BASIC V tokens:
# Base tokens, starting at 0x7f
tokens = [
    'OTHERWISE', # 7f
    'AND', 'DIV', 'EOR', 'MOD', 'OR', 'ERROR', 'LINE', 'OFF',
    'STEP', 'SPC', 'TAB(', 'ELSE', 'THEN', '<line>' # TODO
        , 'OPENIN', 'PTR',

    'PAGE', 'TIME', 'LOMEM', 'HIMEM', 'ABS', 'ACS', 'ADVAL', 'ASC',
    'ASN', 'ATN', 'BGET', 'COS', 'COUNT', 'DEG', 'ERL', 'ERR',

    'EVAL', 'EXP', 'EXT', 'FALSE', 'FN', 'GET', 'INKEY', 'INSTR(',
    'INT', 'LEN', 'LN', 'LOG', 'NOT', 'OPENUP', 'OPENOUT', 'PI',

    'POINT(', 'POS', 'RAD', 'RND', 'SGN', 'SIN', 'SQR', 'TAN',
    'TO', 'TRUE', 'USR', 'VAL', 'VPOS', 'CHR$', 'GET$', 'INKEY$',

    'LEFT$(', 'MID$(', 'RIGHT$(', 'STR$', 'STRING$(', 'EOF',
        '<ESCFN>', '<ESCCOM>', '<ESCSTMT>',
    'WHEN', 'OF', 'ENDCASE', 'ELSE' # ELSE2
        , 'ENDIF', 'ENDWHILE', 'PTR',

    'PAGE', 'TIME', 'LOMEM', 'HIMEM', 'SOUND', 'BPUT', 'CALL', 'CHAIN',
    'CLEAR', 'CLOSE', 'CLG', 'CLS', 'DATA', 'DEF', 'DIM', 'DRAW',

    'END', 'ENDPROC', 'ENVELOPE', 'FOR', 'GOSUB', 'GOTO', 'GCOL', 'IF',
    'INPUT', 'LET', 'LOCAL', 'MODE', 'MOVE', 'NEXT', 'ON', 'VDU',

    'PLOT', 'PRINT', 'PROC', 'READ', 'REM', 'REPEAT', 'REPORT', 'RESTORE',
    'RETURN', 'RUN', 'STOP', 'COLOUR', 'TRACE', 'UNTIL', 'WIDTH', 'OSCLI']

# Referred to as "ESCFN" tokens in the source, starting at 0x8e.
cfnTokens = [
    'SUM', 'BEAT']
# Referred to as "ESCCOM" tokens in the source, starting at 0x8e.
comTokens = [
    'APPEND', 'AUTO', 'CRUNCH', 'DELET', 'EDIT', 'HELP', 'LIST', 'LOAD',
    'LVAR', 'NEW', 'OLD', 'RENUMBER', 'SAVE', 'TEXTLOAD', 'TEXTSAVE', 'TWIN'
    'TWINO', 'INSTALL']
# Referred to as "ESCSTMT", starting at 0x8e.
stmtTokens= [
    'CASE', 'CIRCLE', 'FILL', 'ORIGIN', 'PSET', 'RECT', 'SWAP', 'WHILE',
    'WAIT', 'MOUSE', 'QUIT', 'SYS', 'INSTALL', 'LIBRARY', 'TINT', 'ELLIPSE',
    'BEATS', 'TEMPO', 'VOICES', 'VOICE', 'STEREO', 'OVERLAY']
# fmt: on


def Detokenise(line):
    """Replace all tokens in the line 'line' with their ASCII equivalent."""

    def ReplaceFunc(match):
        ext, token = match.groups()
        tokenOrd = token[0]
        if ext:
            if ext == b'\xc6':
                return cfnTokens[tokenOrd - 0x8E].encode('latin-1')
            if ext == b'\xc7':
                return comTokens[tokenOrd - 0x8E].encode('latin-1')
            if ext == b'\xc8':
                return stmtTokens[tokenOrd - 0x8E].encode('latin-1')
            raise RuntimeError("Bad token")
        else:
            result = tokens[tokenOrd - 127].encode('latin-1')
            # Append any extra bytes as-is
            if len(token) > 1:
                result += token[1:]
            return result

    return re.sub(rb'([\xc6-\xc8])?([\x7f-\xff][\x80-\xff]*|\xf4[^\r]*)', ReplaceFunc, line).decode('latin-1')


def ReadLines(data):
    """Returns a list of [line number, tokenised line] from a binary
    BBC BASIC V format file."""
    lines = []
    while True:
        if len(data) < 2:
            raise RuntimeError("Bad program")
        if data[0] != 0x0d:
            raise RuntimeError("Bad program (expected 0x0d, got 0x%02x)" % data[0])
        if data[1] == 0xff:
            break
        lineNumber, length = struct.unpack(">hB", data[1:4])
        lineData = data[4:length]
        lines.append([lineNumber, lineData])
        data = data[length:]
    return lines


def Decode(data, output):
    """Decode binary data 'data' and write the result to 'output'."""
    lines = ReadLines(data)
    for lineNumber, line in lines:
        lineData = Detokenise(line)
        output.write(lineData + "\n")


if __name__ == "__main__":
    optlist, args = getopt.getopt(sys.argv[1:], "")
    if len(args) != 2:
        print("Usage: %s INPUT OUTPUT" % sys.argv[0])
        sys.exit(1)
    entireFile = open(args[0], "rb").read()
    output = open(args[1], "w")
    Decode(entireFile, output)
    output.close()
