import socket
import subprocess as ps

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

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    # Prevent socket still alive on exit
    s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)

    s.bind(getPort())
    s.listen()

    (addr, port) = s.getsockname()
    print(f"Listening {addr}:{port}")

    conn, addr = s.accept()

    with conn:
        data = conn.recv(5)

        print(data)

