const server = Deno.listen({ port: 3000 });

for await (const conn of server) {
  serveHttp(conn);
}

async function serveHttp(conn: Deno.Conn) {
  const httpConn = Deno.serveHttp(conn);
  for await (const requestEvent of httpConn) {
    const body = "<h1>Hello World</h1>";
    requestEvent.respondWith(
      new Response(body, {
        status: 200,
      })
    );
  }
}
