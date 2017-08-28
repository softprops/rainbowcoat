# rainbowcoat [![Build Status](https://travis-ci.org/softprops/rainbowcoat.svg?branch=master)](https://travis-ci.org/softprops/rainbowcoat)

> Adds rainbows over writers (inspired by [lolcat](https://github.com/busyloop/lolcat))

```rust
extern crate rainbowcoat;
fn main() {
  write!(
    &mut rainbowcoat::stdout(),
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