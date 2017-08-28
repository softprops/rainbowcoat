
extern crate rainbowcoat;
use std::io::{self, Write};

fn run() -> io::Result<()> {
    write!(
        &mut rainbowcoat::Colors::configure(io::stdout(), 2.0, 0.4, 0.0),
        r#"            ,        ,
            /(_,    ,_)\
            \ _/    \_ /
            //        \\
            \\ (@)(@) //
             \'="=="='/
         ,===/        \===,
        ",===\        /===,"
        " ,==='------'===, "
         "                "
"#
    )?;
    Ok(())
}

fn main() {
    run().unwrap()
}
