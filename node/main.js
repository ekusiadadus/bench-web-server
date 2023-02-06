const http = require("http");
const port = 3000;

const server = http.createServer((request, response) => {
  response.writeHead(200, {
    "Content-Type": "text/html"});
  const responseMessage = "<h1>Hello World</h1>";
  response.end(responseMessage)
});

server.listen(port);
