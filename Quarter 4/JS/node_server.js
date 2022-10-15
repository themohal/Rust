const http = require('http');

const hostname = '127.0.0.1';
const port = 3000;
const port2 = 4000;

const server1 = http.createServer((req, res) => {
    res.statusCode = 200;
    res.setHeader('Content-Type', 'text/plain');
    res.end('Hello, World 1st server\n');
});
const server2 = http.createServer((req, res) => {
    res.statusCode = 200;
    res.setHeader('Content-Type', 'text/plain');
    res.end('Hello, World 2nd server\n');
});

server1.listen(port, hostname, () => {
    console.log(`Server running at http://${hostname}:${port}/`);
});
server2.listen(port2, hostname, () => {
    console.log(`Server running at http://${hostname}:${port2}/`);
});