mod game_of_life;

#[cfg(wee_alloc)]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// if building for wasm-bindgen export all the types
#[cfg(feature = "wasm")]
pub use game_of_life::*;

/// The raw C-like API for python interop
#[cfg(feature = "python_wasmtime")]
pub mod python_wasmtime {
    use super::*;
    use game_of_life::*;

    #[no_mangle]
    pub extern "C" fn game_of_life_new(
        seed: u32,
        width: u32,
        height: u32,
    ) -> *mut game_of_life::GameOfLife {
        let game = Box::new(GameOfLife::new_with_seed(seed, width, height));

        Box::into_raw(game)
    }

    #[no_mangle]
    pub extern "C" fn game_of_life_free(game: *mut GameOfLife) {
        let _ = unsafe { Box::from_raw(game) };
    }

    #[no_mangle]
    pub extern "C" fn game_of_life_tick(game: *mut GameOfLife) {
        unsafe {
            game.as_mut().unwrap().tick();
        }
    }

    #[no_mangle]
    pub extern "C" fn game_of_life_get(
        game: *mut GameOfLife,
        x: u32,
        y: u32,
    ) -> Option<Cell> {
        unsafe { game.as_mut().unwrap().get(x, y) }
    }
}
