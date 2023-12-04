These are my solutions to the 2023 Advent Of Code challenge. I'm super-new to Rust, so this is probably full of terrible unidiomatic code! Comments or advice welcome!

## Questions

Confusions I had while implementing these that I'd like to follow-up on later to understand better

### How to make a vector-containing-a-single-string

My implementation for `solve_three` originally included:

```rust
let mut bracketed_lines: Vec<&str> = Vec::new();
let mut bracketing_line_vec = vec!(std::iter::repeat('.').take(length_of_line).collect::<String>().as_str());
bracketed_lines.append(&mut bracketing_line_vec);
bracketed_lines.append(&mut lines_as_vec);
bracketed_lines.append(&mut bracketing_line_vec);
```

For which the second line had an error:

```
temporary value dropped while borrowed
creates a temporary value which is freed while still in use
```

The solution involved extracting `std::iter::...<String>()` to a separate variable - however, I don't know why this
was different. Why would that value be dropped within `vec!`?

### How to reuse a vector?

[Vec::append](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.append) leaves the `other` empty, hence the need to `clone()` in `process_three()` when "wrapping" the lines in symbol-free lines. How should one append the same vector to another vector twice?

### How to find the size of the intersection of two sets?

See implementation of `four.rs` - `.try_len()` gives inaccurate answer.