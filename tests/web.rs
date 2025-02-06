//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use jpeg_compressor::compress_jpeg;
use image::{ImageOutputFormat, DynamicImage, GenericImageView};
use std::io::Cursor;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_compress_jpeg_wasm() {
    // Imagen de prueba en base64 (pixel blanco 1x1)
    // Generar imagen 100x100
    let img = DynamicImage::new_rgb8(100, 100);
    let mut buffer = Vec::new();
    img.write_to(&mut Cursor::new(&mut buffer), ImageOutputFormat::Jpeg(100)).unwrap();
    let input = base64::encode(&buffer);
    
    let compressed = compress_jpeg(&input, 75).unwrap();
    let original_size = base64::decode(input).unwrap().len();
    let compressed_size = base64::decode(compressed).unwrap().len();
    
    assert!(compressed_size < original_size);
}

#[wasm_bindgen_test]
fn should_handle_errors() {
    // Test de input inválido
    let result = compress_jpeg("invalid_base64", 75);
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_image_dimensions_preserved() {
    // Generar imagen 100x100
    let img = DynamicImage::new_rgb8(100, 100);
    let mut buffer = Vec::new();
    img.write_to(&mut Cursor::new(&mut buffer), ImageOutputFormat::Jpeg(100)).unwrap();
    let input = base64::encode(&buffer);
    
    let compressed = compress_jpeg(&input, 80).unwrap();
    let decoded = image::load_from_memory(&base64::decode(compressed).unwrap()).unwrap();
    
    assert_eq!(decoded.dimensions(), (100, 100));
}
