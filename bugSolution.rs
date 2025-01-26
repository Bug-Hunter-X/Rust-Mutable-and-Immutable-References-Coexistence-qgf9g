The code works correctly because the immutable reference `z` is created *after* the mutable reference `y` is created and *before* `y` is used to modify `x`. This is safe because the immutable reference `z` only reads the value of `x`, and no other mutable reference to `x` exists simultaneously.

If you tried to create `z` *after* modifying `x` via `y`, the compiler would correctly throw an error because you'd have both a mutable and immutable reference to `x` concurrently, which is a violation of Rust's ownership rules. 

The key takeaway is that the order of reference creation and modification is crucial. Mutable and immutable references to the same variable can safely coexist as long as the mutable reference isn't used while the immutable reference exists.