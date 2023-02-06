# Go, Node, Rust, Zig: A Benchmark

## イントロ

vim-jp slack の #lang-go で、Go vs Node が土日に繰り広げられていました(月曜日気づいた)。
mattn さんが、Go と Node の速度を比較するベンチマークを書いていたので、それを Rust と Zig で書いてみました。

## ベンチマーク

| Language | Requests per second | Time per request |
| :------- | :------------------ | :--------------- |
| bun      | 22807.88            | 0.438            |
| deno     | 32913.58            | 0.304            |
| go       | 85736.82            | 0.117            |
| node     | 11187.35            | 0.894            |
| rust     | 20267.08            | 0.493            |
| zig      | **未測定**          | **未測定**       |

ということで、**Go が一番速い**です。

Go >> Deno > Rust > bun > Node という結果になりました。

## 注意

この計測は、特定の言語やフレームワークを批判するものではないです。
それぞれの言語やフレームワークには、それぞれの良いところ、悪いところがあります。

## 参考

## ベンチマークのコード

### Go

```go
package main

import (
	"fmt"
	"net/http"
)

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Add("Content-Type", "text/html")
		fmt.Fprintf(w, "<h1>Hello World</h1>")
	})
	http.ListenAndServe(":3000", nil)
}

```

### Rust

```rust
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
		let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
		for stream in listener.incoming() {
				let stream = stream.unwrap();
				handle_connection(stream);
		}
}

fn handle_connection(mut stream: TcpStream) {
		let mut buffer = [0; 512];
		stream.read(&mut buffer).unwrap();
		let response = "HTTP/1.1 200 OK\r\n\r\n<h1>Hello World</h1>";
		stream.write(response.as_bytes()).unwrap();
		stream.flush().unwrap();
}

```

### Zig

```zig
const std = @import("std");
const net = std.net;
const StreamServer = net.StreamServer;
const Address = net.Address;
pub const io_mode = .evented;

pub fn main() anyerror!void {
  var stream_server = StreamServer.init(.{});
  defer stream_server.close();
  const address = try Address.resolveIp("127.0.0.1", 3000);
  try stream_server.listen(address);
  while (true) {
    const client = try stream_server.accept();
      const response = "HTTP/1.1 200 OK\r\n\r\n<h1>Hello World</h1>";
      try client.write(response);
    }
}

```

### Node

```js
const http = require("http");
const port = 3000;

const server = http.createServer((request, response) => {
  response.writeHead(200, {
    "Content-Type": "text/html",
  });
  const responseMessage = "<h1>Hello World</h1>";
  response.end(responseMessage);
});

server.listen(port);
```

### Deno

```ts
// Start listening on port 8080 of localhost.
const server = Deno.listen({ port: 3000 });
console.log(`HTTP webserver running.  Access it at:  http://localhost:3000/`);

// Connections to the server will be yielded up as an async iterable.
for await (const conn of server) {
  // In order to not be blocking, we need to handle each connection individually
  // without awaiting the function
  serveHttp(conn);
}

async function serveHttp(conn: Deno.Conn) {
  // This "upgrades" a network connection into an HTTP connection.
  const httpConn = Deno.serveHttp(conn);
  // Each request sent over the HTTP connection will be yielded as an async
  // iterator from the HTTP connection.
  for await (const requestEvent of httpConn) {
    // The native HTTP server uses the web standard `Request` and `Response`
    // objects.
    const body = "<h1>Hello World</h1>";
    // The requestEvent's `.respondWith()` method is how we send the response
    // back to the client.
    requestEvent.respondWith(
      new Response(body, {
        status: 200,
      })
    );
  }
}
```

### bun

```bun
// TypeScript: http.ts
export default {
  port: 3000,
  fetch(request: Request) {
    return new Response("Hello World");
  },
};

```

## ベンチマークの実行

### install

```sh
sudo apt install apache2-utils
```

### run

```sh
ab -k -c 10 -n 10000 http://127.0.0.1:3000/
```
