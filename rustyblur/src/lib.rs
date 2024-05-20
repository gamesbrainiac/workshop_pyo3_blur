use pyo3::prelude::*;
use imageproc::filter::gaussian_blur_f32;

// Defines a function `gaussian_blur` that can be called from Python.
// It's annotated with #[pyfunction], indicating it's a Python-callable function.
#[pyfunction]
fn gaussian_blur(file_name: String, new_file_name: String) -> PyResult<String> {
    // Opens an image file specified by `file_name`. If the file cannot be opened,
    // the program will panic and display the error message.
    let image = image::open(&file_name).expect("File could not be loaded");
    // Converts the image to RGB format, if not already,
    // and applies a Gaussian blur with a sigma value of 6.0.
    let blurred = gaussian_blur_f32(&image.into_rgb8(), 6.0);
    // Saves the blurred image to a new file specified by `new_file_name`.
    // Ignores the result (hence the `_ =`).
    let _ = blurred.save(&new_file_name);
    // Returns the new file name wrapped in a PyResult,
    // indicating successful completion to Python.
    Ok(new_file_name)
}

/// A Python module implemented in Rust.
// Defines a new Python module called `rustyblur`. 
// The #[pymodule] attribute indicates that this 
// function initializes a Python module.
#[pymodule]
fn rustyblur(_py: Python, m: &PyModule) -> PyResult<()> {
    // Adds the `gaussian_blur` function to the module,
    // making it callable from Python as `rustyblur.gaussian_blur`.
    m.add_function(wrap_pyfunction!(gaussian_blur, m)?)?;
    // Returns Ok, indicating the module was successfully created.
    Ok(())
}
