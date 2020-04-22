# Anonymous Types and Traits

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
type : T1, T2, T3 {
  # ...
}

# 3 - anonymous trait
trait {
  # ...
}

# 4 - anonymous trait implementing traits
trait : T1, T2, T3 {
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
