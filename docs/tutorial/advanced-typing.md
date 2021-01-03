# Advanced Typing

This section discusses some advanced concepts related to types and traits. Far
more functionality is offered than what was seen in the types and traits
sections.

## Types of Types

Types and traits themselves have a type, they are `Type` and `Trait`
respectively. There are also more powerful types available, such as `Types` and
`Traits` (note the plural). These represent sets of types and traits, and can
be used to specify type constraints. `Traits` is simpler than `Types`, so it is
presented first.

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

Types can be specified on every argument, however, there are some redundancies:

* Variadic positional arguments (`*`) are always of type `List`.
* Variadic keyword arguments (`*`) are always of type `HashMap`.
* Block arguments (`&`) are always of type `Fn`.
* Optional block (`?&`) are always of type of `Fn | None`.

It is best practice not to specify types for these arguments since they are
already implicitly handled.

When optional arguments are specified, they must have an option to be `None`,
such as `Int | None`.

Below is an example of how to specify types for all different arguments:

```kaki
fn f(
  a Int,
  ?b Int | None,
  *c List, # Redundant: is always List
  d: Bool | String,
  ?e: Order + Hash | None,
  **f HashMap, # Redundant: is always HashMap
  &g Fn, # Redundant: is always Fn
) List {
  # ...
}

fn g(?&h) { # Redundant: is always Fn | None
  # ...
}
```

Type checking on variadic positional, variadic keyword, and optional block
arguments must be performed inside the function.

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

## Trait Disambiguation

There are two notable cases where traits can have methods with the same names:

1.  Traits have the same names for abstract methods and the abstract methods
    require different implementations,
2.  Traits have the same names for provided methods and the caller wants to use
    a specific one.

Consider two traits, `TraitA` and `TraitB`:

```kaki
trait TraitA {
  abstract message()
  pub show() {
    "From TraitA: {}".fmt(self.message())
  }
}

trait TraitB {
  abstract message()
  pub show() {
    "From TraitB: {}".fmt(self.message())
  }
}
```

Both traits require abstract methods of the same name, `message()` and provide
methods of the same name, `show()`.

To provide separate implementations for these traits, they can be disambiguated
at the time of definition by including the trait name in the method
declaration.

```kaki
type SomeType: TraitA, TraitB {
  pub cons new() {}

  TraitA.message() {
    "Hello!"
  }

  TraitB.message() {
    "Greetings!"
  }

  pub greet() {
    # Which message() is going to be called?
    self.message()
  }
}
```

The `greet()` method calls `self.message()`, but will it call the method on
`TraitA` or `TraitB`? The `message()` method from `TraitA` will be used,
because it appears earlier in the trait list on `SomeType`. This does not
depend on the order that `SomeType` declares its methods, it is decided
entirely by trait linearization.

Given an instance of `SomeType`, if the `message()` method is called on it, is
`TraitA.message()` or `TraitB.message()` called? It is the same as before,
where `TraitA.message()` is used. If the desired behaviour is to use
`TraitB.message`, then a cast must be performed. There are a number of ways to perform a cast.

```kaki
s = SomeType.new()

# Cast on assignment
b TraitB = s

# Cast as an expression
b = (s TraitB)
```

Both types of casting shown have different uses. A cast on assignment is useful
if casts are frequently, but introduces a new variable. Casts on assignment are
implicit in functions which specificy a trait type. Casts as an expression are
useful as part of a larger expression. Both uses are shown below.

```kaki
s = SomeType.new()
b TraitB = s

fn show_as_trait_b(b TraitB) {
  b.show()
}

s.show()           #=> "From TraitA: Hello!"
b.show()           #=> "From TraitB: Greetings!"
show_as_trait_b(s) #=> "From TraitB: Greetings!"
(s TraitB).show()  #=> "From TraitB: Greetings!"
```

## Anonymous Types and Traits

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
  }.new()
  # Here the new() constructor was called at the end
  # to return an instance, rather than the type itself
}

count_by_1 = count_by(1)
count_by_1.increment()
count_by_1.increment()
count_by_1.increment()
count_by_1.count #=> 3

count_by_5 = count_by(5)
count_by_5.increment()
count_by_5.increment()
count_by_5.increment()
count_by_5.count #=> 15
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
  pub left { @left }
  pub right { @right }
}.new(4, 7)

