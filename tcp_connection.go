package irc

import (
	"bufio"
	"net"
)

type TCPConnection struct {
	read_connection  *bufio.Reader
	write_connection *bufio.Writer
}

func TCPConnect(url, port string) (*TCPConnection, error) {
	conn, err := net.Dial("tcp", url+":"+port)
	if err != nil {
		return nil, err
	}

	return &TCPConnection{
		bufio.NewReader(conn),
		bufio.NewWriter(conn)}, nil
}

func (tcp TCPConnection) Write(message string) {
	tcp.write_connection.WriteString(message)
	tcp.write_connection.Flush()
}

func (tcp TCPConnection) Read() (string, error) {
	return tcp.read_connection.ReadString('\n')
}
