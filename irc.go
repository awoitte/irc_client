package irc

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

func (irc IRC) ReadLoop(chunk_size int, respond func(string) error) error {
	for {
		data, err := irc.connection.Read()
		if err != nil {
			return err
		}
		err = respond(data)
		if err != nil {
			break
		}
	}
	return nil
}
