mod engine;
#[macro_use]
mod browser;

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Deserialize)]
struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}
#[derive(Deserialize)]
struct Cell {
    frame: Rect,
}

#[derive(Deserialize)]
struct Sheet {
    frames: HashMap<String, Cell>,
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let context = browser::context().expect("Could not get 2d context");

    browser::spawn_local(async move {
        let json = browser::fetch_json("walk_the_dog_assets-0.0.7/sprite_sheets/rhb.json")
            .await
            .expect("Could not fetch rhb.json");

        let sheet: Sheet = serde_wasm_bindgen::from_value(json)
            .expect("Could not convert rhb.json into a Sheet structure");

        let image = engine::load_image("walk_the_dog_assets-0.0.7/sprite_sheets/rhb.png")
            .await
            .expect("Could not load rhb.png");

        let mut frame = -1;
        let interval_callback = Closure::wrap(Box::new(move || {
            frame = (frame + 1) % 8;
            let frame_name = format!("Run ({}).png", frame + 1);

            context.clear_rect(0.0, 0.0, 600.0, 600.0);

            let sprite = sheet.frames.get(&frame_name).expect("Cell not found");
            context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &image,
                sprite.frame.x.into(),
                sprite.frame.y.into(),
                sprite.frame.w.into(),
                sprite.frame.h.into(),
                300.0,
                300.0,
                sprite.frame.w.into(),
                sprite.frame.h.into(),
            );
        }) as Box<dyn FnMut()>);
        browser::window()
            .unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                interval_callback.as_ref().unchecked_ref(),
                50,
            );
        interval_callback.forget();
    });

    Ok(())
}

async fn fetch_json(json_path: &str) -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_str(json_path)).await?;
    let resp = resp_value.dyn_into::<web_sys::Response>()?;

    wasm_bindgen_futures::JsFuture::from(resp.json()?).await
}
