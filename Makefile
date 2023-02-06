.phony:

build-go:
	go build go/main.go && ./main

build-rust:
	rustc rust/main.rs && ./main

build-zig:
	zig build-exe zig/main.zig && ./main

run-go:
	go run go/main.go

run-node:
	node node/main.js

run-bun:
  bun run main.ts

run-deno:
	deno run --allow-net deno/main.ts

bench:
	ab -k -c 10 -n 10000 http://127.0.0.1:3000/

bench-go:
	ab -k -c 10 -n 10000 http://127.0.0.1:3000/ > bench/go.txt

bench-rust:
	ab -k -c 10 -n 10000 http://127.0.0.1:3000/ > bench/rust.txt

bench-node:
	ab -k -c 10 -n 10000 http://127.0.0.1:3000/ > bench/node.txt

bench-zig:
	ab -k -c 10 -n 10000 http://127.0.0.1:3000/ > bench/zig.txt
