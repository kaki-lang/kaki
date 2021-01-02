# Advanced Typing

This section discusses some advanced concepts related to types and traits. Far more functionality is offered than what was seen in

> TODO Trait disambiguation

> TODO Operator overloading

## Types of Types

Types and traits themselves have a type, they are `Type` and `Trait`
respectively.

There are also more powerful types available, such as `Types` and `Traits`
(note the plural). These represent sets of types and traits, and can be used to
specify type constraints. `Traits` is simpler than `Types`, so it is presented
first.

`Traits` is used to represent a set of traits. It is created by _adding_ traits
together. For example, suppose that a type that implements `Hash` and `Order`
is needed, then it can be specified by the expression `Hash + Order`.
Arbitrarily many traits can be _added_, and the result is of types `Traits`.
When `Traits` is used as a constraint, it requires a type which implements all
of the traits in the trait set.

`Types` is similar to `Traits`, but specifies a set of type constraints of
which at least one ust be satisfied. A `Types` can be created by _ORing_ types
together, such as `Int | List`. This means that a type of `Int` or `List` can
satisfy the constraint. Traits can also be included, for example,
`Int | List | Hash + Order` means that a type of `Int`, `List`, or one that
implements `Hash + Order` can satisfy the constraint.

To create a named type containing these constraints, the `alias` keyword can be
used.

```kaki
alias HashOrd = Hash + Order
alias IntList = Int | List
```

## Specifying a Type

Types are always optional, but can be specified to help eliminate errors. A
variable can be given a type by including it after the variable name.

```kaki
# No type constraint
x = 64

# A simple type constraint
x Int = 64

# A traits constraint. Note that x is still an Int,
# but Hash + Order is used for validation
x Hash + Order = 64
```

An error is generated if the type constraint fails.

```kaki
# Oops! An Int can't be used where a String is expected
x String = 64
```

Types can also be placed on functions. Here is a function that accepts 2 `Int`s
and returns a `String`:

```kaki
fn format_add(a Int, b Int) String {
  "{} + {} = {}".fmt(a, b, a + b)
}
```

Types can be specified on positional and keyword arguments (but not variadic `*`
and catch-all keyword `**` arguments). Optional arguments can have a type
specified, which should be a `Types` that includes `None`, otherwise an error
will be generated is the argument is not supplied. Below is an example of how
to specify types of all of these arguments:

```kaki
fn f(
  a Int,
  ?b Int | None,
  *c, # Cannot specify type
  d: Bool | String,
  ?e: Order + Hash | None,
  **f, # Cannot specify type
  &g, # Is always expected to be Fn
) List {
  # Function body
}

fn g(?&h) { # Is always expected to be Fn | None
  # Function body
}
```

Type checking on variadic, catch-all, and optional block arguments must be
performed inside the function.

When complex types are involved in the function signature, a where clause can
be used to make the function more readable.

```kaki
fn f(a T, b U, c: V) W where
  T = Bool | Int,
  U = Set | HashMap,
  V = Float | String,
  W = Hash + Order
{
  # Function body
}
```

The above is equivalent to the following, which uses aliases instead.

```kaki
alias T = Bool | Int
alias U = Set | HashMap
alias V = Float | String
alias W = Hash + Order

fn f(a T, b U, c: V) W {
  # Function body
}
```

The benfit of the `where` clause over using `alias` is that `where` prevents
crowding the global namespace. An alias should only be made if that constraint
is going to be frequently used.

Of course, `where` can be used in a types and traits as well.

```kaki
type Vec3 {
  pub cons new(x N, y N, z N) where N = Int | Float {
    @x = x
    @y = y
    @z = z
  }
}
```

## Anonymous Types and Traits

!> TODO This section needs improvement

It would be useful if types (and some times) traits could be created on the
fly, like as a return value from a function. This is simple: don't give the it
a name and it will be anonymous. Assuming the following traits exist:

```kaki
trait T1 {}
trait T2 {}
trait T3 {}
```

there are two forms or anonymous types and two forms of anonymous traits:

```kaki
# 1 - anonymous type
type {
  # ...
}

# 2 - anonymous type implementing traits
type: T1, T2, T3 {
  # ...
}

# 3 - anonymous trait
trait {
  # ...
}

# 4 - anonymous trait implementing traits
trait: T1, T2, T3 {
  # ...
}
```

An example of an anonymous type is to create a type definition as a closure. In
this example, a type is created that increments a value by a specified number
every time the `increment()` method is called.

```kaki
fn count_by(step_size) {
  type {
    cons new() {
      @count = 0
    }
    increment() {
      @count = @count + step_size
    }
    count { @count }
  }
}

count_by_1 = count_by(1)
count_by_1.increment()
count_by_1.increment()
count_by_1.increment()
println(count_by_1.count) #=> 3

count_by_5 = count_by(5)
count_by_5.increment()
count_by_5.increment()
count_by_5.increment()
println(count_by_5.count) #=> 15
```

Of course we could have named this anonymous type in the declaration, but we
never needed its name so why bother?

Another reason is to create a type instance without formally naming it:

```kaki
pair = type {
  cons new(left, right) {
    @left = left
    @right = right
  }
  left { @left }
  right { @right }
}.new(4, 7)
println("left = {}, right = {}", pair.left, pair.right)
```

Anywhere a variable can be created, an anonymous type or trait can be used as well.