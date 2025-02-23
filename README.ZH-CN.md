# nlf

[![Crates.io Version](https://img.shields.io/crates/v/nlf?style=for-the-badge&color=blue)](https://crates.io/crates/nlf)

[English](README.md) | 简体中文

在文件末尾追加换行符（LF）。

你是否有这样的困扰------------

使用 CLI 处理别人发来的文本文件却莫名其妙地崩溃？

POSIX 规范要求文本文件的换行符为 LF（`\n`），并且文件末尾必须有一个换行符，Linux 和 macOS 系统都遵循这个规范，
但是在 Windows 系统中，文本文件的换行符为 CRLF（`\r\n`），并且末尾不需要换行符。

一些遵循 POSIX 规范的 CLI 对文本的处理基于上述的假设，当处理 Windows 系统生成的文本文件时，可能会出现问题
*（我伟大的 PowerShell 就不会有这种问题！PowerShell，启动！）*。

甚至你可能还会收到两种换行符混合的文件，在编辑器里看一切正常，但是 CLI 一跑就裂开了。

在挠着头折腾了几个小时后你终于发现了问题所在。

------------这时候应该怎么办呢？

> * 忍气吞声：疯狂查阅文档，复制粘贴了一堆看不懂的 shell 脚本，最后还是没解决问题。
> * 移花接木，说自己电脑快没油了，让上次没有请你吃疯狂星期四的人来处理，并且不提醒这个换行符的问题。
> * 釜底抽薪，找到给你发文件的人并绑起来，当着面把他的系统重装为 Arch Linux，并要求重新发一份新的文件给你，这样就不会有换行符问题了。
> * 破釜沉舟，要求他自己修复这个问题，否则就抱着他的几十 TB 的硬盘跳楼。
> * 其人之道：下次给他发文件的时候偷偷把换行符搞错。
> * 原神启动：搞一个 CLI，让他在给你发文件之前先用这个工具处理一下。

## 安装

### Cargo

```sh
cargo install nlf --locked --profile release-small
```

## 使用方法

```sh
# 追加末尾换行符（如果没有的话）
nlf a.txt

# 修复 dir 目录下所有 .txt 文件的换行符（如果末尾没有换行符则追加）
find dir -type f -name '*.txt' -exec nlf {} \;
```

## 为什么不用 Async Rust 写？

<img src="assets/flowerQ.png" width="200" alt="花 Q"/>

## 许可证

[License](LICENSE)
