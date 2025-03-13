<div align="center">
<image src="lcj.png" alt="Descripción de la imagen">
</div>

# JPEG-COMPRESSOR

**JPEG-COMPRESSOR** is a high-performance tool for compressing JPEG images using **Rust** and **WebAssembly**. It provides a simple API to reduce the file size of JPEG images while maintaining visual quality, making it ideal for web and backend applications.

---

## Features

- ⚡ **Blazing Fast Compression**: Built with Rust for maximum performance.
- 🌐 **Web-Compatible**: Compiled to WebAssembly for seamless use in browsers.
- 📦 **Simple API**: Compress images directly from base64 strings.
- 🎚️ **Custom Quality**: Adjust compression level (0-100) to balance quality and size.
- 🔄 **Lossless Option**: Supports lossless compression for minimal quality degradation.

---

## Installation

### Rust (via GitHub)
Add this to your `Cargo.toml`:
```toml
[dependencies]
jpeg-compressor = { git = "https://github.com/jeancarlos1111/jpeg-compressor.git" }
```

### Web (npm)
```bash
 npm install @jeanzg32/jpeg-compressor
 ```

## Usage

### Rust
```rust
use jpeg_compressor::compress_jpeg;

fn main()-> Result<String> {
    let base64_image = "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQEASABIAAD/...";
    let quality = 80;
    let compressed = compress_jpeg(base64_image, quality)?;
    println!("Compressed size: {}", compressed.len());
    Ok(())
}
 ```

 ### JavaScript (WebAssembly)
```js
import init, { compress_jpeg } from '@jeanzg32/jpeg-compressor';

async function compressImage() {
  await init();
  const base64Image = document.getElementById('image-input').value;
  const quality = 75;
  try {
    const compressed = await compress_jpeg(base64Image, quality);
    console.log('Compression success:', compressed);
  } catch (error) {
    console.error('Compression failed:', error);
  }
}
 ```


> [!NOTE]
> Calidades menores a 80 pueden producir artefactos visibles.
>
> No soporta otros formatos de imagen (solo JPEG).
>
> Para imágenes muy grandes, considerar usar Web Workers.
---
## License

* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)


### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