"left = {}, right = {}".fmt(pair.left, pair.right)
#=> "left = 4, right = 7"
```

Anywhere a variable can be created, an anonymous type or trait can be used as
well.

## Operator Overloading

Operators can be overloaded by implementing specific traits. Recall the `Vec3`
type defined in the types section:

```kaki
type Vec3 {
  pub cons new(x, y, z) {
    @x = x
    @y = y
    @z = z
  }
  pub x { @x }
  pub y { @y }
  pub z { @z }
}
```

When vectors are added, their elements are added pairwise to produce a new
vector of the same length. This functionality will be added by implementing
the `Add` trait.

```kaki
type Vec3: Add {
  # ...

  Add.add(other) {
    Self.new(@x + other.x, @y + other.y, @z + other.z)
  }
}

v1 = Vec3.new(1, 2, 3)
v2 = Vec3.new(6, -7, 8)
v3 = v1 + v2

v3.x #=> 7
v3.y #=> -5
v3.z #=> 11
```

## Reflected Operators

Binary operators have _reflected_ versions which allow for the order of the
operands to change. Consider the following implementation of addition on
`Vec3`:

```kaki
type Vec3: Add {
  # ...

  Add.add(other Int | Float | Vec3) {
    if other.is?(Vec3) {
      Self.new(@x + other.x, @y + other.y, @z + other.z)
    } else {
      Self.new(@x + other, @y + other, @z + other)
    }
  }
}
```

The definition has changed so that element-wise addition of two vectors can be
performed, or a number can be added to each component of the vector. This
allows for the following:

```kaki
v1 = Vec3.new(1, 2, 3)
v2 = v1 + 10

v2.x #=> 11
v2.y #=> 12
v2.z #=> 13
```

But not the following:

```kaki
v1 = Vec3.new(1, 2, 3)
v2 = 10 + v1 # Oops! Int.add() does not know what to do with a Vec3!
```

This is because the `Add.add()` method on the left operand is called and the
argument to the method is the right operand. When the left operand is a `Vec3`,
the `Vec3.add()` method knows how how add an `Int` to a `Vec3`, but when an
`Int` is the left operand, the `Int.add()` method does not know how to add a
`Vec3` to an `Int`.

This problem can be solved by reflecting the operator through implementing the
`AddRefl` trait.

```kaki
type Vec3: Add, AddRefl {
  # ...

  Add.add(other) {
    if other.is?(Vec3) {
      Self.new(@x + other.x, @y + other.y, @z + other.z)
    } else if other.is?(Int | Float) {
      Self.new(@x + other, @y + other, @z + other)
    } else {
      NotImplemented
    }
  }

  AddRefl.add_refl(other) {
    # Use the `Add.add()` implementation since the operator commutes.
    # For operations that do not commute, such as subtraction, this
    # implementation will have to be more complex.
    self + other
  }
}
```

A number of things got changed here:

* The type specifiers got removed from `Add.add()`, allowing it to accept a right operand of any type.
* `Add.add()` now decides internally whether it can perform the addition, and returns a `NotImplemented` if the righ operand is incompatible.
* `AddRefl` was implemented, which is the reflected version of addition that allows a `Vec3` to be the right operand.

Now, if the `10 + v1` expression from before is evaluated, the following happens:

1.  The `Int.add()` method is invoked by the `+` operator, and the argument is
    `v1`, which is equivalent to `10.add(v1)`.
2.  The `Int.add()` method does not know how to add a `Vec3` with an `Int`, so
    it returns `NotImplemented`.
3.  The `+` operator sees that a `NotImplemented` was returned, so it checks to
    see if `Vec3` implements `AddRefl`.
4.  `Vec3` does indeed implement `AddRefl`, so the `+` operator tries
    `v1.add_refl(10)`.
5.  The `Vec3.add_refl()` returns a value that is not `NotImplemented`, so the
    `+` operator considers the operation as a success, and returns that value.

This all works because `Vec3` knows how to add an `Int` to itself.

Suppose that instead of adding an `Int`, a string was used, such as
`"567" + v1`. The execution is roughly the same up until step 5, where
`Vec3.add_refl()` will return a `NotImplemented`, and an error will be
generated since both the addition and reflected addition failed.

The `+` operator was given some degree of personality above, as it was described
as deciding what to do based on the return value of `Int.add()`, the ontology of
`Vec3`, and the return value of `Vec3.add_refl()`. This is due to fundamental
aspect of what Kaki operators are: a function which takes values of any two
types. Unlike many languages where `+` is generally compiled as a single
instruction, the compilation of the `a + b` translates the expression into
`std::ops::add(a, b)`, which contains all of this logic.
