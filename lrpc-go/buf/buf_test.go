package buf

import (
	"lrpc-go/val"
	"testing"
)

func TestBuf(t *testing.T) {
	q := val.NewByteQue()
	q.Push(byte(128))
	r := NewRecvBuf()
	s := make([]byte, q.Len())
	q.CopyTo(s)
	r.Append(s)
	if r.Size() != nil {
		t.Fail()
	}

	q = val.NewByteQue()
	q.Push(byte(127))
	r = NewRecvBuf()
	s = make([]byte, q.Len())
	q.CopyTo(s)
	r.Append(s)
	if *r.Size() != 127 {
		t.Fail()
	}

	q = val.NewByteQue()
	q.Push(byte(2))
	q.Push(byte(1))
	q.Push(byte(2))
	q.Push(byte(3))
	q.Push(byte(4))
	q.Push(byte(5))
	r = NewRecvBuf()
	s = make([]byte, q.Len())
	q.CopyTo(s)
	r.Append(s)
	q = r.ByteQue()
	if q.Len() != 2 {
		t.Fail()
	}
	if v, e := q.Pop("uint8"); e != nil || v.(byte) != 1 {
		t.Error(e)
	}
	if v, e := q.Pop("uint8"); e != nil || v.(byte) != 2 {
		t.Error(e)
	}

	z := "hello world!"
	q = val.NewByteQue()
	q.Push(z)
	v := SendData(q)
	if v[0] != 13 || v[1] != 12 {
		t.Fail()
	}
	r = NewRecvBuf()
	r.Append(v)
	q = r.ByteQue()
	if v, e := q.Pop("string"); e != nil || v.(string) != z {
		t.Error(e)
	}
}
