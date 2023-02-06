// TypeScript: http.ts
export default {
  port: 3000,
  fetch(request: Request) {
    return new Response("<h1>Hello World</h1>");
  },
};
