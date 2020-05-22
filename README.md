# EmojiBomb

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

This is designed to be a bomb man game with Emojis as playable characters.

The core library is a framework that can be used to implement the game with different types of transport and UI. This repo includes a pair of server and client implementations with [TCP](https://tools.ietf.org/html/rfc793) as transport and [Cursive](https://github.com/gyscos/cursive), a TUI(Text User Interface) library as UI. This pair of implementations serves as example implementation.

- [simple TCP server implementation](./simple_tcp_server)
- [simple TCP Cursive client implementation](./simple_tcp_client)

## Under development

This repo is still very early stage

## Demo setup

To set up the demo/example implementation, your system needs to have [ncurses](https://en.wikipedia.org/wiki/Ncurses). Other than that, everything else can be installed on most of modern OS's.

- Rust and Cargo. Recommands using [rustup](https://rustup.rs/) to install them. Mac users can find it in Homebrew.

### starting up the server

```sh
git clone https://github.com/funfoolsuzi/emojibomb.git

cd emojibomb/simple_tcp_server

cargo run
```

example server will be listening on port 8888. It will binds to wildcard IPv4 addresses. You can change it to local loopback(127.0.0.1) to be safer on `emojibomb/simple_tcp_server/src/main.rs` if you want. (as of May 21, 2020)

### starting up the client

open another terminal

```sh
cd emojibomb/simple_tcp_client

cargo run
```

client will prompt you to enter ip and port. If you are running locally, 127.0.0.1:8888 would work. Or your LAN or ISP IP with port 8888. (as of May 21, 2020)

__ESC to exit client !!!!__

## Architecture

```txt

+-Emojibomb---------+    +-Transport---------------------------+
|                   |    |                                     |
|  +-------------+ --msg--> +-------------------------------+  |         Server
|  |Server Engine|  |    |  |Server Transport implementation|------> Implementation
|  +-------------+ <-msg--+ +---------------+-+-------------+  |
|                   |    |                  ^ |                |
|                   |    |                  | |                |
|                   |    |                  | |                |
|                   |    |                  | |                |
|                   |    |                  | |                |
|                   |    |                  | v                |
|  +-------------+  |    |  +---------------+-+-------------+  |
|  |Client Engine| <-msg--+ |Client Transport implementation+--------------+
|  +------+------+  |    |  +------------------------+------+  |           |
|         |         |    |                           ^         |           |
+---------|---------+    +---------------------------|---------+           v
          |                                          |                   Client
          |                                         msg              Implementation
          |             +-UI-------------------------|----------+          ^
          |             |                            |          |          |
          |             |  +---------------+ +-------+-------+  |          |
          +-state update-> |Output(Display)| |Input(Keyboard)|  +----------+
                        |  +---------------+ +---------------+  |
                        |                                       |
                        +---------------------------------------+


```
