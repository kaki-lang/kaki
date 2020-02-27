# Multiline Expressions

Some languages require a `;` or other punctuation after every expression, but
that is not required here. Programs are one statement per line, but of course
multiple can be specified on a line by using `;` as a delimiter. For example:

```kaki
# This code
a = 3
b = 10
println(a + b)

# Can be written in one line as
a = 3; b = 10; println(a + b)
```

However, expressions can be much too long to fit on a single line, so there are
a set of rules for when an expression continues to the next line:

1.  Anything inside of `(...)` and `[...]` are automatically grouped.

    ```kaki
    # A list spanning multiple lines
    COLORS = [
      "red",
      "orange",
      "yellow",
      "green",
      "blue",
      "indigo",
      "violet"
    ]

    # Function arguments one per line
    println(
      "The rainbow has the colors {} through {}!",
      COLORS.first,
      COLORS.last)

    # This even works as part of a larger expression
    x = 2 * (
      1 + 2 + 3 + 4)
    ```

2.  When a line ends with an operator that cannot end an expression, the
    expression continues to the next line.

    ```kaki
    # This sets x to 15
    x = {
      1 + 2 + 3 +
      4 + 5
    }

    # This sets x to 9
    x = {
      1 + 2 + 3
      + 4 + 5
    }
    ```

3.  When a line starts with `\`, it is a continuation of the previous line.

    ```kaki
    # This sets x to 15
    x = {
      1 + 2 + 3
      \ + 4 + 5
    }
    ```

4.  When a line starts with `.`, it is a continuation of the previous line.

    ```kaki
    # Chaining methods
    [1, 2, 3].map { |x| x ** 2 }
      .fold(0) { |acc, x| acc + x }
    ```
