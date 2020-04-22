# Modules

Modules are imported by their path relative to the location of the script.

```kaki
# This imports a module in the path with the
# name `module.kaki`
use module

# This imports a module in the path with the
# name `path/to/module.kaki`
use path::to::module
```

These import modules, and their contents can be used through the whole
namespace name.

```kaki
use module
println(module::SOME_CONSTANT)
```

However, it is sometimes more convenient to import specific names into the
module namespace.

```kaki
use module::{SOME_CONSTANT, SomeType, some_fn}
# The namespace does not need to be fully qualified
println(SOME_CONSTANT)
# But it can be and everything still works
println(module::SOME_CONSTANT)
```

It might be even more convenient to import all of the names using the `*`, but
be careful or else you might crowd your namespace!

```kaki
use module::*
# The namespace does not need to be fully qualified
println(SOME_CONSTANT)
# But it can be and everything still works
println(module::SOME_CONSTANT)
```

## Renaming

Imports can also be renamed at the time of import using `>`.

```kaki
use module::{SOME_CONSTANT > C}

# This works
println(C)

# Panics because `SOME_CONSTANT` is not defined
println(SOME_CONSTANT)
```

## Scoped Imports

Imports can even exist within an arbitrary scope.

```kaki
if x != none {
  use module::{SOME_CONSTANT, some_fn}
  println(some_fn(x, SOME_CONSTANT))
}
# SOME_CONSTANT and some_fn do not exist anymore
```

## Visibility

When creating a module, names must be explicitly exported to be used outside of
it. Just like for types and traits, modules use `pub` to declare that a name is
visibile outside of the module definition file itself. Below is the contents of a file `my-module.kaki`.

```kaki
# my-module.kaki

# Import some things. `SomeType` and `some_function`
# are public as if they were declared in this module
use mod_a::CONSTANT
pub use mod_b::SomeType
use mod_c::{pub some_function, VALUE}

# This is private to the module
SCALE = 10

# Export something
pub fn scale(x) {
  x * SCALE
}
```

To use this in a script, `my-program.kaki`, only certain things can be
imported.

```kaki
# my-program.kaki

# This works because `scale` was declared publicly
use my_module::scale

# This will print 100
println(scale(10))
```
