use std::{fs, io::Write, net::TcpStream, sync::Arc};

use mime_guess::mime;

use crate::thread_pool::Job;

/// HTTP 请求路由
pub enum Route {
    /// 静态文件
    File(String),
    /// 自定义函数
    Custom(Arc<dyn Fn(&TcpStream) -> String + Send + Sync>),
}

impl Route {
    pub fn create_handler(&self, mut stream: TcpStream) -> Job {
        match self {
            Route::File(path) => {
                let path = path.clone();
                Box::new(move || {
                    let status = "HTTP/1.1 200 OK";
                    let content = fs::read_to_string(&path).unwrap();
                    let length = content.len();
                    let mime_type =
                        mime_guess::from_path(&path).first_or(mime::APPLICATION_OCTET_STREAM);
                    let response = format!("{status}\r\nContent-Type: {mime_type}\r\nContent-Length: {length}\r\n\r\n{content}");
                    stream.write_all(response.as_bytes()).unwrap();
                })
            }
            Route::Custom(handler) => {
                let handler = handler.clone();
                Box::new(move || {
                    let response = handler(&stream);
                    stream.write_all(response.as_bytes()).unwrap();
                })
            }
        }
    }
}
