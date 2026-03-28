# Caraway Language Syntax Specification

**Note: until 1.0, this is very subject to change, and not all language features may actually be implemented by the compiler. Consult [README.md](../README.md) for a list of implemented language features.

Declare variables using the '=' assignment operator.
This can only be done once per identifier.
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

The following are a list of valid operators

| operation | operator | example |
| - | - | - |
| variable declaration | = | x = 5 |
| assignment | -> | x -> 5 |
| function call | () | example_function(param1, param2, ...) |
| namespace access | :: | import std::math |
| comparison | ==, <, <=, >, >= | if x == 5 |
| four function arithmetic | +, -, *, / | x = 6 + 7 * 9 |
| exponentiation | ^ | 5 ^ (1/2) |
| absolute value | &#124;x&#124; | &#124;-5&#124; == 5 |

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

Caraway supports multi-file projects. Simply create multiple '.cara' files, then import them with the 'import' keyword.
Expose any public functions or values you need using namespace . To re-export code as a submodule of the current file,
simply use the same `export` keyword in front of the use keyword that you'd use for promoting the namespace of any other value.
```caraway
-- Current file name: 'currentfile.cara'

import matrix_math::* -- Expose all public mebmers at ./matrix_math.cara into the default namespace for the file path, 'matrix_math'

x = matrix_math::zeros(1, 2) -- call a public member of matrix_math

import draw_thingy::draw as draw_thingy_draw -- Expose just the element 'draw' of file ./draw_thingy.cara, and bind that to the identifier draw.

y = draw_thingy_draw((1, 2)) -- draw the point with draw_thingy??draw

export use cheeto::color -- Expose cheeto color, and re-expose it so it can be accessed from another file with `use currentfile::color`
```

Caraway supports namespaces via files and the module keyword. 
To promote an object to the parent namespace (it retains its current namespace as well), use the `export` keyword
```caraway
-- Current file name: 'math.cara'

module addition {
    export add(a, b) = a+b
}

export identity(a) = a 

-- ====================================
-- Current file name: 'using_math.cara'

import math;

three = math::add(1, 2) -- this works because add was exported to the 'math' top level namespace

one = identity(1)       -- this works because identity was exported from the 'math' top level namespace, 
                        -- It was added to the 'using_math' namespace when imported.

four = math::addition::add(2, 2) -- Fully qualified namespaces still work, even after exporting
```


Caraway also supports several macros, prefixed with `@`. Following the identifier, a block encapsulated by `{}` contains the macro syntax.

If you want to re-define a symbol with a constant value in your code, you can do so, and the compiler will postfix it.
You must use the `@redef` macro when re-defining symbols with constant value.
```caraway
@redef{pi} = 4 -- replaces pi with pi_0 when converting to desmos
x = pi         -- compiler knows pi has been re-defined to a literal, and will replace it with pi_0
```

Execute arbitrary python functions, and assign the results to variables. Import a python file via the `@python` and execute a function with `@py`.
All variables and functions in a python file are considered fair game for use in the `@py` macro, but since they can't be 'exported' from python,
you must use a fully qualified path to the function or variable you need.

```caraway
import @python{"path/to/python/file.py"} as file -- exposes all python functions as callable with literal arguments at compile time

image_list = @py{file::load_image_data("path/to/image.png")} -- calls python function load_image_data, contained in the loaded 'file.py'

rows = @py{file::rows}
columns = @py{file::colums} -- loading variables from a python file also works
```




