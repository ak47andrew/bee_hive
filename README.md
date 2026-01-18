# BEE_HIVE

![Written in Rust](https://img.shields.io/static/v1?label=Written%20in&message=Rust&color=green&logo=rust&style=for-the-badge)
![MIT License](https://img.shields.io/badge/License-MIT-green.svg)
![Crates.io Version](https://img.shields.io/crates/v/bee_hive)
![Platform](https://img.shields.io/badge/platform-linux%20%7C%20windows-lightgrey)

HIVE (stands for `Hive handles Integration, Validation, & Execution`) is a compiler and a CLI that transforms programs 
from custom programming language called Bee (stands for `Brainf**k Extension Language`) into brainf\*\*k-like language 
called NJ (stands for `No Joke`) with future plans for compiling directly to brainf\*\*k.

### Why
Why? Why the hell not? Isn't it hype when you put down readable code and get a valid code that is just like *R2-D2 noises*?

Also, it's mostly for educational purposes (of myself), so I feel like it's valid and if someone can find a reason to use
it - I'd be impressed so just go for it if you need it. 

You can also check out [this devlog](docs/devlog/DEVLOG-0006-why.md) as well other ones

## Features

- Ability to write text to the console
- blocking and non-blocking user input
- variables for integers
- *That's it for now... 🥲*

## Installation

### Portable

You can download portable executables from the [Releases tab](https://github.com/ak47andrew/bee_hive/releases)

### Using Cargo

This CLI was uploaded to [crates.io](https://crates.io/crates/bee_hive). You can download it using this commend:
```shell
cargo install bee_hive
```

## Run Locally

Clone the project

```shell
git clone https://github.com/ak47andrew/bee_hive.git
```

Go to the project directory

```bash
cd hive
```

Build the app

```bash
cargo build --release
```

Use `cargo run`

```bash
cargo run --release
```

...or access executable directly
```
./target/release/bee_hive
```


## Usage/Examples

CLI's help command looks something like this: `hive <PATH> [--debug]`
- \<PATH> - path to the source code of your program, often ending at `*.bee`
- \[--debug] - flag that allows to show extra information during compilation process which can be useful for debugging.
Behaves pretty much like `--verbose/-v` flag at most CLIs

Resulting code is outputted to the stdout, so you might use output redirection to write it to file like this: 
`hive code.bee > output.nj`

Example program at BEE language (to output `x = 1`) looks something like this:
```
var x = 10;
put("x=");
print(x);
```

Resulting code will look like this:
```
>!<  // SET_POINTER { index: 1 }
++++++++++<  // LOAD_IMMEDIATE_INTEGER { value: 10 }
>!<  // SET_POINTER { index: 1 }
[-+!#]>!>>>>>>#![+-!]  // STORE_VARIABLE { cell: 6 }
>!<  // SET_POINTER { index: 1 }
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++<
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++<  // LOAD_IMMEDIATE_STRING { value: "=x" }
>!<<  // SET_POINTER { index: 2 }
[-+!#]>!>#![+-!].[-]  // OUTPUT { value_type: Char }
>!<  // SET_POINTER { index: 1 }
[-+!#]>!>#![+-!].[-]  // OUTPUT { value_type: Char }
>!>>>>>>  // SET_POINTER { index: 6 }
[-+!#]>!<#![+-!]  // LOAD_VARIABLE { cell: 1 }
>!<  // SET_POINTER { index: 1 }
[-+!#]>!>#![+-!]>>+<<.[-]>>-  // OUTPUT { value_type: Integer }
>!<  // SET_POINTER { index: 1 }
++++++++++<  // LOAD_IMMEDIATE_STRING { value: "\n" }
>!<  // SET_POINTER { index: 1 }
[-+!#]>!>#![+-!].[-]  // OUTPUT { value_type: Char }
```

Info about BEE programming language and NJ target can be found in [GitHub wiki](https://github.com/ak47andrew/bee_hive/wiki)

## Known Issues

Check [Changelog](CHANGELOG.md) and [GitHub issues](https://github.com/ak47andrew/bee_hive/issues)

## Roadmap

- IR optimization
- Some math
- Ability to write and run code at Arduino and push code directly from this CLI
- If-else statements
- For and while loops
- Ability for multi-cell variables and immediates
- *1.0 Release! Hopefully...*

## Contributing

Contributions are always welcome!

Feels free to suggest your [pull requests](https://github.com/ak47andrew/bee_hive/pulls) with any bug fixes and ideas
> Also feel free to write your own devlogs! I created a `Developer` field specifically for it

If you encounter any errors, please report them [on GitHub](https://github.com/ak47andrew/bee_hive/issues)

[Discussions](https://github.com/ak47andrew/bee_hive/discussions) available for ideas, brainstorming (pun intended), use-cases
"in the wild" and your creations

## License

[MIT](LICENSE)
