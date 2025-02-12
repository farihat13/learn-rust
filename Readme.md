# Learn Rust


**Usage**

Run main.rs
```sh
cargo run
```

Any file in `bin` folder
```sh
cargo run --bin guessing_game
```

Format project
```sh
cargo fmt
```

## [Install Rust](https://www.rust-lang.org/tools/install)

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## `rustc` (Rust Compiler)

`rustc` is responsible for compiling Rust source code into an executable binary.  
Use it to quickly compile a Rust program without package management or dependencies. However, `cargo` is preferred since it automates dependency management, builds, and testing.

```sh
# generates an executable `main`
rustc main.rs
```

## `cargo` (Rust Package Manager & Build System)

`cargo` is the package manager and build system for Rust. It simplifies project creation, building, testing, and dependency management. 

**Common Cargo Commands**

| Command                | Description                                       |
| ---------------------- | ------------------------------------------------- |
| `cargo new my_project` | Creates a new Rust project                        |
| `cargo build`          | Compiles the project (debug mode)                 |
| `cargo run`            | Compiles and runs the project                     |
| `cargo check`          | Checks for errors without building a binary       |
| `cargo test`           | Runs unit and integration tests                   |
| `cargo clean`          | Removes compiled files (`target/` directory)      |
| `cargo doc --open`     | Generates and opens documentation for the project |

**Usage** 
```sh
# creates a new project, compiles, and runs it
cargo new my_project
cd my_project
cargo run
```

### `rustup` (Rust Version & Toolchain Manager)

`rustup` is a tool for managing Rust versions and toolchains.
  - Manages Rust toolchains (`stable`, `beta`, `nightly`)
  - Allows installing different versions of Rust for different projects
  - Provides access to additional tools like `clippy` (linter) and `rustfmt` (formatter)

**Common `rustup` Commands**
| Command                       | Description                               |
| ----------------------------- | ----------------------------------------- |
| `rustup show`                 | Shows active Rust toolchain               |
| `rustup update`               | Updates Rust to the latest stable version |
| `rustup install nightly`      | Installs the nightly Rust version         |
| `rustup default stable`       | Sets stable as the default toolchain      |
| `rustup override set nightly` | Uses nightly Rust for the current project |
| `rustup component add clippy` | Installs the Clippy linter                |

**Usage** 
```sh
rustup install stable
rustup default nightly  # Switches to nightly Rust globally
rustup override set stable  # Uses stable Rust for the current directory
```