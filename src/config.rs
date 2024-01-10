use std::net::IpAddr;

/// HTTP 服务器配置
pub struct Config {
    /// 监听地址
    pub address: IpAddr,
    /// 监听端口
    pub port: u16,
    /// 默认目录路由
    ///
    /// 当没有设置指定路由时，会使用目录下的文件作为响应内容
    pub default_route: Option<String>,
    /// 处理线程数
    pub thread_count: usize,
}
