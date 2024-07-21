// @ts-check
const net = require("node:net")
const ps  = require('node:child_process')

function getPort() {
    let ports = ps.spawnSync("ss",["-tln"])
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


/** @type {any} */
const tcp = net.createServer((socket) => {
    socket.on('data', data => {
        console.log(data.toString())
        setTimeout(() => socket.end('Oof'), 1000)
    })

}).listen(getPort())


console.log(`Listening ${tcp.address().port}`)

