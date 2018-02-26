package irc

type Chat_stream interface {
	Read() (string, error)
	Write(string)
}
