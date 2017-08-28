extern crate rainbowcoat;
use std::io::{self, Write};

fn run() -> io::Result<()> {
    write!(
        &mut rainbowcoat::Cat::configure(io::stdout(), 2.0, 0.4, 0.0),
        r"                  _
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
    )?;
    Ok(())
}

fn main() {
    run().unwrap()
}
