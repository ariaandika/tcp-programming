let port = parseInt(process.argv[2]);

if (!port || isNaN(port)) {
    console.error("Port required")
    process.exit(1);
}

const socket = await Bun.connect({
    port,
    hostname: "127.0.0.1",
    socket: {
        data(_, data) {
            console.log("Then",data.toString());
        },
        close() {
            console.log("Tcp Closed")
        },
    }
})

socket.write("Deez;");

