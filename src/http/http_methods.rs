use std::path::PathBuf;

use crate::http::{http_log, http_request::HttpRequest};
pub fn main_page(path: &str, packet: &mut HttpRequest) {
    let _ = packet.respond_string("HTTP/1.1 200 OK\r\n\r\n");
    let _ = packet.respond_data(
        &std::fs::read(PathBuf::from(path).join("index.html")).expect("Missing index.html page."),
    );
}
// Reads the requested path, and if it matches a file on the server, returns the file in the body
pub fn get(mut packet: HttpRequest, path: &'static str) {
    if let Some(name) = packet.path() {
        let name = name[1..].to_owned();
        if name == "" {
            http_log!("Requesting root page");
            main_page(&path, &mut packet);
        } else {
            let root = PathBuf::from(path)
                .canonicalize()
                .expect("Static directory must exist.");
            if let Ok(path) = root.join(name).canonicalize() {
                if path.starts_with(root) {
                    let _ = packet.respond_string("HTTP/1.1 200 OK\r\n\r\n");
                    let _ = packet.respond_data(
                        &std::fs::read(path)
                            .expect("File was deleted right after being requested."),
                    );
                }
            }
        }
    }
    packet.read_all();
    http_log!("{packet}\n");
}
