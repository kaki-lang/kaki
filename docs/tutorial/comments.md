# Comments

A line comment starts with `#` and anything after `#` is ignored.

```kaki
# This is a comment
```

Comments can also occur on the same line as code.

```kaki
a = 5 # Here a variable is initialized
```

Documentation comments are a special kind of comment that are treated slightly
differently from regular comments. They provide documentation for the code, and
can be used to generate standalone documenation. Documentation comments are a
block where each line begins with `///`.

```kaki
/// Add two values together.
///
/// # Arguments
///
/// * `x` - The augend (first operand).
/// * `y` - The addend (second operand).
///
/// # Returns
///
/// A sum.
fn add(x, y) {
  x + y
}
```
