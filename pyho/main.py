import socket



with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    # Prevent socket still alive on exit
    s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)

    s.bind(("127.0.0.1",4000))
    s.listen()

    conn, addr = s.accept()

    with conn:
        data = conn.recv(5)

        print(data)

print("oof")

