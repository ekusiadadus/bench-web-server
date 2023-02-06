const std = @import("std");
const net = std.net;
const StreamServer = net.StreamServer;
const Address = net.Address;

pub fn main() anyerror!void {
  var stream_server = StreamServer.init(.{});
  defer stream_server.close();
  const address = try Address.resolveIp("127.0.0.1", 8080);
  try stream_server.listen(address);
  while (true) {
    const client = try stream_server.accept();
    try handler(client.stream);
  }

}

fn handler(stream: net.Stream) anyerror!void {
  defer stream.close();
  try stream.writer().print("Hello, World!", .{});
}