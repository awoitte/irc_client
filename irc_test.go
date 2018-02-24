package irc

import (
	"fmt"
	"testing"
)

type MockStream struct {
	lastWrite string
}

func (mock *MockStream) Read(b []byte) (int, error) {
	return 1, nil
}

func (mock *MockStream) Write(message string) {
	mock.lastWrite = message
}

func Test_it_should_add_channel_and_privmsg(t *testing.T) {
	var stream = MockStream{}
	var irc = Connect("test", &stream)
	irc.SendMessage("chan", "test")
	expected := "PRIVMSG chan test\r\n"
	if stream.lastWrite != expected {
		t.Errorf("expected: %v, recieved %v", expected, stream.lastWrite)
	}
}

func Test_it_should_attach_line_breaks(t *testing.T) {
	var stream = MockStream{}
	var irc = Connect("test", &stream)
	irc.SendRaw("test")
	expected := "test\r\n"
	if stream.lastWrite != expected {
		t.Errorf("expected: %v, recieved %v", expected, stream.lastWrite)
	}
}

func Test_it_should_read_until_Err_return(t *testing.T) {
	var stream = &MockStream{}
	var irc = Connect("test", stream)
	calls := 0

	irc.Read(func(message string) error {
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
