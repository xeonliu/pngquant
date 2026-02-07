#!/usr/bin/env python3
"""
Example usage of pngquant Python library

This example demonstrates different ways to use the pngquant library:
1. Basic file compression
2. Compression with custom options
3. Byte stream compression
"""

import pngquant
import sys
import os


def example_basic_file_compression():
    """Example 1: Basic file compression with default settings"""
    print("Example 1: Basic file compression")
    print("-" * 50)
    
    # Create test input (you need to have an input.png file)
    input_file = "input.png"
    output_file = "output_basic.png"
    
    if not os.path.exists(input_file):
        print(f"Error: {input_file} not found")
        print("Please provide an input.png file to test with")
        return False
    
    try:
        pngquant.compress_file(input_file, output_file)
        print(f"✓ Successfully compressed {input_file} -> {output_file}")
        
        # Show file size comparison
        input_size = os.path.getsize(input_file)
        output_size = os.path.getsize(output_file)
        reduction = (1 - output_size / input_size) * 100
        
        print(f"  Input size:  {input_size:,} bytes")
        print(f"  Output size: {output_size:,} bytes")
        print(f"  Reduction:   {reduction:.1f}%")
        return True
    except Exception as e:
        print(f"✗ Error: {e}")
        return False


def example_custom_options():
    """Example 2: Compression with custom quality settings"""
    print("\nExample 2: Compression with custom quality settings")
    print("-" * 50)
    
    input_file = "input.png"
    output_file = "output_custom.png"
    
    if not os.path.exists(input_file):
        print(f"Error: {input_file} not found")
        return False
    
    try:
        # Create custom options
        options = pngquant.PngQuantOptions(
            quality_min=65,    # Minimum quality (0-100)
            quality_max=80,    # Maximum quality (0-100)
            speed=1,           # Speed 1-11 (1=slowest, best quality)
            colors=128,        # Maximum colors in palette (2-256)
            dithering_level=0.8  # Dithering amount (0.0-1.0)
        )
        
        print(f"Using options: {options}")
        
        pngquant.compress_file(input_file, output_file, options)
        print(f"✓ Successfully compressed with custom options")
        
        # Show file size comparison
        input_size = os.path.getsize(input_file)
        output_size = os.path.getsize(output_file)
        reduction = (1 - output_size / input_size) * 100
        
        print(f"  Input size:  {input_size:,} bytes")
        print(f"  Output size: {output_size:,} bytes")
        print(f"  Reduction:   {reduction:.1f}%")
        return True
    except Exception as e:
        print(f"✗ Error: {e}")
        return False


def example_bytes_compression():
    """Example 3: Compress PNG from bytes to bytes"""
    print("\nExample 3: Byte stream compression")
    print("-" * 50)
    
    input_file = "input.png"
    output_file = "output_bytes.png"
    
    if not os.path.exists(input_file):
        print(f"Error: {input_file} not found")
        return False
    
    try:
        # Read PNG file as bytes
        with open(input_file, "rb") as f:
            input_data = f.read()
        
        print(f"Read {len(input_data):,} bytes from {input_file}")
        
        # Compress using bytes interface
        compressed_data = pngquant.compress_bytes(input_data)
        
        # Write compressed bytes to file
        with open(output_file, "wb") as f:
            f.write(compressed_data)
        
        print(f"✓ Successfully compressed using byte streams")
        
        # Show size comparison
        reduction = (1 - len(compressed_data) / len(input_data)) * 100
        print(f"  Input size:  {len(input_data):,} bytes")
        print(f"  Output size: {len(compressed_data):,} bytes")
        print(f"  Reduction:   {reduction:.1f}%")
        return True
    except Exception as e:
        print(f"✗ Error: {e}")
        return False


def main():
    print("=" * 50)
    print("pngquant Python Library Examples")
    print("=" * 50)
    print()
    
    # Check for input file
    if len(sys.argv) > 1:
        # Use provided file
        if os.path.exists(sys.argv[1]):
            # Create a symlink or copy as input.png
            import shutil
            shutil.copy(sys.argv[1], "input.png")
        else:
            print(f"Error: File '{sys.argv[1]}' not found")
            sys.exit(1)
    
    # Run all examples
    results = []
    results.append(example_basic_file_compression())
    results.append(example_custom_options())
    results.append(example_bytes_compression())
    
    # Summary
    print("\n" + "=" * 50)
    print(f"Results: {sum(results)}/{len(results)} examples succeeded")
    print("=" * 50)
    
    if all(results):
        print("\n✓ All examples completed successfully!")
        return 0
    else:
        print("\n✗ Some examples failed")
        return 1


if __name__ == "__main__":
    sys.exit(main())
