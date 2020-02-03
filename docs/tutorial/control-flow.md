# Control Flow

Control mechanisms allow programs to make decisions about their execution.

## Return

Consider a function, `double`, that doubles its argument, is created below.

```kaki
func double(x) {
  return 2 * x
}
```

The `return` expression tells the function to give the value of `2 * x` as the
value of the function. In reality, the function is just a block that can return
at any point in its evaluation.

Functions (and every other block-like structure) by default take on the value
of the last expression they execute. The reason for this is that the `return`
expression is not always required, and is only needed to exit a function early.
For example, the following is equivalent:

```kaki
func double(x) {
  2 * x
}
```

A function with more expressions can leverage an implicit return as well.

```kaki
func g(x, y) {
  a = x ** 2 + y ** 2
  b = x ** 2 - y ** 2
  a / b
}
```

In the above, the result of `a / b` is returned.

## Block Evaluation

The implicit return allows for extracting values from the evaluation of entire
block. Consider the following code:

```kaki
dog = {
  name = "Max"
  color = "brown"
  "Your new dog, {}, is {}.".fmt(name, color)
}
```

The block (everything between the `{` and `}`) is evaluated in the scope that
it appears, and the value of the last executed expression is returned.

Blocks are closures under the scope they are declared in, which means that they
have access to all of the names in the scope of their declarations, and they
can modify them.

```kaki
x = 3
y = {
  x = x + 7
  x * 2
}
println(x) #=> 10
println(y) #=> 20
```

## If and Else

The `if` keyword can be used to make a decision based on a decision:

```kaki
if x > 0 {
  println("positive")
}
```

It can be combined with `else` to execute a different block of code when the
condition fails:

```kaki
if x > 0 {
  println("positive")
} else {
  println("not positive")
}
```

These can be chained too, and only the the first block whose conditioned is
satisfied is executed.

```kaki
if x == 0 {
  println("zero")
} else if x > 0 {
  println("positive")
} else {
  println("negative")
}
```

Conditionals also return the value of the last line they execute, so the above
can be rewritten with only a single print function.

```kaki
println(if x == 0 {
  "zero"
} else if x > 0 {
  "positive"
} else {
  "negative"
})
```

When used in this style, if none of the bracnhes are evaluated then `none` is
returned.

## Loop

The `loop` runs forever. It repeatedly runs its body.

```kaki
loop {
  println("I will never stop!")
}
```

This will print `I will never stop!` over and over and over and over forever.

However, the `loop` can be stopped! When a `break` is encountered in a loop, it
tells the loop to exit and the program will continue after the loop.

```kaki
loop {
  println("I only run once")
  break
}
```

This loop now only runs once - until the `break` is hit. As soon as the the
break is encounterd the loop exits immediately.

When a `continue` is encountered, the loop immediately starts the next
iteration.

```kaki
loop {
  # Print random numbers which are odd
  r = Random.int()
  if r % 2 == 0 {
    continue
  }
  println(r)
}
```

Whenever an even number is generated, the loop hits the `continue` and the loop
starts again on the next iteration.

## For Loop

The `for` loop is used to iterate over the items in a sequence.

```kaki
for x in [1, 2, 3, 4] {
  println(x)
}
```

The `for` loop has the same rules for `break` and `continue` as the `loop` does.
When `continue` is encountered, the loop starts at the top but with the next
item in the sequence.

?>  The for loop iterates over a collection using the `each` method of the
    `Sequence` trait. Any type that implements `Sequence` can be used in a for
    loop.

## While Loop

The `while` loop is used to iterate until a condition is satisfied. Below is the
Babylonian method for computing a square root to three decimals of precision.

```kaki
n = 10
root = n / 2
while Math::abs(root ** 2 - n) > 0.001 {
  root = (root + n / root) / 2
}
println("sqrt({}) = {}", n, root)
```

The `while` loop has the same rules for `break` and `continue` as the `loop`
does.

## Loop Return Values

Like everything else, loops return values! They always return the value of the
last evaluated expression. If the loops doesn't evaluate any expressions, then
`none` is returned.

```kaki
# Get the last value in the sequence
v = for x in [1, 2, 3] {
  x
}
println(v) #=> 3
```

The `break` and `continue` expressions can also optionally take values that are
returned from the loop.

```kaki
# Find the first multiple of 5 in a sequence.
# If there isn't one, return none.
v = for x in [3, 6, 9, 15, 19] {
  if x % 5 == 0 {
    break x
  }
  # Make sure that this is last evaluated statement
  # in case the above condition is never true
  none
}
println(v) #=> 15
```
