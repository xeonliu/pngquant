# pngquant Python Library

Python bindings for [pngquant](https://pngquant.org) - a lossy PNG compressor that significantly reduces file sizes while maintaining high quality.

## Features

- **Easy-to-use Python API** - Simple functions for PNG compression
- **File I/O support** - Compress PNG files directly
- **Byte stream support** - Process PNG data in memory without temporary files
- **Configurable quality** - Control compression quality from 0-100
- **Fast Rust implementation** - Core algorithm implemented in Rust for performance
- **Alpha channel support** - Properly handles transparency

## Installation

### From PyPI (when published)

```bash
pip install pngquant
```

### From source

```bash
# Install maturin (Rust-Python build tool)
pip install maturin

# Build and install
maturin develop --release --features python
```

## Quick Start

### Basic File Compression

```python
import pngquant

# Compress a PNG file with default settings
pngquant.compress_file("input.png", "output.png")
```

### Custom Quality Settings

```python
import pngquant

# Create custom options
options = pngquant.PngQuantOptions(
    quality_min=65,      # Minimum quality (0-100)
    quality_max=80,      # Maximum quality (0-100)
    speed=1,             # Speed 1-11 (1=slowest/best, 11=fastest)
    colors=128,          # Max colors in palette (2-256)
    dithering_level=0.8  # Dithering amount (0.0-1.0)
)

pngquant.compress_file("input.png", "output.png", options)
```

### Byte Stream Compression

```python
import pngquant

# Read PNG from file
with open("input.png", "rb") as f:
    input_data = f.read()

# Compress in memory
compressed_data = pngquant.compress_bytes(input_data)

# Write compressed data
with open("output.png", "wb") as f:
    f.write(compressed_data)
```

### Streaming Workflow (No Temporary Files)

```python
import pngquant

# Process entirely in memory
with open("input.png", "rb") as f:
    input_data = f.read()

options = pngquant.PngQuantOptions(quality_min=70, quality_max=85)
compressed_data = pngquant.compress_bytes(input_data, options)

# Now upload to cloud, send over network, etc.
# No temporary files created!
```

## API Reference

### Functions

#### `compress_file(input_path, output_path, options=None)`

Compress a PNG file.

**Parameters:**
- `input_path` (str): Path to input PNG file
- `output_path` (str): Path to output PNG file
- `options` (PngQuantOptions, optional): Compression options

**Raises:**
- `IOError`: If file operations fail
- `ValueError`: If options are invalid

#### `compress_bytes(input_data, options=None)`

Compress PNG from bytes to bytes.

**Parameters:**
- `input_data` (bytes): Input PNG data
- `options` (PngQuantOptions, optional): Compression options

**Returns:**
- `bytes`: Compressed PNG data

**Raises:**
- `IOError`: If compression fails
- `ValueError`: If options are invalid

### Classes

#### `PngQuantOptions(quality_min=0, quality_max=100, speed=4, colors=256, dithering_level=1.0, posterize=0)`

Configuration options for PNG compression.

**Parameters:**
- `quality_min` (int, 0-100): Minimum quality threshold. Images below this quality won't be saved.
- `quality_max` (int, 0-100): Target maximum quality. The algorithm will use the minimum number of colors needed to achieve this quality.
- `speed` (int, 1-11): Speed/quality trade-off. 1 is slowest with best quality, 11 is fastest.
- `colors` (int, 2-256): Maximum number of colors in the output palette.
- `dithering_level` (float, 0.0-1.0): Amount of dithering. 0.0 is no dithering, 1.0 is full Floyd-Steinberg dithering.
- `posterize` (int): Reduce precision of the palette by this many bits. Useful for low-depth displays.

**Properties:**
All parameters are available as read/write properties:
```python
options = pngquant.PngQuantOptions()
options.quality_min = 65
options.quality_max = 80
options.speed = 1
```

## Examples

See [python/examples/example.py](python/examples/example.py) for more detailed examples.

## Performance

The Python bindings use the same high-performance Rust implementation as the pngquant CLI tool. For best performance:

- Use `speed=4` (default) for balanced performance
- Use `speed=1` for best quality (slower)
- Use `speed=11` for fastest compression (lower quality)
- Process images in parallel using Python's multiprocessing

## License

pngquant is dual-licensed:

- **GPL v3** or later with an additional copyright notice
- **Commercial license** for use in non-GPL software

See the [COPYRIGHT](COPYRIGHT) file and [https://pngquant.org](https://pngquant.org) for details.

## Related Projects

- [pngquant CLI](https://github.com/kornelski/pngquant) - Command-line tool
- [libimagequant](https://github.com/ImageOptim/libimagequant) - Underlying quantization library
- [oxipng](https://github.com/shssoichiro/oxipng) - Lossless PNG optimizer

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Support

- Report issues: [GitHub Issues](https://github.com/kornelski/pngquant/issues)
- Documentation: [GitHub README](https://github.com/kornelski/pngquant#readme)
- Website: [https://pngquant.org](https://pngquant.org)
