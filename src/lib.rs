use core::str;
use pyo3::prelude::*;
use pyo3::pyclass;
// use pyo3::PyObjectProtocol;
use std::collections::HashMap;

mod huffman;
use huffman::*;

#[pyclass]
#[derive(Default)]
struct Value {
    root: Box<Node>,
    assign: HashMap<char, String>,
    text: String,
    #[pyo3(get)]
    isEncoded: bool,
}

#[pymethods]
impl Value {
    #[new]
    pub fn new() -> Self {
        Value::default()
    }
    // #[PyObjectProtocol]
    pub fn __str__(&self) -> PyResult<String> {
        Ok(String::from(format!(
            "Huffpy.Main Class - \nData: {} ({} bytes)",
            self.text,
            self.text.len()
        )))
    }
    pub fn Decode(&self, text: &str) -> PyResult<String> {
        // println!("{}", text);
        if (self.isEncoded != true) {
            return Ok(String::from(""));
        }
        Ok(decode_string(text, &self.root))
    }
    pub fn Create(&mut self, text: &str) -> PyResult<u32> {
        // let text = text;
        let res: HashMap<char, i32> = frequency(&text);
        self.text = String::from(text);
        let mut p = huffman_array_maker(res);
        huffman_tree_maker(&mut p);
        // get the root Huffman tree Node
        let root = p.pop().unwrap();
        // hold the root value insdie the strusture
        self.root = root;

        let mut h: HashMap<char, String> = HashMap::new();
        assign_codes(&self.root, &mut h, "".to_string());

        self.assign = h;
        Ok(0)

        // Return 1 if there is any error
    }
    fn Encode(&mut self) -> String {
        if (self.isEncoded == true) {
            String::from("")
        } else {
            self.isEncoded = true;
            encode_string(self.text.as_str(), &self.assign)
        }
    }
}

// We use #[pymethod] instead of #[pyproto]
// #[pyproto]
// impl PyObjectProtocol for Value {
//     fn __str__(%self) -> PyResult<String> {
//         Ok(format!("This is your special class"));
//     }
// }

/// A Python module implemented in Rust.
#[pymodule]
fn huffpy(_py: Python, m: &PyModule) -> PyResult<()> {
    /* Main Class of Huffpy library */
    m.add_class::<Value>()?;
    Ok(())
}
