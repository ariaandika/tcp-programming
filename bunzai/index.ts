
function getPort() {
    const ports = Bun.spawnSync({
        cmd: ["ss","-tln"],
        stdout: "pipe",
    })
        .stdout
        .toString()
        .split("\n")
        .filter(e => e.startsWith("LISTEN"))
        .map(e => e.split(/\s+/)[3])
        .filter(e => e && e.match(/40\d\d$/))
        .map(e => parseInt(e.slice(-4)))

    for (let i = 4000;i <= 4020;i++) {
        if (!ports.includes(i)) {
            return i
        }
    }

    console.error("No Port Available")
    process.exit(1);
}




const tcp = Bun.listen({
    hostname: "127.0.0.1",
    port: getPort(),
    socket: {
        async data(socket, data) {
            console.log(data.toString());
            await Bun.sleep(1000);
            socket.write("Nutz");
            socket.end()
        },
    },
})

console.log(`Listening in ${tcp.hostname}:${tcp.port}`)


