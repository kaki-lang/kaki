# Names

Having strict naming conventions helps produce high quality and consistent
code. It will become clear that the rules are very strict about what names can
be. This isn't just useful for the programmer, but useful for the language
itself. Names have strict rules so that they can unambiguously represent a
certain concept, allowing optimizations that would not otherwise be possible
for an interpreted language.

The naming rules are:

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

When names end with a `?` or `!`, they give hints to what the name does. Names
ending with `?` indicate a check that returns a `Bool`, like asking a question.
Names ending with `!` do something unexpected. For example, the `sort` method
produces a new collection and the `sort!` method modifies a collection in
place, or how `fmt` uses index substitution and `fmt!` uses named substitution.

In summary:

```kaki
# Variables, methods, properties
x _value horizontal_speed read! mutable? f12

# Fields
@x @_value @horizontal_speed @read! @mutable? @f12

# Constants
ABC DELIMITER MAX_VALUE _LIMIT

# Types
List OrderedSet _Element

# Anonymous names
_0 _1 _2 _3 _17
```

The following words are reserved by the language, and cannot be used for any
of the above:

```kaki
_ abstract break cons continue else false for func if in
loop none pub return self Self trait true type use while
```

The underscore `_` on its own is a special name with special semantics. It is a
name that is write only, which means it can never be used to store a value for
later. For example, consider when the return value of an expression is not
needed or an argument to a function is unused:

```kaki
# Unused value from an expression
first, _ = [15, 27]

# Unused function argument
func print_info(header, _)
  println("The info is '{}'", header)
}
```
