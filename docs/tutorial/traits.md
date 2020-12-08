# Traits

**TODO Intro**

**TODO Explain operator overloading**

## Definition

Traits are used to add functionality to a type. They describe what a type can
do. Traits consist of only instance and static methods and properties.
Additionally, traits  can mark methods as `abstract`, which means that whatever
implements the trait must define it.

```kaki
trait Greater {
  abstract greater?(other)
  pub self > other { self.greater?(other) }
  pub self < other { other.greater?(self) }
}
```

This trait steals similar logic from the built-in `Order` trait, which defines
comparison and equality operators (`>`, `>=`, `<=`, `<`, `==`, and `!=`) in
terms of a single method.

Something that implements `GreaterThan` must define the `greater?` method.

```kaki
type Pair : GreaterThan {
  pub cons new(x, y) {
    @x = x
    @y = y
  }
  pub magnitude {
    (x ** 2 + y ** 2) ** 0.5
  }
  greater?(other) {
    self.magnitude > other.magnitude
  }
}
```

Then the `>` operator can be used like:

```kaki
p = Pair.new(3, 6)
q = Pair.new(4, 5)
println(p > q)
```

## Visibility

Notice that the `GreaterThan` trait used two modifiers that affect visibility:
`abstract` and `pub`.

- `abstract` methods always have an implicit `pub` modifier.
- `pub` methods are visible everywhere: to implementing types and traits, and
  anywhere else they appear.

Like with types, trait methods that are not `pub` are only available for use in the trait itself.

```kaki
trait Counter {
  cons {
    @count = 0
  }
  pub increment() {
    @count = @count + 1
    if self.special?() {
      println("Congratualtions, you counted to {}", @count)
    }
  }
  special?() {
    @count == 100
  }
}
```

Types that implement `Counter` only see the `increment()` method, but `Counter`
can still use `special?()` in its own internal logic.

## Resolution

If a trait implements another trait, it must either implement any abstract
methods, or declare them abstract. It does not need to declare the argument
list again, only the name. For example:

```kaki
trait A {
  pub abstract some_method(a, b, c)
}

trait B : A {
  pub abstract some_method
}
```

When there is a complex tree of traits involved, a process called _trait
linearization_ is used to resolve any name conflicts. Consider the following
hierarchy:

When multiple traits are included on a type, the order that they are included
must be determined so that the methods on the resulting type make sense and are
predictable. The rule used to determine this is that if there is a naming
conflict, the leftmost trait with that name will provide the implementation for
it.

```kaki
trait A {}
trait B {}
trait C {}
trait D : A {}
trait E : D, B {}
type T : D, B, E, A {}
```

The following dependency graph can then be made for `T`.

```
     C
     |
A    D   B
|     \ /
D  B   E  A
 \/     \/
  \     /
   \   /
    \ /
     T
```

When the type is constructed, the order that the traits are included in the type
may not actually be the order that they are specified in the list. A
linearization is performed so that each trait is included once and so that the
conflicts are resolved in a predictable way. Name conflicts are resolved by
selecting the name that appears left most in the trait list on the type.

The first step in determining trait inclusion order is to perform a depth first
search from the left to the right, which gives

```
D A B E D C B A
```

Now, all duplicates are removed from the list and the first occurrences of each
are kept.

```
D A B E C
```

These are then included in the type in **reverse** order, which is:

```
C E B A D
```

That means that the following steps are taken when `T` is created.

> **Note**: All traits and types implicitly implement the `Any` trait, and
> it is also the least significant in the inclusion order.

1.  Create an empty type `T`
2.  Add methods from `Any` to `T`
3.  Add methods from `C` to `T`
4.  Add methods from `E` to `T`
5.  Add methods from `B` to `T`
6.  Add methods from `A` to `T`
7.  Add methods from `D` to `T`
8.  Add methods defined on `T`

Each step overwrites any conflicting names that already exist on `T`, resulting
in the leftmost trait in the list contributing the implementation of a name
should there be any name conflicts.
