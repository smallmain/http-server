<!-- 名字 -->
<h1 align="center">HTTP Server</h1>
<!-- 描述 -->
<p align="center">一个简单的 HTTP 服务器实现。</p>
<br/>

阅读其它语言版本: [English](./README.md) | 简体中文

## 起步

这是一个使用 Rust 编写的 HTTP 服务器，有以下特性：

- 简单易用
- 支持多线程处理请求
- 支持设置监听地址、端口
- 支持将指定目录设置为根路由

> 该项目仅私人使用，未经过严格测试，不建议用于非常正式的生产环境。

## 如何使用

在项目根目录执行

```shell
cargo run
```

即可在 `127.0.0.1:80` 运行一个简单的 HTTP 服务器。

你可以执行

```shell
cargo run -- --help
```

查看所有可用的命令行参数。

## 许可证

[MIT @ SmallMain](./LICENSE)
