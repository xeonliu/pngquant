/*
** © 2024 by Kornel Lesiński.
**
** Python bindings for pngquant
*/

use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::exceptions::{PyValueError, PyIOError};
use crate::core::{compress_png_bytes, compress_png_file, QuantOptions};

/// Python wrapper for QuantOptions
#[pyclass]
#[derive(Clone)]
pub struct PngQuantOptions {
    inner: QuantOptions,
}

#[pymethods]
impl PngQuantOptions {
    /// Create new options with defaults
    #[new]
    #[pyo3(signature = (quality_min=0, quality_max=100, speed=4, colors=256, dithering_level=1.0, posterize=0))]
    fn new(
        quality_min: u8,
        quality_max: u8,
        speed: u32,
        colors: u32,
        dithering_level: f32,
        posterize: u32,
    ) -> PyResult<Self> {
        if quality_min > 100 || quality_max > 100 {
            return Err(PyValueError::new_err("Quality values must be between 0 and 100"));
        }
        if quality_min > quality_max {
            return Err(PyValueError::new_err("quality_min must be <= quality_max"));
        }
        if speed < 1 || speed > 11 {
            return Err(PyValueError::new_err("Speed must be between 1 (slowest) and 11 (fastest)"));
        }
        if colors < 2 || colors > 256 {
            return Err(PyValueError::new_err("Colors must be between 2 and 256"));
        }
        if dithering_level < 0.0 || dithering_level > 1.0 {
            return Err(PyValueError::new_err("Dithering level must be between 0.0 and 1.0"));
        }

        Ok(PngQuantOptions {
            inner: QuantOptions {
                quality_min,
                quality_max,
                speed,
                colors,
                dithering_level,
                posterize,
            },
        })
    }

    /// Get quality_min
    #[getter]
    fn quality_min(&self) -> u8 {
        self.inner.quality_min
    }

    /// Set quality_min
    #[setter]
    fn set_quality_min(&mut self, value: u8) -> PyResult<()> {
        if value > 100 {
            return Err(PyValueError::new_err("Quality must be between 0 and 100"));
        }
        if value > self.inner.quality_max {
            return Err(PyValueError::new_err("quality_min must be <= quality_max"));
        }
        self.inner.quality_min = value;
        Ok(())
    }

    /// Get quality_max
    #[getter]
    fn quality_max(&self) -> u8 {
        self.inner.quality_max
    }

    /// Set quality_max
    #[setter]
    fn set_quality_max(&mut self, value: u8) -> PyResult<()> {
        if value > 100 {
            return Err(PyValueError::new_err("Quality must be between 0 and 100"));
        }
        if value < self.inner.quality_min {
            return Err(PyValueError::new_err("quality_max must be >= quality_min"));
        }
        self.inner.quality_max = value;
        Ok(())
    }

    /// Get speed
    #[getter]
    fn speed(&self) -> u32 {
        self.inner.speed
    }

    /// Set speed (1-11, where 1 is slowest/best quality and 11 is fastest)
    #[setter]
    fn set_speed(&mut self, value: u32) -> PyResult<()> {
        if value < 1 || value > 11 {
            return Err(PyValueError::new_err("Speed must be between 1 and 11"));
        }
        self.inner.speed = value;
        Ok(())
    }

    /// Get colors
    #[getter]
    fn colors(&self) -> u32 {
        self.inner.colors
    }

    /// Set maximum number of colors (2-256)
    #[setter]
    fn set_colors(&mut self, value: u32) -> PyResult<()> {
        if value < 2 || value > 256 {
            return Err(PyValueError::new_err("Colors must be between 2 and 256"));
        }
        self.inner.colors = value;
        Ok(())
    }

    /// Get dithering level
    #[getter]
    fn dithering_level(&self) -> f32 {
        self.inner.dithering_level
    }

    /// Set dithering level (0.0-1.0, where 0 is no dithering and 1 is full)
    #[setter]
    fn set_dithering_level(&mut self, value: f32) -> PyResult<()> {
        if value < 0.0 || value > 1.0 {
            return Err(PyValueError::new_err("Dithering level must be between 0.0 and 1.0"));
        }
        self.inner.dithering_level = value;
        Ok(())
    }

