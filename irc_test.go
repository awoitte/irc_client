package irc

import (
	"fmt"
	"testing"
)

func Test_it_should_send_nick_and_user_on_connect(t *testing.T) {
	stream := MockStream{}
	listener := stream.StartListening()
	go Connect("test", &stream)
	expect_write(t, listener, "NICK test\r\n")
	expect_write(t, listener, "USER test * 0 :test test\r\n")

}

func Test_it_should_add_channel_and_privmsg(t *testing.T) {
	stream := MockStream{}
	irc := Connect("test", &stream)

	stream.StartListening()
	go irc.SendMessage("chan", "test")
	expect_write(t, stream.writes, "PRIVMSG chan test\r\n")
}

func Test_it_should_attach_line_breaks(t *testing.T) {
	stream := MockStream{}
	irc := Connect("test", &stream)
	stream.StartListening()
	go irc.SendRaw("test")
	expect_write(t, stream.writes, "test\r\n")
}

func Test_it_should_read_until_Err_return(t *testing.T) {
	stream := &MockStream{}
	irc := Connect("test", stream)
	calls := 0

	irc.ReadLoop(1, func(message string) error {
		calls++
		if calls > 1 {
			return fmt.Errorf("done reading")
		}
		return nil
	})

	if calls != 2 {
		t.Errorf("called too many times %d", calls)
	}
}
