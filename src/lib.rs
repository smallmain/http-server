//! # 使用方法
//!
//! 在项目根目录执行
//!
//! ```shell
//! cargo run
//! ```
//!
//! 即可在 `127.0.0.1:80` 运行一个简单的 HTTP 服务器。
//! 
//! 你可以执行
//! 
//! ```shell
//! cargo run -- --help
//! ```
//! 
//! 查看所有可用的命令行参数。

use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader},
    net::{TcpListener, TcpStream},
    path::Path,
    sync::Arc,
};

pub use config::Config;
pub use route::Route;
pub use thread_pool::ThreadPool;

pub mod config;
pub mod route;
pub mod thread_pool;

/// HTTP 服务器
pub struct Server {
    /// 配置
    pub config: Config,
    /// 是否处于运行状态
    pub running: bool,
    /// 路由集合
    routes: HashMap<String, Route>,
    /// 线程池
    thread_pool: ThreadPool,
}

impl Server {
    /// 创建 HTTP 服务器实例
    pub fn new(config: Config) -> Server {
        Server {
            thread_pool: ThreadPool::new(config.thread_count),
            config,
            running: false,
            routes: HashMap::new(),
        }
    }

    /// 启动服务器
    pub fn start(&mut self) -> Result<(), io::Error> {
        if self.running {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "server is already running",
            ));
        }

        self.running = true;

        let listener = TcpListener::bind((self.config.address, self.config.port))?;

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            if let Some(Ok(request_line)) = BufReader::new(&stream).lines().next() {
                if let Some(request_path) = request_line.split_whitespace().nth(1) {
                    // 查找路由
                    if let Some(route) = self.routes.get(request_path) {
                        self.start_handle_thread(route, stream);
                        continue;
                    }

                    // 查找默认路由
                    if let Some(default_route) = self.config.default_route.clone() {
                        let path = Path::new(&default_route).join(&request_path[1..]);
                        if path.exists() {
                            let route = Route::File(
                                path.into_os_string().into_string().unwrap().to_string(),
                            );
                            self.start_handle_thread(&route, stream);
                            continue;
                        }
                    }

                    // 自定义 404 页面
                    if let Some(route) = self.routes.get("/404") {
                        self.start_handle_thread(route, stream);
                        continue;
                    }

                    // 默认 404 页面
                    let route = Route::Custom(Arc::new(move |_| {
                        format!("HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n")
                    }));
                    self.start_handle_thread(&route, stream);
                }
            }
        }

        return Ok(());
    }

    /// 启动新的处理线程
    fn start_handle_thread(&self, route: &Route, stream: TcpStream) {
        let handler = route.create_handler(stream);
        self.thread_pool.execute(handler);
    }

    /// 设置路由
    pub fn set_route(&mut self, path: &str, route: Route) {
        self.routes.insert(path.to_string(), route);
    }
}
