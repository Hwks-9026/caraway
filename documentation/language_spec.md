# Caraway Language Syntax Specification

**Note: until 1.0, this is very subject to change, and not all language features may actually be implemented by the compiler. Consult [README.md](../README.md) for a list of implemented language features.

Assign variables using the '=' assignment operator.
```caraway
x = 4
```

Functions can be assigned in the same way.
```caraway
f(x) = x^2
```

Functions and variables can both be assigned to the result of a block of code.
```caraway
g(x) = {
    m = x^(1/2)
    y = mx
    y + x
}
```
The value of will then be the result of the last expression, after evaluating each statement.

Single line code comments (not exported to desmos) can be made with a `--`.
```caraway
x = 3.1415926 -- approximation for pi
```

Exported comments to desmos can be done by simply enclosing text in single quotes on an otherwise empty line (excluding code comments).
```caraway
x = "not a valid exportable comment"
"valid exportable comment"
"also valid comment" -- this is still valid
```

Symbols supported by desmos will be auto-collapsed from the corresponding identifier.
If these symbols have a default constant value (like pi), they don't need to be defined in your code to be used.
```caraway
my_pi = pi -- sets my_pi to pi. my_pi will be replaced with symbol '\pi' in desmos.
mu = 0.7   -- binds symbol mu to 0.7. 
one = e^0  -- use symbol e (euler's number) without needing to define it
```


Caraway supports multi-file projects. Simply create multiple '.cara' files, then include them with the 'use' keyword.
Expose any public functions or values you need using namespace . To re-export code as a submodule of the current file,
simply use the same 'pub' keyword in front of the use keyword.
```
-- Current file name: 'currentfile.cara'

use matrix_math::*      -- Expose all public mebmers at ./matrix_math.cara
use draw_thingy::draw   -- Expose just the element 'draw' of file ./draw_thingy.cara

pub use cheeto::color   -- Expose cheeto color, and re-expose it so it can be accessed from another file with `use currentfile::color`
```

Caraway also supports several macros, prefixed with `@`. 

If you want to re-define a symbol with a constant value in your code, you can do so, and the compiler will postfix it.
You must use the `@redef` macro when re-defining symbols with constant value.
```caraway
@redef(pi) = 4 -- replaces pi with pi_0 when converting to desmos
x = pi         -- compiler knows pi has been re-defined to a literal, and will replace it with pi_0
```

Execute arbitrary python functions, and assign the results to variables. Include a file with `@use_python` and execute a function with `@py`.
***TODO: Explain how and what the return values need to be to be valid***
```caraway
@use_python("path/to/python/file.py") -- exposes all python functions as callable with literal arguments at compile time

image_list = @py(file::load_image_data("path/to/image.png")) -- calls python function load_image_data, contained in the loaded 'file.py'
```
A current constraint of the compiler is that python code cannot accept caraway variables, only literals like strings, numbers, etc. 




