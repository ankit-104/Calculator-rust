# Calculator in Rust

A modular mathematical expression tokenizer, parser, and evaluator written in Rust.

## Features

- **Basic Arithmetic**: Addition (`+`), Subtraction (`-`), Multiplication (`*`), Division (`/`), Modulo (`%` or `mod`), Exponentiation (`^`).
- **Unary & Postfix Operators**: Unary plus/minus (`+`, `-`), Factorial (`!`).
- **Mathematical Functions**:
  - Trigonometry: `sin`, `cos`, `tan` (evaluates angles in degrees)
  - Logarithms & Exponential: `log` (base-10), `ln` (natural log), `exp` ($e^x$)
  - Utilities: `sqrt` (square root), `abs` (absolute value)
- **Constants**: `pi` ($\pi$), `e` ($e$)
- **Parentheses**: Grouping expressions with `(` and `)`.

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024 or later)

### Running the CLI

Run the interactive calculator prompt via `cargo`:

```bash
cargo run
```

When prompted, type a mathematical expression:

```text
Enter the expression you want to calculate :
2 * (3 + sqrt(16)) - sin(30)
Result : 13.5
```

---

## Project Structure

```text
src/
├── lib.rs                   # Library root
├── main.rs                  # CLI interactive entry point
└── calculator.rs            # Calculator wrapper & module declarations
    ├── token.rs             # Tokenizer / Lexer
    ├── parser.rs            # Recursive-descent AST Parser
    ├── operations.rs        # Arithmetic helper operations (division, modulo)
    └── function.rs          # Mathematical functions & constants implementation
```

---

## Module Overview

- **`token.rs`**: Converts raw input strings into a sequence of tokens (`number`, `plus`, `minus`, `multiply`, `devide`, `leftParen`, `rightParen`, `factorial`, `power`, `identifier`).
- **`parser.rs`**: Parses tokens using precedence rules (Add/Sub $\rightarrow$ Mul/Div $\rightarrow$ Power $\rightarrow$ Unary $\rightarrow$ Postfix $\rightarrow$ Primary/Function).
- **`function.rs`**: Implements functions (`sin`, `cos`, `tan`, `sqrt`, `abs`, `exp`, `log`, `ln`, `factorial`).
- **`operations.rs`**: Handles edge cases like division or modulo by zero.

---

## License

This project is open-source under the MIT license.

