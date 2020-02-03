# Types

The most basic type is declared like:

```kaki
type A {}
```

but it is not very useful since it is unable to do anything.

## Constructors

Types are created to store data and operations on them. A _constructor_ is used
to build an _instance_ of a type.

```kaki
type Dog {
  pub cons new() {
    println("I am a dog!")
  }
}

Dog.new()
```

A constructor can be named anything,

```kaki
type Dog {
  pub cons red() {
    println("I am a red dog!")
  }
}

Dog.red()
```

though a generic constructor should have the name `new`.

## Fields

An instance is created using a constructor, and the instance data, called
_fields_, can be set up in a constructor. Fields hold private data, and their
names are prefixed with `@`. A field always takes the value of `none` before it
assigned.

```kaki
type Dog {
  pub cons new(name) {
    @name = name
  }
}
```

Now a `Dog` can be created with

```kaki
dog = Dog.new("Rover")
```

## Methods

_Methods_ can be added to a type, which are functions that are bound to it.

```kaki
type Dog {
  pub cons new(name) {
    @name = name
  }
  pub self.speak() {
    println("Woof! I am {}.", @name)
  }
}
```

which can be called like

```kaki
dog.speak()
```

Notice the inclusion of the new `self` name? This is a reference to the type
instance. It tells the type that this is an instance method, instead of of a
_static_ method. If the `self` is left out of the signature, the method is
implicitly an instance method. The same is true for _properties_, which are
discussed in the next section. Knowing this, the above definition can be
written as:

```kaki
type Dog {
  # ...
  pub speak() {
    println("Woof! I am {}.", name)
  }
}
```

Methods can accept all of the same argument types that functions do, and
additionally, they implicitly define a value named `self`, which is a reference
to the instance of the type. This is useful for calling other methods.

```kaki
type Dog {
  pub cons new(name, color) {
    @name = name
    @color = color
  }
  pub name() { @name }
  pub color() { @color }
  pub description(something_nice) {
    "My dog {} is {} and {}!".fmt(self.name(), self.color(), something_nice)
  }
}

dog = Dog.new("Rover", "brown")
println(dog.description("he is the best"))
# My dog Rover is brown and he is the best!
```

## Properties

Consider the previous example, this is a useful time to use _properties_.
Properties are like methods, but they are used for accessing data.

```kaki
type Dog {
  # ...
  pub name { @name }
  pub color { @color }
  pub description(something_nice) {
    "My dog {} is {} and {}!".fmt(self.name, self.color, something_nice)
  }
}
```

Properties also allow modifying data.

```kaki
type Dog {
  # ...
  pub name { @name }
  pub name = name { @name = name }
  pub color { @color }
  pub color = color { @color = color }
  # ...
}
```

Since properties are just methods, they can run any arbitrary code. This is
useful for patterns that involve generating data or verifying that the value
being set is valid.

## Visibility

Notice how we have been declaring constructors, methods, and properties as
`pub`? This means that they are for public use, and anyone can use them. If we
didn't declare them pub, they would only be available within the type.

Let's see how this works.

```kaki
type Circle {
  pub cons new(radius) {
    @radius = radius
  }
  pub circumference {
    3.141_593 * self.diameter
  }
  # This property is not public
  diameter {
    2 * @radius
  }
}
```

Notice that the `diameter` property is not declared as `pub`. `diameter` is
used in `circumference`, which is valid because `circumference` is still within
the `Circle` type. However, `diameter` cannot be used outside of the `Circle`
definition.

```kaki
c = Circle.new(3)
println("circumference = {}", c.circumference) # Works!
println("diameter = {}", c.diameter) # Error! diameter is not pub
```

## Static

There can also be static fields, methods, and properties, which belong to the
type rather than instances of the type. This means that a single value is
shared across multiple all instances of a type.

First, consider static fields. They behave like instance fields, but are
prefixed with `@@` instead of `@`. Like instance fields, static fields have a
value of `none` before they are assigned.

```kaki
# This type counts how many instances of it have been made
# (which is the same as the number of times `new` has been
# called).
type InstanceCounter {
  pub cons new() {
    if @@counter == none {
      @@counter = 0
    }
    @@counter = @@counter + 1
  }
  pub count { @@counter }
}

i1 = InstanceCounter.new()
println(i1.count) #=> 1
i2 = InstanceCounter.new()
println(i1.count) #=> 2
println(i2.count) #=> 2
```

Static methods and properties are similar to instance methods and properties,
though they can only access static fields, and other static methods and
properties.

```kaki
type Calculations {
  pub Self.pi {
    3.141_593
  }
  pub Self.circle_area(radius) {
    Self.pi * radius ** 2
  }
}
```

Static methods and properties are declared using the `Self` reference. These
static methods and properties can be used in the following way:

```kaki
println(Calculations.pi)
println(Calculations.circle_area(5))
```

Static methods can also be called in an instance context.

```kaki
type Circle {
  pub cons new(radius) {
    @radius = radius
  }

  # Create a static method that can compute the area of a
  # circle of any radius
  pub Self.area(radius) { 3.141_593 * radius ** 2 }

  # Use the generic area method for computing the area of
  # this specific circle
  pub radius { Self.area(@radius) }
}
```
