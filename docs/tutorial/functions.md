# Functions

## Function Definition

A new function can be created using `fn`:

```kaki
fn add(x, y) {
  x + y
}
```

It is then applied by passing arguments between `(` and `)`:

```kaki
# This produces the value 10
add(7, 3)
```

The `fn` keyword creates an `Fn` instance, so the above definition is the
same as:

```kaki
add = Fn.new { |x, y|
  x + y
}
```

## Block Arguments

When creating an `Fn` directly, a _block argument_ is used, which defines an
anonymous function. The anonymous function is wrapped and accessed like a
normal function. Block arguments can be used in other instances, like `map`,
which applies a function to every item in a sequence.

```kaki
squares = [1, 2, 3].map { |x| x ** 2 }
```

To create a function that accepts a block argument, the `&` syntax must be
used. Only a single block argument is allowed. Block arguments can be passed
either anonymously or by name with `&`, as shown below.

```kaki
# Create a function which accepts a block
fn f(a, b, &block) {
  block(a, b)
}

# Pass an anonymous block argument
println(f(2, 3) { |a, b| a + b })

# Pass a named block argument
fn add(a, b) {
  a + b
}
println(f(2, 3, &add))

# Pass an expression as a block argument
println(f(2, 3, &Fn.new { |a, b| a + b }))
```

## Argument Types

The argument types below can be specified, and must appear in the following
order:

1.  **Positional** arguments are the normal type of arguments that are
    specified one after another.
2.  **Optional** arguments are prefixed with `?`. Optional arguments are
    positional, but do not necessarily need to be supplied. They take on a
    default value of `none` if not specified.
3.  **Variadic** arguments allow an unlimited number of arguments to be
    supplied. There can be one variadic argument collector per function, which
    is prefixed with `*`. The collector will collect all of the arguments in a
    list. If no variadic arguments are given, then it takes the value of the
    empty list, `[]`
4.  **Keyword** arguments are arguments which are always passed by name, but
    are position independent. Keyword arguments are specified and passed in the
    form `name = value`. A keyword aergument is specified in the function
    signature using a trailing `=`, such as `a=`. Keyword arguments are
    required unless specified with a leading `?`, such as `?a=`. When optional
    keywords are not specified they take on the value of `none`. A catch all
    variable for keywords (similar to variadic arguments) can be defined using
    a prefix `**`. If none are given then the value is the empty hash map,
    `{}`.
5.  **Block** arguments are specified with a prefix `&`, and can be passed as a
    closure. Blocks are required, but can be made optional by prefixing with
    `?&` instead of `&`, in which case the block argument is `none` if not
    supplied.

Let's take a look at all of these individually, then how they play together.

The simplest are positional arguments, which we saw already.

```kaki
# Positional arguments
fn f(a, b) {}
f(1, 2) # a = 1, b = 2
```

Next are optional arguments.

```kaki
# Optional arguments
fn f(?a, ?b) {}
f()     # a = none, b = none
f(1)    # a = 1,    b = none
f(1, 2) # a = 1,    b = 2
```

Next are variadic arguments.

```kaki
# Variadic arguments
fn f(*xs) {}
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
fn f(a=, ?b=) {}
f()                  # Error, a is required
f(a = 10)            # a = 10, b = none
f(a = 10, b = "two") # a = 10, b = "two"

# Catch all keyword arguments
fn f(a=, b=?, **kws) {}
f(a = 10, c = 50)           # a = 10, b = none, kws = {"c": 50}
f(a = 10, c = 50, d = true) # a = 10, b = none, kws = {"c": 50, "d": true}
# Splat all keyword arguments
f(b = 30, **{"a": 1000, "d": false}) # a = 1000, b = 30, kws = {"d": false}
```

We already saw one way to pass block arguments, but there are more options.

```kaki
# Block arguments
fn f(&block) {}

# Anonymous block
f() { |a, b|
  a + b
}

# Parantheses are optional
f { |a, b|
  a + b
}

# Named block
fn g(a, b) {
  a + b
}
f(&g)

# Optional block
fn h(?&block) {}
f() # block = none
```

These can be combined all together, but the order of the arguments must be the
same as the above.

```kaki
fn f(a, ?b, *c, d=, **e, &f) {}

fn block_func(n, m) {
  n * m
}

f(1, 2, 3, 4, 5, *[6, 7], 8,
  z = 50, **{"d": 30, "x": false}, y = 10,
  &block_func)

# The function arguments to f are:
a = 1
b = 2
c = [3, 4, 5, 6, 7, 8]
d = 30
e = {"x": false, "y": 10, "z": 50}
f = block_func
```

## Argument None Coalescing

We saw that when an optional argument is not specified, that it takes on the
value `none`. Defaults can quickly be given to these arguments using the none
coalescing operator, `?=`.

```kaki
fn f(?a, ?b=) {
  a ?= 10
  b ?= 20
  a + b
}

f()       #=> 30
f(5)      #=> 25
f(b=100)  #=> 110
f(1, b=2) #=> 3
```

## Block Shorthand

Consider a block function consisting of only positional arguments, which is the
most common use case for a block. One such case is the `map` method of a
`Sequence`, as shown below.

```kaki
squares = [1, 2, 3].map { |x| x ** 2 }
```

That is a lot of work to simply square each number in the list. Instead, when
only positional arguments are required, this can be written with an implicit
anonymous function.

```kaki
squares = [1, 2, 3].map { _0 ** 2 }
```

These two expressions are equivalent. When `_0` appears in an expression, it
turns that expression into a function body. In fact, multiple arguments can be
used through enumeration, where `_0`, `_1`, `_2`, and so on are positional
arguments. In the following example, both expressions are equivalent for
computing the sum of a list.

```kaki
sum = [1, 2, 3].fold(0) { |acc, x| acc + x }
sum = [1, 2, 3].fold(0) { _0 + _1 }
```

If any type of argument other than positional are required in a the block, then
this shorthand cannot be used.
