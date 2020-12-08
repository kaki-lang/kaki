# Multiline Expressions

Some languages require a `;` or other punctuation after every expression, but
that is not required here. Programs are one statement per line, but of course
multiple can be specified on a line by using `;` as a delimiter. For example:

```kaki
# This code
a = 3
b = 10
println(a + b)

# can be written in one line as
a = 3; b = 10; println(a + b)
```

However, expressions can be much too long to fit on a single line, so there are
a set of rules for when an expression continues to the next line:

1.  Anything inside of `(...)` and `[...]` is automatically grouped.

    ```kaki
    # A list spanning multiple lines
    NUMBERS = [
      1,
      2,
      3
    ]

    # Function arguments spanning multiple
    println("The numbers span from {} to {}",
      NUMBERS.first, NUMBERS.last)

    # This even works as part of a larger expression.
    # x is 20 after evaluating this line.
    x = 2 * (1 + 2
      + 3 + 4)
    ```

2.  When a line starts with `.`, it is a continuation of the previous line.

    ```kaki
    # Chaining methods
    [1, 2, 3].map { |x| x ** 2 }
      .fold(0) { |acc, x| acc + x }
    ```

3.  When a line starts with `\`, it is a continuation of the previous line.

    ```kaki
    # This sets x to 15
    x = 1 + 2 + 3
      \ + 4 + 5
    ```
