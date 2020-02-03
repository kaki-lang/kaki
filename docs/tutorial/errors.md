# Errors

There are two types of errors:

- Panics
- Recoverable errors

## Panic

Panics are an aggressive form of error reporting and indicate something went
very wrong. A panic will bubble up the stack until the program crashes.

```kaki
q = {
  if y == 0 {
    panic("Can't divide by 0")
  }
  x / y
}
```

A panic can be caught, but it should only be done if it can be handled
properly.

```kaki
Fiber.try {
  panic("Oh no!")
}.then { |r|
  # If the first block executes normally, this runs and can
  # optionally accept the result of the block.
  println("Success! Got: {}", r)
}.catch { |msg|
  # If the first block panics, this runs and can optionally
  # accept the panic message.
  println("Panic: {}", msg)
}
```

It is more useful to panic with an error type than a message, since the type
can be used to run different catch blocks.

```kaki
x = 10
x = 5
Fiber.try {
  if y == 0 {
    panic(DivideByZeroError())
  }
  z = x / y
  if z == 3 {
    panic(ValueError("Bad value"))
  }
}.catch(DivideByZeroError) {
  println("Don't divide by zero!")
}.catch(ValueError) {
  println("Don't have a quotient of 3!")
}
```

# Recoverable Errors

If an error is obviosly recoverable, then it should be placed in a `Result`
type.

```kaki
func divide(x, y) {
  if y == 0 {
    Result.err("Can't divide by 0")
  }
  Result.ok(x / y)
}
```

The result type is useful because it can easily be used with blocks to
manipulate it into another result. The `then` and `then_err` methods are run
depending on whether the result was `ok` or `err`, with the value or error
passed as the argument, respectively.

```kaki
divide(20, 4).then { |v| println(v) }.then_err { |e| println(e) }
```

There are also `map` and `map_err` methods which can be used to transform the
value into a new `Result`.

```kaki
divide(20, 4)
  .map { |v| Result.ok(v + 1) }
  .map_err { |_| Result.err("Everyone knows you can't divide by zero!") }
```
This strategy can also be used on the error.
