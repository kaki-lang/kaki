# Functions

## Function Definition

A new function can be created using `func`:

```kaki
func add(x, y) {
  x + y
}
```

It if then used by placing arguments between `(` and `)`:

```kaki
# This produces the value 10
add(7, 3)
```

The `func` keyword creates an `Fn` instance, so the above definition is the
same as:

```kaki
add = Fn.new { |x, y|
  x + y
}
```

As an aside, `func` can be used in a similar way to `Fn.new`:

```kaki
add = func(x, y) {
  x + y
}
```

## Block Arguments

When creating an `Fn`, a _block argument_ is used, which defines an anonymous
function. The anonymous function is wrapped and accessed like a normal
function. Block arguments can be used in other instances, like `map`.

```kaki
squares = [1, 2, 3].map { |x| x ** 2 }
```

To create a function that accepts a block argument, the `&` syntax must be
used. Only a single block argument is allowed. If the block argument is not
supplied, it will be `none`. Block arguments can be passed either anonymously
or by name with `&`, as shown below.

```kaki
# Create a function which accepts a block
func f(a, b, &block) {
  block(a, b)
}

# Pass an anonymous block argument
println(f(2, 3) { |a, b| a + b })

# Pass a named block argument
func add(a, b) {
  a + b
}
println(f(2, 3, &add))
```

## Argument Types

The argument types below can be specified, and must appear in the following
order:

1.  **Positional** arguments are the normal type of arguments that are
    specified one after another.
2.  **Optional** arguments are prefixed with `?`. Optional arguments are
    positional, but do not necessarily need to be supplied. They take on a
    default value of `none` if not specified. A different default value can
    also be specified with `?name: default`.
3.  **Variadic** arguments allow an unlimited number of arguments to be
    supplied. There can be one variadic argument collector per function, which
    is prefixed with `*`. The collector will collect all of the arguments in a
    list. If no variadic arguments are given, then it takes the value of the
    empty list, `[]`
4.  **Keyword** arguments are arguments which are always passed by name, but
    are position independent. Keyword arguments are specified and passed in the
    form `name: value`. A catch all of variable for keywords (similar to
    variadic) using a prefix `**`. If none are given then the value is the
    empty dictionary, `@{}`.
5.  **Block** arguments are specified with a prefix `&`, and can be passed as a
    closure. Blocks are required, but can be made optional by prefixing with
    `?&` instead of `&`, in which case the block argument is `none` if not
    supplied.

Let's take a look at all of these individually, then how they play together.

The simplest are positional arguments, which we saw already.

```kaki
# Positional arguments
func f(a, b) {}
f(1, 2) # a = 1, b = 2
```

Next are optional arguments.

```kaki
# Optional arguments
func f(?a, ?b: 50) {}
f()     # a = none, b = 50
f(1)    # a = 1,    b = 50
f(1, 2) # a = 1,    b = 2
```

Next are variadic arguments.

```kaki
# Variadic arguments
func f(*xs) {}
f()        # xs = []
f(1)       # xs = [1]
f(1, 2)    # xs = [1, 2]
f(1, 2, 3) # xs = [1, 2, 3]

# Splat all variadic arguments
f(*[1, 2, 3]) # xs = [1, 2, 3]
f(*[1, 2, 3]) # xs = [1, 2, 3]
```

Keyword arguments are a bit more complex.

```kaki
# Basic keyword arguments
func f(a: 1, b: 2) {}
f()                # a = 1,  b = 2
f(a: 10)           # a = 10, b = 2
f(b: "two")        # a = 1,  b = "two"
f(a: 10, b: "two") # a = 10, b = "two"

# Catch all keyword arguments
func f(a: 1, b: 2, **kws) {}
f()                      # a = 1,  b = 2, kws = @{}
f(a: 10, c: 50)          # a = 10, b = 2, kws = @{"c": 50}
f(a: 10, c: 50, d: true) # a = 10, b = 2, kws = @{"c": 50, "d": true}

# Splat all keyword arguments
f(b: 30, **@{"a": 1000, "d": false}) # a = 1000, b = 30, kws = @{"d", false}
```

We already saw one way to pass block arguments, but there are more options.

```kaki
# Block arguments
func f(&block) {}

# Anonymous block
f() { |a, b|
  a + b
}

# Parantheses are optional
f { |a, b|
  a + b
}

# Named block
func g(a, b) {
  a + b
}
f(&g)

# Optional block
func h(?&block) {}
f() # block = none
```

These can be combined all together, but the order of the arguments must be the
same as the above.

```kaki
func fn(a, ?b, *c, d: "hello", **e, &f) {}

func block_func(n, m) {
  n * m
}

fn(1, 2, 3, 4, 5, *[6, 7], 8,
  z: 50, **@{"d": 30, "x": false}, y: 10,
  &block_func)

# The function arguments to fn are:
a = 1
b = 2
c = [4, 5, 6, 7, 8]
d = 30
e = @{"x": false, "y": 10, "z": 50}
f = block_func
```

## Block Shorthand

When a block consisting of only positional arguments is required, it is
necessary to supply an argument list. Consider:

```kaki
squares = [1, 2, 3].map { |x| x ** 2 }
```

That is a lot of work to simply square each number in the list. Instead, this
can be written with an implicit anonymous function.

```kaki
squares = [1, 2, 3].map { $ ** 2 }
```

These two expressions are equivalent. When `$` appears in an expression, it
turns that expression into a function body. In fact, multiple arguments can be
used through enumeration, where `$0`, `$1`, `$2`, and so on are positional
arguments. The `$` is an alias for `$0`. In the following example, both
expressions are equivalent for computing the sum of a list.

```kaki
sum = [1, 2, 3].fold(0) { |acc, x| acc + x }
sum = [1, 2, 3].fold(0) { $0 + $1 }
```
