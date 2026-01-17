# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v0.12.1] - 2026-01-17

### Fixed
- Fixed an issue from last version's changelog

## [v0.12.0] - 2026-01-16

### Added
- Four new functions for handling input
- - `input_int` for getting integer corresponding to the pressed key on a keyboard according to the ASCII table
- - `intput_char` - for getting a single character
- - `poll_int` - same as `input_int`, but without waiting for input and getting `0` if no value is available in the moment
- - `poll_char` - same as `poll_char`, but for a char and getting `\0` as default

### Fixed
- Fixed an issue where variable didn't change after assigning a value to an already defined variable

### Known issues
- For now to variables can only be assigned integer literals: not functions that return integer, not other variables 
holding integers - nothing. This will be fixed in `v0.12.1`

## [v0.11.0] - 2026-01-15

### Added
- `put` function was temporarily added and then will be changed as an argument for `print` function
- Variables added

### Fixed
- Using integers
- Printing non-strings

## [v0.10.1] - 2026-01-09

### Changed
- Updated version and README for `cargo`
- Fixed build action (hopefully)

## [v0.10.0] - 2026-01-09
**This is a whole rewrite of a project in rust because last codebase was a huge mess. Mostly because `python` and my crooked arms**

With it codebase quality will (hopefully) rise dramatically, as well as some performance improvements. 
Also see the [devlog](docs/devlog/DEVLOG-0005-about-rewrite-in-rust.md) for more "unofficial" explanation

### Changed
- Comments changed from C-style: `//` to Python-style: `#`

### Removed
- Preprocessor isn't present and won't be present any soon. So now `#loop i 0 5` and similar will be treated as comments

### Known issues
- Passing integer to a `print` function will output corresponding char. This is not intended behavior and will be fixed in the update
- You can't use even escaped double quotes in string literals. Not sure if it's a top priority for the next update (because it requires partial rewrite of the parsing system), but it will be fixed eventually
- `print` temporarily function now only takes a single argument
- `put` function was temporarily removed. It will be back soon in one way or another

## [v0.4.1] - 2025-12-06

### Change
- Fixed the GitHub workflow

## [v0.4.0] - 2025-12-06

### Added
- Added debug mode into NJ interpreter now triggered with `--debug` argument
- Extended NJ specification and interpreter with special cell that can be accessed from anywhere on the tape and instructions to handle it (see [devlog](docs/devlog/DEVLOG-0004-recent-advencements.md))
- Added preprocessing step to the pipeline, ability to use macroses and first "loop" macros
- Changed the way IL generates from `print` statements, making it more optimized

### Change
- Renamed `brainfCustomInterpreter` folder into a `NJInterpreter`; Also changed the translator's name to `hive` to remove ambiguity with `bee` language
- Moved all the code into `src` folder and setup proper python package system
- Moved `main.py` to `__main__.py`. Now command to run the interpreter is `python -m src.hive <filename> [--debug]`

### Fixed
- Now `\n` in strings works properly
- Fixed an issue when `PRINT_ALL` didn't correctly handle multiple values on the stack

### Deprecated
- Visualizations are no longer supported and updated due the lack of quality

## [v0.3.0] - 2025-10-15

### Added
- Added two python scripts for visualizing translation process and NJ's execution (second one is still quite buggy tho)

### Changed
- Put all the code into `src` function and made it possible to work as with python's modules

## [v0.2.1] - 2025-10-05

### Changed
- Made Token model more stable code-wise by removing nested lists and using args

### Fixed
- Now `print` and `put` functions properly handle several arguments

## [v0.2.0] - 2025-09-30

### Added
- `put` function that works just like `print`, but without newline
- Support for string

### Fixed
- Fixed incorrect tokenizing when using commas in strings
- Fixed incorrect interpreting when having space as a value in LOAD_IMMEDIATE (when you had strings with spaces in it)

## [v0.1.1] - 2025-09-30

### Fixed
- Fixed an issue where you could set value out of 0-255 range and loaded value would overflow

## [v0.1.0] - 2025-09-30

### Added
- **First Alpha Release**: Basic compiler pipeline is now functional!
- Complete compilation flow: BEE → Intermediate Language → NJ → Brainfuck
- Tokenizer and parser for BEE syntax
- Intermediate Language (IL) representation
- Custom Brainfuck interpreter/runner
- IL to NJ language translation
- `print` function with automatic newline
- Support for integers (0-255 range)
- CLI with `--debug` flag for development
- Initial project structure and documentation

## [v0.0.1] - 2025-09-20

### Added
- Initial commit for the new implementation of HIVE compiler. For more info check corresponding 
[devlog](docs/devlog/DEVLOG-0001-rewrite.md)
