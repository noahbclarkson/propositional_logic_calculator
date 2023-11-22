
# propositional_logic_calculator

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/noahbclarkson/propositional_logic_calculator)
[![Rust Version](https://img.shields.io/badge/rust-2021-blue)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-yellow)](https://opensource.org/licenses/MIT)

## Overview

`propositional_logic_calculator` is a comprehensive Rust library designed for the computation and analysis of propositional logic expressions. It's an ideal tool for students, educators, and researchers in logic and computer science.

## Features

- **Expression Parsing**: Efficient parsing of various propositional logic expressions.
- **Proof Generation**: Automatic generation of proofs for given statements.
- **Logical Rule Application**: Implementation of logical rules like Modus Ponens, Modus Tollens, etc.
- **Customizable Proof Strategies**: Flexible definition of proof strategies for complex expressions.
- **Display and Debugging**: Clear formatting of logic expressions and proofs.

## Installation

To use `propositional_logic_calculator` in your project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
propositional_logic_calculator = { git = "https://github.com/noahbclarkson/propositional_logic_calculator" }
```

## Usage

Here's a basic example to get started with the library:

```rust
use propositional_logic_calculator::proof::Proof;

fn main() {
    let assumptions = vec!["A".to_string(), "B".to_string()];
    let conclusion = "A & B".to_string();
    let proof = Proof::new(assumptions, conclusion, None);
    proof.run();
    println!("Generated Proof: {}", proof);
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

For support, questions, or feedback, please open an issue in the [GitHub issue tracker](https://github.com/noahbclarkson/propositional_logic_calculator/issues).

---

> Note: The badges in this README are for illustrative purposes and do not represent the current status of the project.
