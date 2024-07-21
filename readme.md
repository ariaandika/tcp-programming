# TCP Programming

learn how programming languages provide tcp

languages:

- rust
- javascript (Bun)
- javascript (Node)
- go
- python

# Flow

every program will connect to each other socket.

one can send message to the others, the others knows
where the message come from.

# Port Discoveries

there is 2 options, currently option 1 is used

## Option 1, use core utils

using `ss -tln` to list listening tcp ports, then find
ports between 4000 to 4020 where its not used

## Option 2, brute force

when a single program spin up, it will try to
bind to port 4000, if it fail, retry with 4001
and so on.

however, it will stop retrying until port 4020

then, it can connect to one of the previously bound
port to ask for other clients port, and connect to each
of them

