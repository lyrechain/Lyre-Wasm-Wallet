#![forbid(unsafe_code)]
#![allow(unused_braces)]

mod offchain;
mod pages;
mod routing;

use log::{trace, Level};
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::HashChangeEvent;

use crate::routing::{App, Route};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn main(parent_id: Option<String>) -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Trace).unwrap();

    //TODO
    //wasm_bindgen_futures::spawn_local(crate::offchain::airdrop());

    let gizmo = Gizmo::from(App {
        route: Route::WalletCheckpoint,
    });
    let view = View::from(gizmo.view_builder());
    if let Some(id) = parent_id {
        let parent = utils::document().get_element_by_id(&id).unwrap();
        view.run_in_container(&parent)
    } else {
        view.run()
    }
}
