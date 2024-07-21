
const socket = await Bun.connect({
    port: 4000,
    hostname: "127.0.0.1",
    socket: {
        data(socket, data) {
            console.log("Then",data.toString());
        },
    }
})

socket.write("Deez;");

