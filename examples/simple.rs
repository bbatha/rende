extern crate rende;

use rende::component::Div;
#[cfg( target = "asmjs-unknown-emscripten" )]
use rende::rende;

fn main() {
    let content = "Rende!";
    // TODO(bbatha): need equalivalent of "innerText" for div
    let div = Div::with_children(vec![content]);
    #[cfg( target = "asmjs-unknown-emscripten" )]
    rende("rende-entry", div);
}

