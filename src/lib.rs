//mod utils;

use base64::{decode, encode};
use image::ImageOutputFormat;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compress_jpeg(base64_input: &str, quality: u8) -> Result<String, JsValue> {
    console_error_panic_hook::set_once();
    // Decodificar Base64 a datos binarios
    let decoded_data = decode(base64_input).map_err(|e| e.to_string())?;

    // Cargar la imagen desde los datos binarios
    let img = image::load_from_memory(&decoded_data).map_err(|e| e.to_string())?;

    // Comprimir la imagen en formato JPEG con la calidad especificada
    let mut compressed_data: Vec<u8> = Vec::new();
    img.write_to(
        &mut Cursor::new(&mut compressed_data),
        ImageOutputFormat::Jpeg(quality),
    ).map_err(|e| e.to_string())?;

    // Codificar la imagen comprimida a Base64
    let compressed_base64 = encode(&compressed_data);

    Ok(compressed_base64)
}
