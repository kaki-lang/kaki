# Operator Overloading

Operators can be overloaded for instances of a type. The following operators
can be overloaded:

| Operator          | Name                        | Reverse?   |
|-------------------|-----------------------------|------------|
| `()`              | Call                        |            |
| `[]`              | Subscript                   |            |
| `!` `~` `-`       | Not, complement, negate     |            |
| `**`              | Exponentiation              | Yes        |
| `*` `/` `//` `%`  | Multiply, divide, modulo    | Yes        |
| `+` `-`           | Add, subtract               | Yes        |
| `<<` `>>`         | Shift                       | Yes        |
| `&`               | Bitwise AND                 | Yes        |
| `^`               | Bitwise XOR                 | Yes        |
| `\|`              | Bitwise OR                  | Yes        |
| `<` `<=` `>=` `>` | Comparison                  | Yes        |
| `==` `!=`         | Equals, not equals          | Yes        |

## Example

Here is an example that performs vector addition with the `+` operator.

```kaki
type Vector {
  pub cons new(x, y, z) {
    @x = x
    @y = y
    @z = z
  }
  pub x { @x }
  pub y { @y }
  pub z { @z }
  pub self + other {
    Self.new(@x + other.x, @y + other.y, @z + other.z)
  }
}
```

The call and subscript operators are overridden in the same way. For example,
the subscript operator on the the above `Vector` type would be:

```kaki
type Vector {
  # ...
  pub self[i] {
    if i >= 0 && i < 3 {
      [@x, @y, @z][i]
    } else {
      panic("Index is out of range")
    }
  }
}
```

The unary operators are overridden in a similar way:

```kaki
type Vector {
  # ...
  pub -self {
    @x = -@x
    @y = -@y
    @z = -@z
  }
}
```

## Forward and Reverse Operators

The binary operators all have a reversed implementation. Suppose that the
`Vector` type needs to have an operator to scale all three elements by the same
number. The `*` operator will be used for that. Reverse operators are useful,
because `3 * vector` and `vector * 3` are two different things. However, they
can both be defined.

```kaki
type Vector {
  # ...

  # Forward operator
  pub self * c {
    @x = c * @x
    @y = c * @y
    @z = c * @z
  }

  # Reverse operator
  pub c * self {
    # Call the forward operator since this is commutative
    self * c
  }
}
```

Now both `3 * vector` and `vector * 3` will work. What is not seen here is that
when `3 * vector` is called, it returns a special value. When a type implements
a binary operator and the supplied argument is not compatible with the
operation, a special value must be returned so the reverse operator is used.

Consider this minimal example:

```kaki
type Number {
  pub cons new(n) {
    @n = n
  }

  n { @n }

  # Forward operator
  self + other {
    if Type.is?(other, Self) {
      Self.new(self.n + other.n)
    } else if Type.is?(other, Int, Float) {
      Self.new(self.n + other)
    } else {
      NotImplemented
    }
  }

  # Reverse operator
  other + self {
    self + other
  }
}

v = 5 + Number.new(7)
println(v.n)
```

The power of reverse operators is demonstrated here where an `Int` is added
with our custom defined `Number` type.

Here's what happened:

1.  The expression `5 + Number.new(7)` is encountered.
2.  The forward `+` operator on the `Int` with a value `5` is called, but it
    does not work so it returns `NotImplemented`.
3.  Since `NotImplemented` was returned, the `+` operator tries the reverse `+`
    overload on `Number`. This succeeds, since the reverse operator on `Number`
    defers to the forward operator, which is defined for the `Int` type.

Note that in the third step if the reverse operator returns the error, a panic
will be issued. Another important detail is that if both operands are the same
type, the reverse operator will never be called even if the error is returned
from the forward operator. It is assumed that the forward operator will handle
this case, and if the error is returned a panic will immediately be issued.

If something like `Number.new(10) + "20"` were used, the `+` on `Number` would
return a `NotImplemented`, which would cause the reverse operator on `String`
to be called. This will obviously lead to a panic because the `String` atom
will not know how to deal with our `Number` type.

Reverse operators **cannot** be called directly. They are only called
implicitly through the operator expression.
