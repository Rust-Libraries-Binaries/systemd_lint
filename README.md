## Archived Repository

**This repository was a gift from me to the Rust and open-source community. It is no longer actively maintained and has been archived. Feel free to fork and continue development on your own.**

# systemd_lint

A tool to validate and analyze systemd configuration files (`.service`, `.socket`, etc.) for potential issues, misconfigurations, and optimizations.

## Features

- **Parse & Validate**: Analyze `.service` files for syntax and logical errors.
- **Linting**: Offer best practice recommendations and checks for common mistakes.
- **Optimization Suggestions**: Highlight areas for reducing dependencies and improving security settings.

## Installation

To use `systemd_lint`, you need to have Rust installed. If you don't have Rust, you can install it from [rustup.rs](https://rustup.rs).

Clone the repository and build the project:

```sh
git clone https://github.com/yourusername/systemd_lint.git
cd systemd_lint
cargo build
```
## Usage
To run systemd_lint on a systemd service file:
```sh
cargo run -- path/to/your/service/file.service
```
For Example:
```sh
cargo run -- example.service
```
This will parse the provided service file, apply linting checks, and suggest optimizations.

## Examples
Warnings:
- ExecStart points to /bin/false, which may be incorrect.
- Running as root is not recommended. Consider using a non-privileged user.

Suggestions:
- Consider removing 'After=network.target' if not needed.
- Add 'ProtectSystem=full' for improved security.

## License
This project is licensed under the MIT License

## Author
Ben Santora /

