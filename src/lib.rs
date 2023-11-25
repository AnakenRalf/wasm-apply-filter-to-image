use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::{Engine as _, engine::general_purpose};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use std::io::Cursor;


#[wasm_bindgen]
pub fn gray_scale(encoded_file: &str) -> String {
    log(&"Grayscale image conversion called".into());
    let mut buffer = Vec::<u8>::new();

    general_purpose::STANDARD

     .decode_vec(encoded_file, &mut buffer, ).unwrap();

    

    let mut img = load_from_memory(&buffer).unwrap();
    log(&"Image loaded".into());

    img = img.grayscale();

    

    log(&"Grayscalle conversion is appliied to Image".into());

    let mut bff = Vec::new();
    // let mut writer = Cursor::new(&mut bff);

    img.write_to(&mut Cursor::new(&mut bff), Png).unwrap();

    let encoded_img = general_purpose::STANDARD.encode(&bff);
    let encoded_img = format!("data:image/png;base64,{}", encoded_img);
    encoded_img

    
}