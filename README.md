# The Caraway Programming Language for Desmos

> "Life is much more successfully looked at from a single window"
> Nick Caraway as Narrator, The Great Gastbzy by Scott F. Fitzgerald

## Overview

**Caraway** is a consise and expressive language designed to evaluate to a Desmos graphing calculator state.

Caraway offers clean syntax with modules, namespaces, and compiletime macros for all the functionality not provided out of the box.

Using Caraway is a tradeoff vs. the simplicity of Desmos. You should not use Caraway without taking advantage of the powerful features it provides that aren't already available with Desmos.

You should use Caraway if

- You want to process large amounts of data with a python script, etc, and load it into a Desmos graph.
- You want to programatically generate large numbers of Desmos expressions.
- You have a complex project that could benefit from namespaces and/or more control of your data.
- You want to be able to use other tools that aren't integrated into Desmos.


## Table of Contents

- [Getting Started](#Getting Started)
- [Language Overview](#Language Overview)
- [Implemented Features (Pre-Alpha)](#Implemented Features)
- [Detailed Language Documentation](./documentation/language_spec.md)

## Getting Started

The Caraway Compiler (`cwc`) is written in Rust.

### Building the compiler

to build the compiler from source, ensure `cargo` and the Rust Toolchain are installed (minimum version **Rust 1.85.1**). Run the following:
```bash
# go to compiler directory
cd ./cwc

# build compiler in release mode 
cargo build --release

# the binary will be at
./target/release/cwc --help
```

Optional but reccomended - symlink cwc to your .local/bin directory.
```bash
# create a symbolic link to the compiler at ~/.local/bin/cwc
ln -s ./target/release/cwc ~/.local/bin/
```

To test the compiler's functionality, try running it in the project directory for any of the example projects located in `./examples`

## Language Overview
Caraway's syntax is designed to be readable for those with some background in programming and knowlege of Desmos. Caraway adheres to some of the quirks of Desmos to avoid using high-overhead workarounds.

### Basics
Variables and functions are assigned exactly once per identifier.
```caraway 
initial_pos = 10
distance(x) = |x - initial_pos|

-- blocks evaluate to the final expression 
g(x) = {
    m = x^(1/2)
    y = mx
    y + x
}

-- not allowed!!! distance is already assigned
distance(y) = |y - g(y)|
```

Caraway integrates natively with Desmos.
```caraway
"quoted strings get exported as native comments in desmos"

temp_pi = pi -- certain identifiers are pre-claimed such as pi, which will be replaced by '\pi'
```

## Implemented Features