    /// Get posterize bits
    #[getter]
    fn posterize(&self) -> u32 {
        self.inner.posterize
    }

    /// Set posterize bits (reduce precision of palette)
    #[setter]
    fn set_posterize(&mut self, value: u32) -> PyResult<()> {
        self.inner.posterize = value;
        Ok(())
    }

    fn __repr__(&self) -> String {
        format!(
            "PngQuantOptions(quality_min={}, quality_max={}, speed={}, colors={}, dithering_level={}, posterize={})",
            self.inner.quality_min,
            self.inner.quality_max,
            self.inner.speed,
            self.inner.colors,
            self.inner.dithering_level,
            self.inner.posterize
        )
    }
}

/// Compress a PNG file
///
/// Args:
///     input_path: Path to input PNG file
///     output_path: Path to output PNG file
///     options: Optional PngQuantOptions instance for compression settings
///
/// Example:
///     >>> import pngquant
///     >>> pngquant.compress_file("input.png", "output.png")
///     >>> # Or with custom options
///     >>> options = pngquant.PngQuantOptions(quality_min=65, quality_max=80, speed=1)
///     >>> pngquant.compress_file("input.png", "output.png", options)
#[pyfunction]
#[pyo3(signature = (input_path, output_path, options=None))]
fn compress_file(
    input_path: &str,
    output_path: &str,
    options: Option<&PngQuantOptions>,
) -> PyResult<()> {
    let opts = options.map(|o| o.inner.clone()).unwrap_or_default();
    
    compress_png_file(input_path, output_path, &opts)
        .map_err(|e| PyIOError::new_err(format!("Failed to compress PNG file: {}", e)))
}

/// Compress PNG from bytes to bytes
///
/// Args:
///     input_data: Input PNG data as bytes
///     options: Optional PngQuantOptions instance for compression settings
///
/// Returns:
///     Compressed PNG data as bytes
///
/// Example:
///     >>> import pngquant
///     >>> with open("input.png", "rb") as f:
///     ...     input_data = f.read()
///     >>> compressed = pngquant.compress_bytes(input_data)
///     >>> with open("output.png", "wb") as f:
///     ...     f.write(compressed)
#[pyfunction]
#[pyo3(signature = (input_data, options=None))]
fn compress_bytes<'py>(
    py: Python<'py>,
    input_data: &[u8],
    options: Option<&PngQuantOptions>,
) -> PyResult<Bound<'py, PyBytes>> {
    let opts = options.map(|o| o.inner.clone()).unwrap_or_default();
    
    let result = compress_png_bytes(input_data, &opts)
        .map_err(|e| PyIOError::new_err(format!("Failed to compress PNG: {}", e)))?;
    
    Ok(PyBytes::new_bound(py, &result))
}

/// pngquant Python module
///
/// Convert 24/32-bit PNG images to efficient 8-bit format with alpha channel.
/// This module provides Python bindings to the pngquant library, which uses
/// advanced quantization algorithms implemented in Rust for high-quality
/// palette generation.
///
/// Functions:
///     compress_file: Compress a PNG file
///     compress_bytes: Compress PNG from bytes to bytes
///
/// Classes:
///     PngQuantOptions: Configuration options for PNG compression
///
/// Example:
///     >>> import pngquant
///     >>> # Compress a file with default settings
///     >>> pngquant.compress_file("input.png", "output.png")
///     >>> 
///     >>> # Compress with custom quality settings
///     >>> options = pngquant.PngQuantOptions(quality_min=65, quality_max=80)
///     >>> pngquant.compress_file("input.png", "output.png", options)
///     >>>
///     >>> # Compress from bytes
///     >>> with open("input.png", "rb") as f:
///     ...     data = f.read()
///     >>> compressed = pngquant.compress_bytes(data)
#[pymodule]
fn pngquant(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compress_file, m)?)?;
    m.add_function(wrap_pyfunction!(compress_bytes, m)?)?;
    m.add_class::<PngQuantOptions>()?;
    
    // Add module metadata
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__doc__", "Convert 24/32-bit PNG images to efficient 8-bit format with alpha channel")?;
    
    Ok(())
}
