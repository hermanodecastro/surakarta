const WebSocket = require('ws');
const server = new WebSocket.Server({
    port: 8081
});

let sockets = [];

server.on('connection', (socket) => {
    console.log(socket);
    sockets.push(socket);
    socket.on('message', msg => {
        console.log(JSON.parse(msg));
        sockets.forEach(s => s.send(msg));
    });
    socket.on('close', () => {
        sockets = sockets.filter(s => s != socket);
    })
});
