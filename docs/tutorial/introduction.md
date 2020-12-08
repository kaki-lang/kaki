# Introduction

Before going any further, there are a couple of important rules for the way
that Kaki source code should be presented:

1. Any source code must be UTF-8 encoded.
2. Any souce file should have the `.kaki` extension.

With that in mind, let's see what Kaki code looks like.

```kaki
# Create a new function
fn add(x, y) {
  # Implicit return
  x + y
}

# Constant declaration. Numbers can use
# underscores for readability
PI = 3.141_592_654

# Higher-order functions. This returns a new
# list with all items squared
squares = [1, 2, 3].map { |x| x ** 2 }

# UTF-8 encoding allows for special characters
println("Hello 👋")

# Types are similar to classes in
# object-oriented languages
type Point {
  pub cons new(x, y) {
    @x = x
    @y = y
  }
}
```

We can see that Kaki is functionally inspired, and for good reason, as the
intended design of Kaki is functional-first.
