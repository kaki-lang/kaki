# Names

This section describes the rules for names, which are not complex, but are
different from most languages.

## Naming Rules

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
_ abstract alias break cons continue else false for fn if in
loop none pub return self Self trait true type use where while
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

## Raw Names

Raw names are a special feature that should only be used when really needed.
Raw names are enclosed in backticks `` `...` ``, and can contain any Unicode except
control sequences. Some examples include:

```kaki
# Characters normally not allowed in names can be
# used, such as spaces
fn `add two things`(a, b) { a + b }

# Names can be the same as reserved words
`if` = true

# Even things like emojis can be used
type `Spooky 👻` {}
```

Raw names do not follow the same strict rules as regular names. For example:

```kaki
type `lowercase name` {}
```

is fine, even though the name begins with a lowercase letter. Name checking
rules are ignored when raw names are used.

Raw names are the same as their regular counterparts, provided the name is
valid as a regular name. For example:

```kaki
`some_value` = 123

`some_value` #=> 123
some_value #=> 123
```

Since `some_value` is a valid name, it can be referenced without the
surrounding `` ` ``.

Raw names can easily be abused, so there are some guidelines on when they
should be used:

1.  For backwards compatibility and language interoperability. For example,
    suppose a library exports a function named `async`, and in a future version
    of the language `async` is made into a keyword. To use this library
    function, it can be referenced with `` `async` ``.
2.  For referencing operators. Consider the `+` operator. It is also provided
    as a function, which is named `` `+` ``. This allows it to easily be passed
    around as a higher order function. COnsider the following example:

    ```kaki
    # The following expressions are equivalent
    3 + 8 #=> 11
    `+`(3, 8) # 11

    # `+` can be used as a higher order function.
    # The following expressions are equivalent.
    [1, 2, 3].fold(0) { |a, b| a + b } #=> 6
    [1, 2, 3].fold(0, &`+`) #=> 6
    ```
3.  For providing test case names. Good tests can be made even better by having
    good names.

    ```kaki
    fn `shortest path in a directed acyclic graph`() {
      # ...
    }
    ```
