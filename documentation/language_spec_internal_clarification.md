# Caraway Syntax
## Internal syntax
### Variables and functions

| operation | operator | example |
| - | - | - |
| variable declaration | = | x = 5 |
| assignment | -> | x -> 5 |
| function call | () | example_function(param1, param2, ...) |
| namespace access | :: | import std::math |

### Math

| operation | operator | example |
| - | - | - |
| comparison | =, <, <=, >, >= | if x = 5 |
| four function arithmetic | +, -, *, / | x = 6 + 7 * 9 |
| exponentiation | ^ | 5 ^ (1/2) |
| absolute value | &#124;x&#124; | &#124;-5&#124; = 5 |

### Macros

| name | use | example |
| - | - | - |
| py | execute arbitrary python at compile time | `@py { list(range(10))[5] }` |
| rust | execute arbitrary rust at compile time | `@rust {use path::to::rust; rust::run_the_function()}`

## Desmos operators

The following desmos operators have wrapped versions in the standard library, that use default parameters.

- derivative
- integral
- sum
- product 
- log 
- root
