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
    //write <h1> Hello World </h1> to the client
    try client.write("HTTP/1.1 200 OK\r\n\r\nContent-Type: text/html\r\n\r\n<h1>Hello World</h1>", .{});
  }

}
