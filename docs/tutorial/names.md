# Names

Naming conventions help to produce high quality and consistent code. In Kaki,
these aren't just conventions, but rather rules that must be followed. The
naming rules are:

- The names of **variables**, **fields**, **methods**, and **properties** must
  match the regex `_*[a-z][a-z0-9_]*(!\?)?`, which means the names can start
  with 0 or more underscores, then have a lowercase letter, then contain any
  number of lowercase letters and underscores, and optionally end with a `!` or
  `?`. Fields must be prefixed with a `@` or `@@`.
- The names of **constants** must match the regex `_*[A-Z][A-Z0-9_]*`, which
  means the names can start with 0 or more underscores, then have an uppercase
  letter, then contain any number of uppercase letters, numbers, and
  underscores.
- The names of **types** must match the regex `_*[A-Z][A-Za-z0-9_]*`, which
  means the names can start with 0 or more underscores, then have an uppercase
  letter, then contain any number of alphanumeric characters and underscores.
- **Anonymous names**, which are convenience names used for implicit block
  arguments, must match the regex `_[0-9]+`. This is an underscore with one or
  more decimal digiats after it. Note that leading zeros are allowed, so `_1`
  and `_0001` are the same.
- When names end with a `?` or `!`, they give hints to what the name does.
  - Names ending with `?` indicate a check that returns a `Bool`, like asking a
    question.
  - Names ending with `!` do something unexpected. For example, the `sort`
    method produces a new collection and the `sort!` method modifies a
    collection in place, or how `fmt` uses index substitution and `fmt!` uses
    named substitution.

To give some examples:

```kaki
# Variables, methods, properties
x _value horizontal_speed read! mutable? f12

# Fields
@x @_value @horizontal_speed @read! @mutable? @f12

# Constants
ABC DELIMITER MAX_VALUE _LIMIT READONLY?

# Types and traits
List OrderedSet _Element Cell!

# Anonymous names
_0 _1 _2 _3 _17
```

The following words are reserved by the language, and cannot be used for any
of the above:

```kaki
_ abstract break cons continue else false for fn if in
loop none pub return self Self trait true type use while
```

The underscore `_` on its own is a special name with special semantics. It is a
name that is write only, which means it can never be used in a situation where
it is read, such as `println!(_)`, which will generate a syntax error. The
purpose of `_`, is to act as a black hole for values that are not needed. Some
cases are when the return value of an expression is not needed or an argument
to a function is unused:

```kaki
# Unused value from an expression
first, _ = [15, 27]

# Unused function argument
fn print_info(header, _)
  println("The info is '{}'", header)
}
```
