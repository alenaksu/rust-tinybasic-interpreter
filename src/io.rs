use std::io::{stdin, stdout, Write};

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = terminal)]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn terminal_read_line() -> Result<JsValue, JsValue>;
    fn terminal_write(line: &str);
    fn terminal_clear();
    fn terminal_set_prompt(prompt: &str);
}

#[wasm_bindgen(js_namespace = io)]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn io_load_file() -> Result<JsValue, JsValue>;
    fn io_save_file(source: &str);
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn read_line() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    buffer
}

#[cfg(not(target_arch = "wasm32"))]
pub fn write(line: &str) {
    print!("{}", line);
    stdout().flush().unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn write_line(line: &str) {
    write(line);
    write("\n");
}

#[cfg(not(target_arch = "wasm32"))]
pub fn clear() {
    std::process::Command::new("clear").status().unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn set_prompt(prompt: &str) {
    print!("{}", prompt);
    stdout().flush().unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn load_file() -> Option<String> {
    None
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_file(source: &str) {}

#[cfg(target_arch = "wasm32")]
pub async fn read_line() -> String {
    let value = terminal_read_line().await;

    match value {
        Ok(value) => value.as_string().unwrap(),
        Err(error) => error.as_string().unwrap(),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn write(line: &str) {
    terminal_write(line);
}

#[cfg(target_arch = "wasm32")]
pub fn write_line(line: &str) {
    terminal_write(line);
    terminal_write("\n");
}

#[cfg(target_arch = "wasm32")]
pub fn clear() {
    terminal_clear();
}

#[cfg(target_arch = "wasm32")]
pub fn set_prompt(prompt: &str) {
    terminal_set_prompt(prompt);
}

#[cfg(target_arch = "wasm32")]
pub async fn load_file() -> Option<String> {
    let value = io_load_file().await;

    match value {
        Ok(value) => Some(value.as_string().unwrap()),
        Err(error) => None,
    }
}

#[cfg(target_arch = "wasm32")]
pub fn save_file(source: &str) {
    io_save_file(source);
}
