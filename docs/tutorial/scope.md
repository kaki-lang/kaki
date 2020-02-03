# Scope

The scope of a name is the context in which it can be used. In the following
example, everything is in the same scope.

```kaki
a = 5
b = 10
println(a)
println(b)
```

However, whenever `{` and `}` are used, a new scope is defined.

```kaki
# Parent scope
a = 5
{
  # Child scope
  b = 10
  println(a)
  println(b)
  # b goes out of scope here
}
println(a)
# This panics because `b` is not in scope here
println(b)
```

This shows that child scope can access any names that its parent scope can,
but this is not a symmetric relationship. A parent scope cannot access the
names declared in the child scope.
