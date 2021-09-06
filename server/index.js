const WebSocket = require('ws');
const server = new WebSocket.Server({
    port: 8081
});

let users = [];
let sockets = [];

server.on('connection', (socket) => {
    sockets.push(socket);
    socket.on('message', data => {
        // console.log(JSON.parse(data));
        handleData(JSON.parse(data));
        // sockets.forEach(s => s.send(data));
    });
    socket.on('close', () => {
        sockets = sockets.filter(s => s != socket);
    })
});


let handleData = (data) => {
    if(data.metadata === "username") {
        handleUserName(data);
    } else if(data.metadata === "message") {
        handleMessage(data);
    }
}

let handleUserName = (data) => {
    let username = data.content;
    users.push(username);
    console.log(`Lenght: ${users.length}`);
    if(users.length % 2 == 0) {
        //inverte o nome dos usuarios, manda o nome do oponente para cada usuario
        let firstUserData = {
            metadata: "ready",
            content: users[users.length - 2]
        }
        let secondUserData = {
            metadata: "ready",
            content: users[users.length - 1]
        }
        // sockets.forEach(socket => socket.send(data));
        sockets[sockets.length - 1].send(JSON.stringify(firstUserData));
        sockets[sockets.length - 2].send(JSON.stringify(secondUserData));
    } else {    
        socket = sockets[sockets.length - 1];
        let data = {
            metadata: "waiting",
            content: ""
        }
        socket.send(JSON.stringify(data));
    }
}

let handleMessage = (data) => {
    // console.log(`From handleMessage: ${me}`)
    sockets.forEach(socket => socket.send(JSON.stringify(data)));
}
