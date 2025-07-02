# N-Dimensional Point Library

A simple, generic N-dimensional point library in Rust. This library provides a `Point` struct that can be used to represent points in an N-dimensional space. It supports various mathematical operations like addition, subtraction, multiplication, and division for both point-point and point-scalar operations.

## Features

  - **Generic N-dimensional Point:** Create points of any dimension with any numeric type.
  - **Basic Point Operations:**
      - `new(p: Vec<T>)`: Creates a new point.
      - `dim() -> usize`: Returns the dimension of the point.
      - `dist() -> f64`: Calculates the Euclidean distance from the origin.
      - `apply(func: F) -> f64`: Applies a custom function to the point's data.
      - `data() -> &[T]`: Returns a slice of the point's data.
  - **Operator Overloading:** Intuitive arithmetic operations.
      - **Point-Point Operations:** `+`, `-`, `*`
      - **Scalar Operations:** `+`, `-`, `*`, `/`
  - **Ownership and Borrowing:** Operations are implemented for both owned types (`Point<T>`) and references (`&Point<T>`).

-----

## Getting Started

### Creating a Point

You can create a new `Point` from a `Vec<T>`, where `T` is a numeric type.

```rust
use crate::Point;

// Create a 3-dimensional point with integer coordinates
let p1 = Point::new(vec![1, 2, 3]);

// Create a 2-dimensional point with float coordinates
let p2 = Point::new(vec![1.5, 2.5]);
```

-----

## Usage

### Basic Methods

Here are some of the basic methods available for `Point` instances:

```rust
// Get the dimension of the point
println!("Dimension of p1: {}", p1.dim()); // Output: 3

// Calculate the distance from the origin
println!("Distance of p1 from origin: {}", p1.dist()); // Output: 3.7416573867739413

// Get a slice of the point's data
let data = p1.data();
println!("Data of p1: {:?}", data); // Output: [1, 2, 3]

// Apply a custom function to the point's data
let sum = p1.apply(|coords| coords.iter().sum::<i32>() as f64);
println!("Sum of p1's coordinates: {}", sum); // Output: 6.0
```

-----

### Point Arithmetic

The library supports arithmetic operations between points of the same dimension.

```rust
let p1 = Point::new(vec![1, 2, 3]);
let p2 = Point::new(vec![4, 5, 6]);

// Addition
let p_add = &p1 + &p2;
println!("p1 + p2 = {:?}", p_add.data()); // Output: [5, 7, 9]

// Subtraction
let p_sub = &p1 - &p2;
println!("p1 - p2 = {:?}", p_sub.data()); // Output: [-3, -3, -3]

// Multiplication (element-wise)
let p_mul = &p1 * &p2;
println!("p1 * p2 = {:?}", p_mul.data()); // Output: [4, 10, 18]
```

-----

### Scalar Arithmetic

You can also perform arithmetic operations between a `Point` and a scalar value.

```rust
let p = Point::new(vec![1, 2, 3]);

// Scalar Addition
let p_add_scalar = &p + 10;
println!("p + 10 = {:?}", p_add_scalar.data()); // Output: [11, 12, 13]

// Scalar Subtraction
let p_sub_scalar = &p - 5;
println!("p - 5 = {:?}", p_sub_scalar.data()); // Output: [-4, -3, -2]

// Scalar Multiplication
let p_mul_scalar = &p * 2;
println!("p * 2 = {:?}", p_mul_scalar.data()); // Output: [2, 4, 6]

// Scalar Division
let p_div_scalar = &Point::new(vec![10, 20, 30]) / 10;
println!("[10, 20, 30] / 10 = {:?}", p_div_scalar.data()); // Output: [1, 2, 3]
```

-----

## Testing

To run the tests and verify that everything is working correctly, you can use the following command:

```bash
cargo test
```
