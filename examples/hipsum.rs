extern crate rainbowcoat;
use std::io::{self, Write};

fn run() -> io::Result<()> {
    write!(
        &mut rainbowcoat::stdout(),
        "Kombucha cray keffiyeh freegan hammock bushwick kitsch gochujang pinterest listicle man bun actually.
Pickled austin tilde raw denim bicycle rights. Trust fund echo park cliche, artisan cold-pressed gluten-free
hammock put a bird on it live-edge banh mi microdosing marfa chillwave unicorn vape. Kogi chambray master cleanse
locavore. Fixie plaid intelligentsia tumeric, single-origin coffee shaman hell of hexagon freegan. Taxidermy retro
irony, poke prism iPhone actually glossier dreamcatcher kombucha bitters cornhole. Tumblr retro skateboard helvetica
chicharrones meggings four dollar toast yr lumbersexual 90's sriracha. Pug pop-up williamsburg, lomo distillery pork belly
coloring book PBR&B readymade waistcoat church-key glossier austin tbh fam. Kale chips four dollar toast gochujang swag fashion axe.
Truffaut la croix gochujang, helvetica kogi PBR&B locavore. Wayfarers snackwave fashion axe, retro fixie stumptown readymade yuccie
skateboard microdosing you probably haven't heard of them mumblecore portland waistcoat."
    )?;
    Ok(())
}

fn main() {
    run().unwrap()
}
