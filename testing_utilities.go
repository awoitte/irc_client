package irc

import (
	"testing"
	"time"
)

type MockStream struct {
	writes chan string
}

func (mock *MockStream) Read(b []byte) (int, error) {
	return 1, nil
}

func (mock *MockStream) Write(message string) {
	if mock.writes != nil {
		select {
		case mock.writes <- message:
			return
		case <-time.After(time.Millisecond * 100):
			panic("Timed out waiting to write to mock stream. You're probably starting to listen too early")
		}
	}
}

func (mock *MockStream) StartListening() chan string {
	mock.writes = make(chan string)
	return mock.writes
}

func expect_write(t *testing.T, writes chan string, expected string) {
	select {
	case recieved := <-writes:
		if recieved == expected {
			return
		} else {
			t.Errorf("expected '%v', recieved '%v'", expected, recieved)
		}
	case <-time.After(time.Millisecond * 100):
		t.Errorf("timed out waiting for '%v'", expected)
	}
}
