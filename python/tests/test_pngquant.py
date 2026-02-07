"""
Tests for pngquant Python bindings
"""
import os
import tempfile
import struct
import zlib


def create_test_png(width=100, height=100):
    """Create a simple test PNG image"""
    # PNG file header
    png_data = bytearray(b'\x89PNG\r\n\x1a\n')

    def write_chunk(chunk_type, data):
        chunk_len = len(data)
        crc_data = chunk_type + data
        crc = zlib.crc32(crc_data) & 0xFFFFFFFF
        return struct.pack('>I', chunk_len) + crc_data + struct.pack('>I', crc)

    # IHDR chunk
    ihdr = struct.pack('>IIBBBBB', width, height, 8, 2, 0, 0, 0)  # RGB, 8-bit
    png_data += write_chunk(b'IHDR', ihdr)

    # IDAT chunk - create RGB image data
    rgb_data = bytearray()
    for y in range(height):
        rgb_data.append(0)  # Filter type: None
        for x in range(width):
            # Create gradient colors
            r = (x * 255) // width
            g = (y * 255) // height
            b = ((x + y) * 255) // (width + height)
            rgb_data.extend([r, g, b])

    compressed = zlib.compress(bytes(rgb_data), 9)
    png_data += write_chunk(b'IDAT', compressed)

    # IEND chunk
    png_data += write_chunk(b'IEND', b'')

    return bytes(png_data)


def test_module_info():
    """Test that module loads and has correct metadata"""
    import pngquant
    
    assert hasattr(pngquant, '__version__')
    assert hasattr(pngquant, 'compress_file')
    assert hasattr(pngquant, 'compress_bytes')
    assert hasattr(pngquant, 'PngQuantOptions')
    print("✓ Module info test passed")


def test_file_compression():
    """Test basic file compression"""
    import pngquant
    
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as input_file:
        input_path = input_file.name
        input_file.write(create_test_png())
    
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as output_file:
        output_path = output_file.name
    
    try:
        pngquant.compress_file(input_path, output_path)
        
        input_size = os.path.getsize(input_path)
        output_size = os.path.getsize(output_path)
        
        assert output_size > 0, "Output file should not be empty"
        assert output_size < input_size, "Output should be smaller than input"
        
        print(f"✓ File compression test passed (reduced from {input_size} to {output_size} bytes)")
    finally:
        os.unlink(input_path)
        os.unlink(output_path)


def test_file_compression_with_options():
    """Test file compression with custom options"""
    import pngquant
    
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as input_file:
        input_path = input_file.name
        input_file.write(create_test_png())
    
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as output_file:
        output_path = output_file.name
    
    try:
        # Use more relaxed quality settings to avoid QualityTooLow error
        options = pngquant.PngQuantOptions(
            quality_min=0,
            quality_max=100,
            speed=2,
            colors=128
        )
        pngquant.compress_file(input_path, output_path, options)
        
        assert os.path.getsize(output_path) > 0
        print("✓ File compression with options test passed")
    finally:
        os.unlink(input_path)
        os.unlink(output_path)


def test_bytes_compression():
    """Test byte stream compression"""
    import pngquant
    
    input_data = create_test_png()
    compressed = pngquant.compress_bytes(input_data)
    
    assert len(compressed) > 0, "Compressed data should not be empty"
    assert len(compressed) < len(input_data), "Compressed data should be smaller"
    
    # Verify it's still a valid PNG by checking header
    assert compressed[:8] == b'\x89PNG\r\n\x1a\n', "Should be a valid PNG"
    
    print(f"✓ Bytes compression test passed (reduced from {len(input_data)} to {len(compressed)} bytes)")


def test_bytes_compression_with_options():
    """Test byte stream compression with custom options"""
    import pngquant
    
    input_data = create_test_png()
    options = pngquant.PngQuantOptions(
        quality_min=60,
        quality_max=85,
        speed=3,
        colors=200,
        dithering_level=0.5
    )
    compressed = pngquant.compress_bytes(input_data, options)
    
    assert len(compressed) > 0
    assert compressed[:8] == b'\x89PNG\r\n\x1a\n'
    print("✓ Bytes compression with options test passed")


def test_options_properties():
    """Test PngQuantOptions property getters and setters"""
    import pngquant
    
    # Test default values
    opts = pngquant.PngQuantOptions()
    assert opts.quality_min == 0
    assert opts.quality_max == 100
    assert opts.speed == 4
    assert opts.colors == 256
    assert abs(opts.dithering_level - 1.0) < 0.001  # Use approximate comparison for float
    assert opts.posterize == 0
    
    # Test setters
    opts.quality_min = 70
    opts.quality_max = 90
    opts.speed = 2
    opts.colors = 128
    opts.dithering_level = 0.8
    opts.posterize = 2
    
    # Verify
    assert opts.quality_min == 70
    assert opts.quality_max == 90
    assert opts.speed == 2
    assert opts.colors == 128
    assert abs(opts.dithering_level - 0.8) < 0.001  # Use approximate comparison for float
    assert opts.posterize == 2
    
    print("✓ Options properties test passed")


def test_options_validation():
    """Test that invalid options are rejected"""
    import pngquant
    
    # Test quality validation
    try:
        pngquant.PngQuantOptions(quality_min=150)
        assert False, "Should have raised ValueError for quality_min > 100"
    except ValueError:
        pass
    
    # Test quality_min > quality_max
    try:
        pngquant.PngQuantOptions(quality_min=80, quality_max=60)
        assert False, "Should have raised ValueError for quality_min > quality_max"
    except ValueError:
        pass
    
    # Test speed validation
    try:
        pngquant.PngQuantOptions(speed=15)
        assert False, "Should have raised ValueError for speed > 11"
    except ValueError:
        pass
    
    # Test colors validation
    try:
        pngquant.PngQuantOptions(colors=300)
        assert False, "Should have raised ValueError for colors > 256"
    except ValueError:
        pass
    
    # Test dithering_level validation
    try:
        pngquant.PngQuantOptions(dithering_level=2.0)
        assert False, "Should have raised ValueError for dithering_level > 1.0"
    except ValueError:
        pass
    
    print("✓ Options validation test passed")


def run_all_tests():
    """Run all tests"""
    print("=" * 60)
    print("Running pngquant Python bindings tests")
    print("=" * 60)
    
    tests = [
        test_module_info,
        test_file_compression,
        test_file_compression_with_options,
        test_bytes_compression,
        test_bytes_compression_with_options,
        test_options_properties,
        test_options_validation,
    ]
    
    failed = 0
    for test in tests:
        try:
            test()
        except Exception as e:
            print(f"✗ {test.__name__} failed: {e}")
            failed += 1
    
    print("=" * 60)
    if failed == 0:
        print(f"All {len(tests)} tests passed!")
        return 0
    else:
        print(f"{failed}/{len(tests)} tests failed")
        return 1


if __name__ == "__main__":
    import sys
    sys.exit(run_all_tests())
