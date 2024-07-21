package main

import (
	"bufio"
	"log"
	"net"
	"time"
)

func main() {
    tcp, err := net.Listen("tcp",":4000")
    if err != nil {
        log.Fatal(err)
    }
    defer tcp.Close()

    log.Println("Listening ",tcp.Addr().String())

    for {
        conn, err := tcp.Accept()
        if err != nil {
            log.Println(err)
        }

        go func(c net.Conn) {
            defer c.Close()
            reader := bufio.NewReader(c)

            data, err := reader.ReadString(';')

            if err != nil {
                log.Fatal(err)
            }

            log.Println(string(data))

            time.Sleep(1 * time.Second)

            _, err = c.Write([]byte("Goblin"))

            if err != nil {
                log.Fatal(err)
            }
        }(conn)
    }
}

