use crate::http::{Request, Response, StatusCode, Method};
use crate::server::Handler;
use std::fs;

pub struct Router {
    public_path: String
}

impl Router {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    pub fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Invalid directory {}", file_path);
                    None
                }
            }
            Err(_) => None
        }
    }
}

impl Handler for Router {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::OK, self.read_file("index.html")),
                path =>  match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::OK, Some(contents)),
                    _ => Response::new(StatusCode::NotFound, None)
                }
            }
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}
