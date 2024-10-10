# structify

structify is a Rust crate that provides a procedural macro to transform functions into structs with state and execution dependencies.

## Features

- **Struct Generation**: Automatically generate structs from functions using the `#[structify]` attribute.
- **Dependency Injection**: Manage execution dependencies seamlessly with the `Dep<T>` type.
- **Async Support**: Compatible with both synchronous and asynchronous functions.
- **Custom Struct Names**: Optionally specify custom names for generated structs.
- **State Management**: Handle state within structs to maintain clean and modular code.

## Installation

Add `structify` to your `Cargo.toml` dependencies:

```toml
[dependencies]
structify = "0.1.0"  # Replace with the actual version
```

Import the macro in your crate:

```rust
use structify::structify;
```

## Usage

### Basic Function Transformation

Annotate a function with `#[structify]` to generate a struct with `new()` and `execute()` methods:

```rust
#[structify]
fn greet() {
    println!("Hello, World!");
}

fn main() {
    Greet::new().execute();
}
```

### Functions with Arguments and Return Values

Functions with parameters will have those parameters as fields in the generated struct:

```rust
#[structify]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = Add::new(2, 3).execute();
    assert_eq!(result, 5);
}
```

### Using Dependencies with `Dep<T>`

Use `Dep<T>` to inject dependencies or state into your functions:

```rust
use structify::Dep;

#[structify]
fn increment(value: i32, state: Dep<i32>) -> i32 {
    value + *state.inner()
}

fn main() {
    let state = Dep::new(10);
    let result = Increment::new(5).execute(state);
    assert_eq!(result, 15);
}
```

### Mixing Arguments and Dependencies

You can mix regular arguments and dependencies:

```rust
#[structify]
fn compute(a: i32, state: Dep<i32>, b: i32) -> i32 {
    a + *state.inner() + b
}

fn main() {
    let state = Dep::new(20);
    let result = Compute::new(5, 10).execute(state);
    assert_eq!(result, 35);
}
```

### Custom Struct Names

Specify a custom name for the generated struct using attributes:

```rust
#[structify(CustomName)]
fn my_function() {
    // Function body
}

fn main() {
    CustomName::new().execute();
}
```

### Asynchronous Functions

Annotate async functions to generate structs that support asynchronous execution:

```rust
#[structify]
async fn fetch_data() -> String {
    // Simulate fetching data asynchronously
    "Data fetched".to_string()
}

#[tokio::main]
async fn main() {
    let result = FetchData::new().execute().await;
    println!("{}", result);
}
```

## How It Works

The `#[structify]` macro transforms the annotated function into a struct:

- **Struct Fields**: Parameters of the function become fields of the struct.
- **Constructor (`new()`)**: Initializes the struct with provided arguments.
- **Execution Method (`execute()`)**: Executes the original function, injecting any dependencies.

Dependencies are handled via the `Dep<T>` type, which uses `Arc<T>` for shared ownership and thread safety.

## Examples

### Stateful Computation with Dependencies

```rust
use structify::{structify, Dep};

#[structify]
fn multiply_and_add(x: i32, multiplier: Dep<i32>, adder: i32) -> i32 {
    x * *multiplier.inner() + adder
}

fn main() {
    let multiplier = Dep::new(3);
    let result = MultiplyAndAdd::new(5, 2).execute(multiplier);
    assert_eq!(result, 17); // (5 * 3) + 2
}
```

### Asynchronous Processing

```rust
use structify::structify;

#[structify]
async fn async_compute(a: i32, b: i32) -> i32 {
    a + b
}

#[tokio::main]
async fn main() {
    let result = AsyncCompute::new(10, 20).execute().await;
    assert_eq!(result, 30);
}
```

## Limitations

- **Lifetime Support**: Currently, the macro does not support functions with lifetime parameters or references.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests on [GitHub](https://github.com/yourusername/structify).

## License

This project is licensed under the [MIT License](LICENSE).
