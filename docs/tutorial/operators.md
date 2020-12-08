# Operators

Kaki offers a number of operators, similar to many other languages.

## List of Operators

Below is every single operator and punctuation. If the overload column has a
trait name listed, then implementing that trait from `std::ops` allows a custom
type to be used with that operator. Some operators are simply not overloadable,
such as the logical and `&&`, which operates on `Bool` only.

| Operator | Description              | Example                    | Overload         |
|----------|--------------------------|----------------------------|------------------|
| `!=`     | Not equal                | `a != b`                   | `Eq` or `Ord`    |
| `!`      | Logical NOT              | `!a`                       | `Not`            |
| `%=`     | Modulo assign            | `a %= b`                   | `ModAssign`      |
| `%`      | Modulo                   | `a % b`                    | `Mod`            |
| `&&`     | Logical AND              | `a && b`                   |                  |
| `&=`     | Bitwise AND assign       | `a &= b`                   | `BitAndAssign`   |
| `&`      | Bitwise AND              | `a & b`                    | `BitAnd`         |
| `&`      | Block argument           | `fn f(&b) {}`, `f(&b)`     |                  |
| `()`     | Grouping                 | `(a + b) * c`              |                  |
| `**=`    | Power assign             | `a **= b`                  | `PowAssign`      |
| `**`     | Keyword argument         | `fn f(**x) {}`, `f(**x)`   |                  |
| `**`     | Power                    | `a ** b`                   | `Pow`            |
| `*=`     | Multiply assign          | `a *= b`                   | `MulAssign`      |
| `*`      | Multiply                 | `a * b`                    | `Mul`            |
| `*`      | Variadic argument        | `fn f(*x) {}`, `f(*x)`     |                  |
| `+=`     | Add assign               | `a += b`                   | `AddAssign`      |
| `+`      | Add                      | `a + b`                    | `Add`            |
| `,`      | Expression separator     | `a, b`                     |                  |
| `-=`     | Subtract assign          | `a -= b`                   | `SubAssign`      |
| `-`      | Negate                   | `-a`                       | `Neg`            |
| `-`      | Subtract                 | `a - b`                    | `Sub`            |
| `..=`    | Right inclusive range    | `a..=b`, `..=b`            |                  |
| `..`     | Right exclusive range    | `a..b`, `a..`, `..b`, `..` |                  |
| `.`      | Member access            | `a.b`                      |                  |
| `//=`    | Floor divide assign      | `a //= b`                  | `FloorDivAssign` |
| `//`     | Floor divide             | `a // b`                   | `FloorDiv`       |
| `/=`     | Divide assign            | `a /= b`                   | `DivAssign`      |
| `/`      | Divide                   | `a / b`                    | `Div`            |
| `::`     | Namespace access         | `a::b`                     |                  |
| `:`      | Map separator            | `{a: b}`                   |                  |
| `:`      | Type specifier           | `a: T`                     |                  |
| `;`      | Expression terminator    | `a; b`                     |                  |
| `<<=`    | Shift left assign        | `a <<= b`                  | `ShLeftAssign`   |
| `<<`     | Shift left               | `a << b`                   | `ShLeft`         |
| `<=>`    | Total comparison         | `a <=> b`                  | `Ord`            |
| `<=`     | Less than or equal       | `a <= b`                   | `Ord`            |
| `<`      | Less than                | `a < b`                    | `Ord`            |
| `==`     | Equal                    | `a == b`                   | `Eq` or `Ord`    |
| `=`      | Assign                   | `a = b`                    |                  |
| `=`      | Keyword argument default | `fn f(x = 3) {}`           |                  |
| `>=`     | Greater than or equal    | `a >= b`                   | `Ord`            |
| `>>=`    | Shift right assign       | `a >>= b`                  | `ShRightAssign`  |
| `>>`     | Shift right              | `a >> b`                   | `ShRight`        |
| `>`      | Greater than             | `a > b`                    | `Ord`            |
| `?=`     | None coalescing assign   | `a ?= b`                   |                  |
| `?`      | Optional argument        | `fn f(?x) {}`              |                  |
| `[] =`   | Subscript assign         | `a[b] = c`                 | `IndexAssign`    |
| `[]`     | List creation            | `[a, b, c]`                |                  |
| `[]`     | Subscript                | `a[b]`                     | `Index`          |
| `\\`     | Expression continuation  | `a <newline> \ + b`        |                  |
| `\|=`    | Bitwise OR assign        | `a \| = b`                 | `BitOrAssign`    |
| `\|\|`   | Logical OR               | `a \|\| b`                 |                  |
| `\|`     | Bitwise OR               | `a \| b`                   | `BitOr`          |
| `^=`     | Bitwise XOR assign       | `a ^= b`                   | `BitXorAssign`   |
| `^`      | Bitwise XOR              | `a ^ b`                    | `BitXor`         |
| `{}`     | Map creation             | `{a: b, c: d}`             |                  |
| `{}`     | Scope                    | `{a; b; c}`                |                  |
| `~`      | Bitwise NOT              | `~a`                       | `BitNot`         |

## Precendence

Operators are used for common interactions with various objects. The following
table shows all of the operators, their precedence order, and associativity.
The precedence order is sorted from high to low precendence. For example,
in `a + b * c`, the subexpression `b * c` is evaluated first as `*` has higher
precendence than `+`.

| Precedence | Symbol                      | Description                 | Associates |
|------------|-----------------------------|-----------------------------|------------|
| 1          | `.`, `::`, `()`, `[]`       | Access, grouping, subscript | Left       |
| 2          | `!`, `~`, `-`               | Not, complement, negate     | Right      |
| 3          | `**`                        | Exponentiate                | Right      |
| 4          | `*`, `/`, `//`, `%`         | Multiply, divide, modulo    | Left       |
| 5          | `+`, `-`                    | Add, subtract               | Left       |
| 6          | `..`, `..=`                 | Range                       | Left       |
| 7          | `<<`, `>>`                  | Shift                       | Left       |
| 8          | `&`                         | Bitwise AND                 | Left       |
| 9          | `^`                         | Bitwise XOR                 | Left       |
| 10         | `\|`                        | Bitwise OR                  | Left       |
| 11         | `<`, `<=`, `>=`, `>`, `<=>` | Comparison                  | Left       |
| 12         | `==`, `!=`                  | Equals, not equals          | Left       |
| 13         | `&&`                        | Logical AND                 | Left       |
| 14         | `\|\|`                      | Logical OR                  | Left       |
| 15         | `,`                         | Expression separator        | Left       |
| 16         | `=`, `?=`, `+=`, and others | Assignment                  | Right      |

Associativity can be confusing, so consider an example. There exists some
binary operator `⊕` that is used in the expression `a ⊕ b ⊕ c`:

- If `⊕` is left associative then the expression is interpreted as
  `(a ⊕ b) ⊕ c`.
- If `⊕` is right associative then the expression is interpreted as
  `a ⊕ (b ⊕ c)`.
