# Types

The most basic type is declared like:

```kaki
type A {}
```

but it is not very useful since it is unable to do anything. Let's see how extra
functionality can be added to it.

## Constructors

Types are created to store data and operations on them. A _constructor_ is used
to build an _instance_ of a type. In this example, a type which stores a vector
of three values will be created and built upon.

```kaki
type Vec3 {
  pub cons new(x, y, z) {
    @x = x
    @y = y
    @z = z
  }
}

# Create an instance of the type using the constructor.
Vec3.new(1, 2, 3)
```

A constructor can take any name. By convention, a constructor which creates an
instance of the type from its constituent parts should have the name `new`,
while a constructor which creates the type from something similar should have
the name `from`, such as:

```kaki
type Vec3 {
  pub cons from(xs) {
    @x, @y, @z = xs
  }
}
```

There is no limit on the number of constructors which can be used. To create a
vector of all zeros, the following constructor could be used:

```kaki
type Vec3 {
  pub cons zeros() {
    @x = 0
    @y = 0
    @z = 0
  }
}
```

## Fields

An instance is created using a constructor, and the instance data, called
_fields_, can be set up in a constructor (as shown above). Fields hold private
data which can only be accessed within the type itself, and the field names are
prefixed with `@`. A field always takes the value of `none` before it assigned.

In the case of `Vec3`, it has 3 fields: `@x`, `@y`, and `@z`.

## Methods

_Methods_ can be added to a type, which are functions that are bound to it.

```kaki
type Vec3 {
  pub self.as_list() {
    [@x, @y, @z]
  }
}

v = Vec3.new(1, 2, 3)
v.as_list() #=> [1, 2, 3]
```

Notice the inclusion of the new `self` name? This is a reference to the type
instance. It tells the type that this is an instance method, instead of of a
_static_ method. If the `self` is left out of the signature, the method is
implicitly an instance method. Including it is somewhat of a formality in this
example, though the reason for its presence use will become apparent in the
discussion of _static methods_. The same is true for _properties_, which are
discussed in the next section. Knowing this, the above definition can be
written as:

```kaki
type Vec3 {
  pub as_list() {
    [@x, @y, @z]
  }
}
```

Methods can accept all of the same argument types that functions do, and
additionally, they implicitly define a value named `self`, which is a reference
to the instance of the type. This is useful for calling other methods.

```kaki
type Vec3 {
  # Get the length of the vector
  pub length() {
    (@x ** 2 + @y ** 2 + @z ** 2) ** 0.5
  }

  # Scale a vector by a constant value. The direction
  # is preserved but the length is changed.
  pub scale(c) {
    Vec3.new(c * @x, c * @y,  c * @z)
  }

  # Compute a unit vector, which is a vector with the
  # same direction, but length of 1
  pub unit() {
    self.scale(1 / self.length())
  }
}
```

## Properties

Consider the previous example, this is a useful time to use _properties_.
Properties are like methods, but they are used for accessing data. They do not
accept any arguments, and are used to access data in a way that looks transparent
- there may be no backing data, it can be computed on the fly!

For `Vec3`, rather than the `length()` method, let's put it in a property
called `len`.

```kaki
type Vec3 {
  pub len() {
    (@x ** 2 + @y ** 2 + @z ** 2) ** 0.5
  }
}

v = Vec3.new(4, -1, 8)
v.len #=> 3
```

Here, a property was used to compute the length of the vector. Properties can
also be used to expose data in fields.

```kaki
type Vec3 {
  pub x { @x }
  pub y { @y }
  pub z { @z }
}

v = Vec3.new(4, -1, 8)
v.x #=> 4
v.y #=> -1
v.z #=> 8
```

These properties can be used to access data, but they cannot be used to change
it. In order to change the data, setter proerties can be created.


Properties also allow modifying data.

```kaki
type Vec3 {
  pub x = value {
    @x = value
  }

  pub y = value {
    @y = value
  }

  pub z = value {
    @z = value
  }
}

v = Vec3.new(4, -1, 8)

# Use the setter properties
v.y = 5
v.z = 6

v.x #=> 4
v.y #=> 5
v.z #=> 6
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
type Vec3 {
  # This is private
  is_zero? {
    return @x == 0 && @y == 0 && @z == 0
  }

  # This is public
  pub unit() {
    if self.is_zero? {
      Vec3.new(0, 0, 0)
    } else {
      self.scale(1 / self.length())
    }
  }
}
```

Here, the `is_zero?` property is private because we do not wish to expose it as
part of a public API. It is used as a check to fix our previous implementation
for `unit()`, which encounters a divide by zero when the length of the vector
is 0.

If we try to use `is_zero?` outside of the type, it causes a runtime error.

```kaki
v = Vec3.new(1, 2, 3)
v.is_zero? # Oops! is_zero? cannot be used in this context
```

## Static

There can also be static fields, methods, and properties, which belong to the
type rather than instances of the type. This means that a single value is
shared across multiple all instances of a type.

First, consider static fields. They behave like instance fields, but are
prefixed with `@@` instead of `@`. Like instance fields, static fields have a
value of `none` before they are assigned.

For a moment we will abandon our example of `Vec3` and consider some other type
implementations.

```kaki
# This type counts how many instances of it have been made
# (which is the same as the number of times `new` has been
# called).
type InstanceCounter {
  pub cons new() {
    if @@counter == none {
      @@counter = 0
    }
    @@counter += 1
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

  # Create an instance method for computing the area of
  # the specific circle in the instance described by the
  # instance
  pub area { Self.area(@radius) }
}
```
