package irc

import (
	"bytes"
)

//"net"

type IRC struct {
	connection Chat_stream
}

func Connect(name string, conn Chat_stream) IRC {
	irc := IRC{conn}
	irc.SendRaw("NICK " + name)
	irc.SendRaw("USER " + name + " * 0 :" + name + " " + name)
	return irc
}

func (irc IRC) SendMessage(channel, message string) {
	irc.SendRaw("PRIVMSG " + channel + " " + message)
}

func (irc IRC) SendRaw(message string) {
	irc.connection.Write(message + "\r\n")
}

func (irc IRC) Read(respond func(string) error) error {
	buf := &bytes.Buffer{}
	data := make([]byte, 512)
	n := 0
	var err error
	for {
		n, err = irc.connection.Read(data)
		if err != nil {
			break
		}
		buf.Write(data[:n])
		if n > 0 {
			err = respond(string(buf.Bytes()))
			if err != nil {
				break
			}
			buf = &bytes.Buffer{}
		}
	}
	return err
}
