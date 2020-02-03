# Atoms

Atoms are the fundamental building blocks and cannot be decomposed into
anything smaller. Additionally, atoms are _immutable_ and can never be changed
after creation. Below are the types of atoms that the languaage defines.

## Booleans

Booleans are of type `Bool`, and can take on the value `true` or `false` to
represent truthiness.

## Integers

Integers are of type `Int`, and can take on any integral value of arbitrary
precision. Integers can be expressed in decimal, binary, octal, and
hexadecimal. `_` can be used for clarity as long as they are not the first or
last character, and are not adjacent to the base marker (like `0b` in binary
literals).

```kaki
# Decimal:
0 50 1_000_000
# Binary
0b10101001 0b1111_011
# Octal
0o12345670 0o123_005_774
# Hexadecimal
0x1234567890abcdefABCDEF 0xfe_23_06
```

## Floats

Floats are of type `Float`, and can take on any value represented by an IEEE
754 64-bit floating point type.

```kaki
# Regular declaration
0.0 23.45 1_057.1 3.141_593

# Scientific notation
1e9 2.5e-4 2_712.349_753e10
```

The `_` rules for floats are:

- `_` cannot appear at the start or end of the number.
- `_` cannot appear beside the `.`.
- `_` cannot appear directly before the `e` of the exponent.
- `_` cannot appear after the `e` of the exponent.

## Strings

Strings are of type `String`, and represent textual data as a sequence of
Unicode code points encoded in UTF-8. Strings are delimited in a few ways:

* `"` delimited strings can only span one line.
* `"""` delimited strings are _multiline_ and can span any number of lines in
  the source.
* Multiline strings that are prefixed with `@` are _smart_ strings. All leading
  and trailing whitespace is stripped, all shared leading whitespace on each
  line is stripped, and all trailing whitespace on each line is stripped.

```kaki
# This type of string must start and end on the same line
"This is a single line string"

# This type of string can start and end anywhere
"""
This is a multiline string.
It can span
  any
    number
      of lines
that you want.
"""

# It can also be a single line
"""Single line"""

# This is a smart string
@"""

  This is some string
    that has multiple lines
      indented differently

"""
# and it is the same as
"This is some string" +
"  that has multiple lines" +
"    indented differently" +
```

Certain characters can be escaped in strings:

- `\n` Newline
- `\r` Carriage return
- `\t` Tab
- `\\` Backslash
- `\0` Null
- `\"` Double quote
- `\u{UUUUUU}` a unicode escape of up to 24 bits, where each `U` is a hex
  digit. That means that there can be 1-6 hex digits in the curly braces.

Note that unicode can be used directly in strings.

```kaki
# These are both the same string
"This is a 🦀"
"This is a \u{1F980}"
```

## None

The `None` type only has one value: `none`. It represents the absence of any
other value. A useful property of `none` is that its truth value is `false`.
