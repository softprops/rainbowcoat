# rainbowcoat [![Build Status](https://travis-ci.org/softprops/rainbowcoat.svg?branch=master)](https://travis-ci.org/softprops/rainbowcoat)

> Adds rainbows over writers (inspired by [lolcat](https://github.com/busyloop/lolcat))

```rust
extern crate rainbowcoat;
use std::io::Write;

fn main() {
  write!(
    &mut rainbowcoat::Colors::configure(
      io::stdout(), 2.0, 0.4, 0.0
    ),
    "              _
                 ( |
                   |
            __,--./|.--,__
          .`   \ \ / /    `.
        .`      \ | /       `.
       /   /     ^|^      \   \
      /   / |     |o     | \   \
     /===/  |     |      |  \===\
    /___/   |     |o     |   \___\
            |     |      |
            |     |o     |
            |     |      |
            |     |o     |
            |     |      |
            |     |o     |
            |_____/\_____|
"
  )
}
```

Doug Tangren (softprops) 2017