use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, about, version)]
pub struct Cli {
    /// 监听地址
    #[arg(long, short, default_value = "127.0.0.1")]
    pub address: String,
    /// 监听端口
    #[arg(long, short, default_value = "80")]
    pub port: u16,
    /// 默认目录路由
    #[arg(long, short, default_value = ".")]
    pub default_route: String,
    /// 处理线程数
    #[arg(long, short, default_value = "4")]
    pub thread_count: usize,
}
