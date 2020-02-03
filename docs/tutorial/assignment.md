# Assignment

Assignment is the act of pointing a name to a value. Consider this code:

```kaki
a = 5
a = 8
```

The first line creates a value `5` then assigns the name `a` to it. The second
line creates a value `8`, then removes the name `a` from the value `5` and
instead assigns the name `a` to the newly created `8`.

One of the more advanced ways that assignment can be used is to assign multiple
names to the same value:

```kaki
value = "cake"
a = b = c = value
```

which is equivalent to:

```kaki
c = value
b = value
a = value
```

## Unpacking

Another assignment feature is sequence unpacking.

```kaki
x, y, z = [2, 5, 4] # x = 2, y = 5, z = 3
```

Collections with more or fewer items than the number of names on the left side
of the `=` can also be unpacked. Longer or shorter lists can be unpacked as
well using the `?` and `*` syntax, similar to functions.

Optional names can be unpacked using the optional argument `?` syntax:

```kaki
x, ?y, ?z = ["hello", "there"]
# x = "hello", y = "there", z = none
```

Variadic names can be unpacked using the variadic argument `*` syntax:

```kaki
x, y, *z = [1, 2]       # x = 1, y = 2, z = []
x, y, *z = [1, 2, 3]    # x = 1, y = 2, z = [3]
x, y, *z = [1, 2, 3, 4] # x = 1, y = 2, z = [3, 4]
```

A mix of optional and required names can be specified:

```kaki
x, ?y, *z = [1]       # x = 1, y = none, z = []
x, ?y, *z = [1, 2]    # x = 1, y = 2,    z = []
x, ?y, *z = [1, 2, 3] # x = 1, y = 2,    z = [3]
```

In all cases, the names must be ordered from left to right as required,
optional, then variadic.

## None Coalescing

The `?=` is a special operation called the _none coalescing assignment_. An
assignment like

```kaki
speed ?= 5
```

assigns the name `speed` to the value `5` if one of the following conditions is
satisifed:

1.  The name `speed` does not exist.
2.  The name `speed` is assigned to the value `none`.
