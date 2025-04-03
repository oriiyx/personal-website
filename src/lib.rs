mod utils;
mod commands;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

// Re-export the CommandRegistry
pub use commands::CommandRegistry;

// Set up panic hook
cfg_if! {
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        // use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

// Initialize function to set up things like panic hook
#[wasm_bindgen(start)]
pub fn init() {
    utils::set_panic_hook();
}