from asyncio.streams import StreamReader, StreamWriter
from asyncio.tasks import sleep
import asyncio, subprocess as ps

def getPort():
    ss = ps.run(["ss","-tln"],stdout=ps.PIPE)

    lines = [line.split()[3].decode().split(':') for line in ss.stdout.splitlines() if line.startswith(b"LISTEN")]
    lines = [line for line in lines if int(line[1]) >= 4000 and int(line[1]) <= 4020]

    ports = iter(range(4000, 4020))
    addrs = iter(lines)

    for [[addr,port2], port] in zip(addrs, ports):
        if port != int(port2):
            return (addr, port)

    for port in ports:
        return (str('127.0.0.1'),port)

    print("No Port Available")
    exit(1)

# https://stackoverflow.com/questions/48506460/python-simple-socket-client-server-using-asyncio
async def server():
    (host, port) = getPort()
    server = await asyncio.start_server(handler,host=host,port=int(port))
    print(f'Listening {host}:{port}')
    async with server:
        await server.serve_forever()

async def handler(reader: StreamReader, writer: StreamWriter):
    print((await reader.read(255)).decode())
    await sleep(1)
    writer.write(b"IOC")
    writer.close()

asyncio.run(server())

