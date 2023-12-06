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

### How to reference a Vec field of a struct in impl function?

See `five.rs`. I originally had (paraphrased):

```
struct AlmanacMap {
    ranges: Vec<AlmanacMapRange>
}

impl AlmanacMap {
    pub fn map(&self, source: i32) -> i32 {
        return self.ranges.into_iter()
            ...
    }
}
```

but got `cannot move out of 'self.ranges' which is behind a shared reference`. I think I understand part of the problem (the `self.ranges` in the implementation would move that field, and because it's a field the borrow-checker can't guarantee there aren't other moves happening), but am not sure how to resolve it. This seems like a situation where I'd want to borrow, but some quick Googling around suggested that that's impossible.

### Why does `lines` behave differently when passed into a function?

The following is fine:

```rust
let mut lines = fs::read_to_string("myfilename.txt").unwrap().lines();
lines.next()
```

But this fails:

```rust
let mut lines = fs::read_to_string("myfilename.txt").unwrap().lines();
do_something_with_lines(lines);

fn do_something_with_lines(lines: Lines<&str>) {
    lines.next();
}
```

with the following error:

```
the method `next` exists for struct `std::io::Lines<&str>`, but its trait bounds were not satisfied
the following trait bounds were not satisfied:
`&str: BufRead`
which is required by `std::io::Lines<&str>: Iterator`
```

I tried changing the type signature to `do_something_with_lines(lines: Lines<dyn BufRead>)` and got:

```
the size for values of type `(dyn BufRead + 'static)` cannot be known at compilation time
the trait `Sized` is not implemented for `(dyn BufRead + 'static)`
```