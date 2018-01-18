#[macro_use]
extern crate stdweb;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;

use stdweb::unstable::TryInto;
use stdweb::web::event::PopStateEvent;
use stdweb::web::{
    IEventTarget,
    History,
    set_timeout,
    math,
    Math,
};

fn main() {
    stdweb::initialize();
    set_timeout(|| {
        js!( console.log(@{math().random()}) );
    }, 20);
}
