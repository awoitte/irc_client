package irc

import (
	"fmt"
	"net"
)

type TCPConnection struct {
	connection net.Conn
}

func TCPConnect(url, port string) (*TCPConnection, error) {
	conn, err := net.Dial("tcp", url+":"+port)
	if err != nil {
		return nil, err
	}
	return &TCPConnection{conn}, nil
}

func (tcp TCPConnection) Write(message string) {
	fmt.Fprintf(tcp.connection, message)
}
func (tcp TCPConnection) Read(b []byte) (int, error) {
	return tcp.connection.Read(b)
}
