package irc

type Chat_stream interface {
	Read([]byte) (int, error)
	Write(string)
}
