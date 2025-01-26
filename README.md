# Rust Mutable and Immutable References Coexistence

This example demonstrates a potential pitfall in understanding mutable and immutable references in Rust.  The code compiles and runs without error, even though it might seem to violate Rust's borrowing rules at first glance.

The `bug.rs` file contains the potentially confusing code, while `bugSolution.rs` offers an explanation and clarification.

This repo serves as a learning tool for understanding how Rust manages references and avoids data races.