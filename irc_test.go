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

func Test_it_should_send_join_message_on_join(t *testing.T) {
	stream := &MockStream{}
	irc := Connect("test", stream)

	stream.StartListening()
	go irc.JoinChannel("test")
	expect_write(t, stream.writes, "JOIN #test\r\n")
}

func Test_it_should_add_channel_and_privmsg(t *testing.T) {
	stream := MockStream{}
	irc := Connect("test", &stream)
	irc.JoinChannel("chan")

	stream.StartListening()
	go irc.SendMessage("test")
	expect_write(t, stream.writes, "PRIVMSG #chan test\r\n")
}

func Test_it_should_require_channel_to_send_message(t *testing.T) {
	stream := &MockStream{}
	irc := Connect("test", stream)
	err := irc.SendMessage("test_message")
	if err == nil {
		t.Fail()
	}
}

func Test_it_should_send_part_message(t *testing.T) {
	stream := MockStream{}
	irc := Connect("test", &stream)
	irc.JoinChannel("chan")

	stream.StartListening()
	go irc.PartChannel()
	expect_write(t, stream.writes, "PART #chan\r\n")
}

func Test_it_should_require_channel_to_send_part(t *testing.T) {
	stream := &MockStream{}
	irc := Connect("test", stream)
	err := irc.PartChannel()
	if err == nil {
		t.Fail()
	}
}
