# LoosenMap

Calls a function with `self`.
ie. `a.loose_map(b)` = `b(a)`.

Declares a trait/method similar to `std::iter::Iterator::map`,
where any type that is the sole argument for a function can be
called into such function. 

ps. I called this "being mapped into such function"
but I don't know if this is correct.

## Example with Loosen

This is an example of usage with the 
[loosen](https://crates.io/crates/loosen)
attr derive macro.

```rust
use loosen::loose;
use loosen_map::LooseMap;

struct A;
struct B;

#[loose] 
fn fa(a: A, b: B) -> bool { true }

// normal call
assert!(fa(A, B));

// loose call, available from the `loosen` crate
assert!(fa_loose((A, B)));

// loose_map with loose call
assert!((A, B).loose_map(fa_loose));
// ie. from the argument (or loosened arguments),
// you may call some function (or loosened function)
```

