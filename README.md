# r-ressources

[![CI](https://github.com/Glubus/r-ressources/workflows/CI/badge.svg)](https://github.com/Glubus/r-ressources/actions)
[![Crates.io](https://img.shields.io/crates/v/r-ressources.svg)](https://crates.io/crates/r-ressources)
[![Documentation](https://docs.rs/r-ressources/badge.svg)](https://docs.rs/r-ressources)
[![License](https://img.shields.io/crates/l/r-ressources.svg)](https://github.com/Glubus/r-ressources#license)

A Rust library inspired by Android/Kotlin's `R` system for managing resources at build time.

## Features

- **Build Time**: Resources are compiled directly into your binary
- **Type-safe**: Strongly typed constants
- **Zero-cost**: No runtime overhead
- **Thread-safe**: All resources are `const` - safe to use in multi-threaded contexts
- **Async-safe**: Works perfectly with tokio, async-std, and other async runtimes
- **Simple**: Clear and elegant syntax

## Supported Types

- `string`: String values
- `int`: Integer values (i64)
- `float`: Floating-point values (f64)
- `string-array`: String arrays
- `int-array`: Integer arrays
- `float-array`: Float arrays

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
r-ressources = "0.1.0"
```

## Usage

### 1. Create your resources file

Create `res/values.xml` at the root of your project (just like Android!):

```xml
<?xml version="1.0" encoding="utf-8"?>
<resources>
    <string name="app_name">My App</string>
    <string name="welcome">Hello!</string>
    
    <int name="max_retries">3</int>
    <int name="timeout_ms">5000</int>
    
    <float name="pi">3.14159</float>
    <float name="tax_rate">0.20</float>
    
    <string-array name="languages">
        <item>en</item>
        <item>fr</item>
        <item>es</item>
    </string-array>
    
    <int-array name="fibonacci">
        <item>1</item>
        <item>1</item>
        <item>2</item>
    </int-array>
    
    <float-array name="prices">
        <item>9.99</item>
        <item>19.99</item>
    </float-array>
</resources>
```

### 2. Use your resources

Two access styles are available:

```rust
use r_ressources::*;

fn main() {
    // Option 1: Flat access (shorter, cleaner)
    println!("{}", r::APP_NAME);
    println!("{}", r::MAX_RETRIES);
    
    // Option 2: Type-organized (avoids naming conflicts)
    println!("{}", string::APP_NAME);
    println!("{}", int::MAX_RETRIES);
    
    // Both work for arrays too
    for lang in r::LANGUAGES {
        println!("Language: {}", lang);
    }
}
```

## Conventions

- Keys are automatically converted to `UPPER_SNAKE_CASE`
- Special characters are replaced with `_`
- Resources are organized by type in modules

## Complete Example

```rust
use r_ressources::*;

fn configure_app() {
    // Use the flat r:: syntax for cleaner code
    let config = AppConfig {
        name: r::APP_NAME,
        max_retries: r::MAX_RETRIES,
        timeout: r::TIMEOUT_MS,
        supported_langs: r::SUPPORTED_LANGS,
    };
    
    println!("Configured: {}", config.name);
}
```

## Access Patterns

### Flat Access (Recommended)

```rust
use r_ressources::r;

fn main() {
    println!("{}", r::APP_NAME);      // Shorter
    println!("{}", r::MAX_RETRIES);   // Cleaner
    println!("{}", r::VERSION);       // More convenient
}
```

### Type-Organized Access

```rust
use r_ressources::*;

fn main() {
    println!("{}", string::APP_NAME);  // Explicit type
    println!("{}", int::MAX_RETRIES);  // Avoids naming conflicts
    println!("{}", float::VERSION);    // Clear organization
}
```

Choose based on your preferences - both are equally performant!

## Thread Safety

All resources are `const` values, making them completely thread-safe with zero synchronization overhead:

```rust
use std::thread;
use r_ressources::r;

// Safe to access from multiple threads
let handles: Vec<_> = (0..10)
    .map(|_| {
        thread::spawn(|| {
            println!("App: {}", r::APP_NAME);
            println!("Max retries: {}", r::MAX_RETRIES);
        })
    })
    .collect();

for handle in handles {
    handle.join().unwrap();
}
```

Works seamlessly with async runtimes:

```rust
use tokio::task;
use r_ressources::*;

#[tokio::main]
async fn main() {
    let tasks: Vec<_> = (0..10)
        .map(|_| {
            task::spawn(async {
                println!("App: {}", string::APP_NAME);
            })
        })
        .collect();
    
    for task in tasks {
        task.await.unwrap();
    }
}
```

## Error System

The library exposes an `RError` type for error handling:

```rust
pub enum RError {
    ResourceNotFound { resource_type: String, key: String },
    InvalidResourceFile { path: String, reason: String },
    TypeMismatch { expected: String, found: String },
}
```

## How It Works

When you add `r-ressources` as a dependency:

1. Create a `res/values.xml` file in your project root
2. Add your resources (strings, ints, floats, arrays)
3. At compile time, the build script parses your XML
4. Type-safe Rust constants are generated automatically
5. Access them via `string::NAME`, `int::NAME`, etc.

Each project has its own resources - just like Android apps!

## Philosophy

Like Android's `R` but in Rust: simple, efficient, and compiled at build time!

All resources are:
- Compiled into your binary (no runtime parsing)
- Type-safe (compile-time errors for typos)
- Zero-cost (direct memory access)
- Thread-safe (immutable `const` values)

## Performance

- **Compilation**: Resources are parsed once at build time
- **Runtime**: Zero overhead - direct constant access
- **Memory**: Resources live in the binary's data segment
- **Concurrency**: No locks, no synchronization needed

## Development

### Building from source

```bash
git clone https://github.com/Glubus/r-ressources.git
cd r-ressources
cargo build
cargo test
```

### Running examples

```bash
cargo run --example basic_usage
```

### Code quality

```bash
cargo fmt       # Format code
cargo clippy    # Lint code
cargo test      # Run all tests
```

## Contributing

Contributions are welcome! Please see [ARCHITECTURE.md](ARCHITECTURE.md) for technical details about the codebase structure.

## Publishing to crates.io

To publish this library:

```bash
cargo login      # One-time setup with your token
cargo publish    # Publish to crates.io
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
