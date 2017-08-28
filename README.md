# rainbowcoat [![Build Status](https://travis-ci.org/softprops/rainbowcoat.svg?branch=master)](https://travis-ci.org/softprops/rainbowcoat)

> Adds rainbows over writers (inspired by [lolcat](https://github.com/busyloop/lolcat))

```rust
extern crate rainbowcoat;
fn main() {
  write!(
    &mut rainbowcoat::stdout(),
    "hello color. how do you do?"
  )
}
```

Doug Tangren (softprops) 2017