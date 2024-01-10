<!-- 名字 -->
<h1 align="center">HTTP Server</h1>
<!-- 描述 -->
<p align="center">A simple http server.</p>
<br/>

Read this in other languages: English | [简体中文](./README_zh-CN.md)

## Introduction

This is an HTTP server written in Rust with the following features:

- Simple and easy to use

- Support multi-thread processing of requests

- Support setting listening address and port

- Support setting the specified directory as the root route

> This project is for private use only, has not been rigorously tested, and is not recommended for use in very formal production environments.

## How to use

Execute in the project root directory:

```shell
cargo run
```

You can run a simple HTTP server at `127.0.0.1:80`.

You can execute

```shell
cargo run -- --help
```

view all available command line parameters.

## License

[MIT @ SmallMain](./LICENSE)
