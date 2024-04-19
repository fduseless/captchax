use pyo3::{prelude::*, types::{PyBytes, PyTuple}};

mod captcha;

#[pyfunction]
#[pyo3(signature = (len=4,difficulty=5,line=true,noise=false,format="png"))]
#[pyo3(text_signature = "
len: num of character,
difficulty: range [1,10]
line: draw bezier curve or ellipse
noise: whether add gaussian noise
format: png | jpg | jpeg | webp
")]
pub fn create_image<'a>(_py: Python<'a>,
    len: usize,
    difficulty: usize,
    line: bool,
    noise: bool,
    format: &'a str,
) -> &'a PyTuple {
    let c = captcha::CaptchaBuilder::new();
    let out = c
        .complexity(difficulty)
        .length(len)
        .line(line)
        .noise(noise)
        .format(&format)
        .build();
    return PyTuple::new(_py, &[
        out.text.to_object(_py),
        PyBytes::new(_py, &out.image).to_object(_py),
    ])
}

/// A Python module implemented in Rust.
#[pymodule]
fn captchax(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_image, m)?)?;
    Ok(())
}
