# propositional_logic_calculator

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/noahbclarkson/propositional_logic_calculator)
[![Rust Version](https://img.shields.io/badge/rust-2021-blue)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-yellow)](https://opensource.org/licenses/MIT)

A comprehensive Rust library for computing and analyzing propositional logic expressions.

## Overview

`propositional_logic_calculator` is an evolving Rust project aimed at providing a robust and efficient way to compute and analyze propositional logic statements. This library is perfect for students, educators, and researchers in the field of logic and computer science.

## Features

- **Expression Parsing**: Parses and handles various propositional logic expressions, including conjunction, disjunction, implication, negation, and variables.
- **Proof Generation**: Automatically generates proofs for given propositional logic statements.
- **Logical Rule Application**: Applies logical rules like Modus Ponens, Modus Tollens, Double Negation, and more.
- **Customizable Proof Strategies**: Offers flexibility to define custom proof strategies and iterations for complex expressions.
- **Display and Debugging**: Neatly formats and displays logic expressions and proofs for easy debugging and analysis.

## Under Construction

This project is actively under development. New features and improvements are continuously being added. Contributions, suggestions, and feedback are always welcome!

## Getting Started

To get started with `propositional_logic_calculator`, add the following to your Cargo.toml:

```toml
[dependencies]
propositional_logic_calculator = { git = "https://github.com/noahbclarkson/propositional_logic_calculator" }
```

## Examples

Below is a basic example of how to use the library:

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

The included main function lets users run the project directly with individual queries. For example:

```Enter the propositional logic statement: ``` ```A>B,B>C,A/C```
```
Generated proof for: (A -> B), (B -> C), A / C
1 (1) (A -> B) A
2 (2) (B -> C) A
3 (3) A A
1, 3 (4) B MPP 1, 3
1, 2, 3 (5) C MPP 2, 4
```

## Contributing

Contributions to the `propositional_logic_calculator` project are welcome. Please feel free to fork the repository, make your changes, and submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

> Note: The badges in this README are for illustrative purposes and do not represent the current status of the project.
