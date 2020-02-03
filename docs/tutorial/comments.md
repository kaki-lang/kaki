# Comments

A line comment starts with `#` and anything after `#` is ignored.

```kaki
# This is a comment
```

Comments can also occur on the same line as code.

```kaki
a = 5 # Here a variable is initialized
```

Block comments are possible as well using `#[[` and `]]`.

```kaki
#[[ This is a block comment ]]

#[[
Block comments can
span multiple lines
]]
```

Block comments can even be nested, allowing for code to be conveniently
commented out without worry.

```kaki
#[[
This is some comment with
#[[ another comment ]]
inside of it
]]
```

Be careful, because a block comment cannot start on a line occupied by line
comment.

```kaki
# This does not work #[[
println("Hello")
]] # <- syntax error
```
