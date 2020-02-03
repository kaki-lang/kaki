# Operators

Operators are used for common interactions with various objects. The following
table shows all of the operators, their precedence, and associativity.

| Precendence | Symbol             | Name                        | Associates |
|-------------|--------------------|-----------------------------|------------|
| 0           | `.` `::` `()` `[]` | Access, grouping, subscript | Left       |
| 1           | `!` `~` `-`        | Not, complement, negate     | Right      |
| 2           | `**`               | Exponentiate                | Right      |
| 3           | `*` `/` `//` `%`   | Multiply, divide, modulo    | Left       |
| 4           | `+` `-`            | Add, subtract               | Left       |
| 5           | `<<` `>>`          | Shift                       | Left       |
| 6           | `&`                | Bitwise AND                 | Left       |
| 7           | `^`                | Bitwise XOR                 | Left       |
| 8           | `\|`               | Bitwise OR                  | Left       |
| 9           | `<` `<=` `>=` `>`  | Comparison                  | Left       |
| 10          | `==` `!=`          | Equals, not equals          | Left       |
| 11          | `&&`               | Logical AND                 | Left       |
| 12          | `\|\|`             | Logical OR                  | Left       |
| 13          | `,`                | Comma                       | Left       |
| 14          | `=`, `?=`          | Assignment                  | Right      |

Associativity can be confusing, so consider an example. There exists some
binary operator `⊕` that is used in the expression `a ⊕ b ⊕ c`:

- If `⊕` is left associative then the expression is interpreted as
  `(a ⊕ b) ⊕ c`.
- If `⊕` is right associative then the expression is interpreted as
  `a ⊕ (b ⊕ c)`.

To illustrate, division is left associative and exponentiation is right
associative:

```kaki
# Division: all of these are equivalent
a = 100 / 4 / 5
a = (100 / 4) / 5
a = 25.0 / 5
a = 5.0

# Exponentiation: all of these are equivalent
b = 3 ** 2 ** 4
b = 3 ** (2 ** 4)
b = 3 ** 16
b = 43046721
```
