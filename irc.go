package irc

import "errors"

type IRC struct {
	connection Chat_stream
	channel    string
}

func Connect(name string, conn Chat_stream) IRC {
	irc := IRC{conn, ""}
	irc.SendRaw("NICK " + name)
	irc.SendRaw("USER " + name + " * 0 :" + name + " " + name)
	return irc
}

func (irc *IRC) SendMessage(message string) error {
	if irc.channel == "" {
		return errors.New("You must join a channel before you can send a message")
	}
	irc.SendRaw("PRIVMSG #" + irc.channel + " " + message)
	return nil
}

func (irc *IRC) SendRaw(message string) {
	irc.connection.Write(message + "\r\n")
}

func (irc *IRC) JoinChannel(channel string) {
	irc.channel = channel
	irc.SendRaw("JOIN #" + channel)
}

func (irc *IRC) PartChannel() error {
	if irc.channel == "" {
		return errors.New("You must join a channel before you can send a message")
	}
	irc.SendRaw("PART #" + irc.channel)
	irc.channel = ""
	return nil
}

func (irc *IRC) ReadLoop(chunk_size int, respond func(string) error) error {
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
