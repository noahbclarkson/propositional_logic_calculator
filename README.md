
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

## Example

The propositional_logic_calculator project can be interactively used to compute proofs for propositional logic statements. When the project is run, it prompts the user to enter a propositional logic statement. Upon entering a valid statement, the program computes and displays a proof for the given statement.

Interactive Usage Example
When you run the project, it asks for a propositional logic statement in the format `Assumptions/Conclusion`. Here's an example of how this interaction works:

```bash
Enter the propositional logic statement:
Pv(Q>R),Q,P>W/W
```

```bash
Assumptions: [(P v (Q -> R)), Q, (P -> W), (R -> W)]
Conclusion: W
Total Proof Steps: 10
Proof Steps:
Line 1: (P v (Q -> R)) [1] using A
Line 2: Q [2] using A
Line 3: (P -> W) [3] using A
Line 4: (R -> W) [4] using A
    Line 5: P [1] using A(vE) from lines 1
    Line 6: W [1, 3] using MPP from lines 3, 5
    Line 7: (Q -> R) [1] using A(vE) from lines 1
    Line 8: R [1, 2] using MPP from lines 2, 7
    Line 9: W [1, 2, 4] using MPP from lines 4, 8
Line 10: W [1, 2, 3, 4] using vE from lines 5, 6, 7, 8, 9
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

For support, questions, or feedback, please open an issue in the [GitHub issue tracker](https://github.com/noahbclarkson/propositional_logic_calculator/issues).

---

> Note: The badges in this README are for illustrative purposes and do not represent the current status of the project.
