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
| absolute value | \|x\| | \|-5\| = 5 |

### Macros
| name | use | example |
| - | - | - |
| py | execute arbitrary python at compile time | @py { list(range(10))[5] } |

## Desmos operators
Most desmos functions can be written literally and used. sin(x) will use the desmos sine function, unique([1,2,3]) will use the desmos unique function, etc.
Below are the desmos functions that are not just the name of the function
| Desmos function | operator | example |
| - | - | - |
| derivative (d/dx) | deriv[variable] | deriv[x] x^2 = 2x |
| integral | integral(expression)[variable] | integral(2x)[x] = x^2 + C |
| sum | sum[variable, start, end] | sum[n, 1, infinity] 1/(n^2) = pi^2 / 6 |
| product | product[variable, start, end] | product[n, 1, infinity] (1 - x^2 / (4n^2 pi^2))
| log | log[base], base assumed to be 10 if not specified | log[4] 16 = 2 OR log 100 = 2 |
| root | sqrt[n](x), n assumed to be 2 if not specified | 2 ^ (1/3) = sqrt[3](2) OR 2 ^ (1/2) = sqrt(2) |

note: the operators defined with square brackers are defined via macro

