# Traits

Traits are used to add functionality to a type. They describe what a type can
do. Traits consist of only instance methods and properties, and as such, are
only useful in an instance context. Traits are similar to the concept of
_abstract classes_ in object-oriented programming, but offer greater
flexibility.

## An Example

This example shows the creation of a `ReverseItems` trait that can be used to
reverse the items of a collection. The trait provides a method named
`reverse_items()` that can be used to reverse the items of a collection an
retain the original type of the collection.

```kaki
trait ReverseItems {
  # Returns a list of the items in the collection
  abstract items_to_reverse()

  # Builds a new collection of the same type from
  # the reversed items
  abstract from_reversed_items(items)

  # The "useful" method that is actually used to
  # reverse the items. This is the one that should
  # be used outside of the type
  pub reverse_items() {
    items = self.items_to_reverse()
    reversed = items.reverse()
    self.from_reverse_items(reversed)
  }
}
```

`ReverseItems` can be implemented for the `Vec3` type from the previous
section. Recall the definition of `Vec3` to be:

```kaki
type Vec3 {
  pub cons new(x, y, z) {
    @x = x
    @y = y
    @z = z
  }

  # ...
}
```

This definition can be modified to implement the `ReverseItems` trait as:

```kaki
type Vec3: ReverseItems {
  pub cons new(x, y, z) {
    @x = x
    @y = y
    @z = z
  }

  items_to_reverse() {
    [@x, @y, @z]
  }

  from_reversed_items(items) {
    Self.new(*items)
  }

  # ...
}
```

This can be used in an example, such as:

```kaki
v = Vec3.new(1, 2, 3)
v.x #=> 1
v.y #=> 2
v.z #=> 3
r = v.reverse_items()
r.x #=> 3
r.y #=> 2
r.z #=> 1
```

The powerful feature of traits is that they can provide complex functionality
by by only requiring the type to implement a handful of methods. An example of
this is the `Order` trait, which implements the operators `<`, `<=`, `>=`, `>`,
`<=>`, `==`, and `!=` in terms of a single method supplied by the type.

## Visibility

Notice that the `ReverseItems` trait used two modifiers that affect visibility:
`abstract` and `pub`.

- `abstract` methods are private by default. They can optionally have a `pub`
   modifier if they are intended to be used outside of the trait.
- `pub` methods are visible everywhere: to implementing types and traits, and
  anywhere else they appear.

Like with types, trait methods that are not `pub` are only available for use in
the trait itself. We see that in the `ReverseItems` trait, that the abstract
methods `items_to_reverse()` and `from_reversed_items(items)` are private and
are not able to be used outside the trait or the implementing type. Since
`abstract` methods are private by default, to be used outside of the
implementing type and trait they must be declared as `pub abstract`.

## Trait Constructors and Fields

Traits may optionally use a single constructor, which is unnamed, private, and
cannot be called directly. The constructor is automatically run when the type
is instantiated, and can used to initialize any data stored in fields on trait.
Consequently, traits cannot access the fields of their implementing types.

```kaki
trait StoreSingleValue {
  cons {
    @value = None
  }

  pub value {
    @value
  }

  pub value = x {
    @value = x
  }
}
```

## Resolution

If a trait implements another trait, it must either implement any abstract
methods, or declare them abstract. It does not need to declare the argument
list again, only the name. For example:

```kaki
trait A {
  abstract some_method(a, b, c)
}

trait B : A {
  abstract some_method
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
trait D: A {}
trait E: D, B {}
type T: D, B, E, A {}
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
search from the left to the right and writing down every node in the order that
they are first encountered, which gives

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
> it is also the least significant in the inclusion order. `Any` is always
> included first regardless of where it appears in trit list.

1.  Create the empty type, `T`
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
