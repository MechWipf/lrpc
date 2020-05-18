package buf

import "github.com/lipogem/lrpc/lrpc-go/val"

//RecvBuf actual length data
type RecvBuf struct {
	buff []byte
	size *int
}

//NewRecvBuf create a new recvbuf
func NewRecvBuf() *RecvBuf {
	return &RecvBuf{
		buff: nil,
		size: nil,
	}
}

//Append add some data to the buffer until it reaches the specified length
func (r *RecvBuf) Append(other []byte) {
	if r.size == nil {
		if len(r.buff) == 0 {
			for x := 0; x < len(other); x++ {
				if x == 9 || other[x] <= 0x7f {
					s := 0
					for i := 0; i <= x; i++ {
						s |= (int(other[i]) & 0x7f) << (7 * i)
					}
					r.size = &s
					t := other[x+1:]
					if s < len(t) {
						r.buff = append(r.buff, t[:s]...)
					} else {
						r.buff = append(r.buff, t...)
					}
					return
				}
			}
			r.buff = append(r.buff, other...)
		} else {
			r.buff = append(r.buff, other...)
			for x := 0; x < len(r.buff); x++ {
				if x == 9 || r.buff[x] <= 0x7f {
					s := 0
					for i := 0; i <= x; i++ {
						s |= (int(r.buff[i]) & 0x7f) << (7 * i)
					}
					r.buff = r.buff[x+1:]
					r.size = &s
					if len(r.buff) > s {
						r.buff = r.buff[:s]
					}
					break
				}
			}
		}
	} else {
		if *r.size > len(r.buff) {
			l := *r.size - len(r.buff)
			if l < len(other) {
				r.buff = append(r.buff, other[:l]...)
			} else {
				r.buff = append(r.buff, other...)
			}
		}
	}
}

//Size the length of data that should be received
func (r *RecvBuf) Size() *int {
	return r.size
}

//Len length of data received
func (r *RecvBuf) Len() int {
	return len(r.buff)
}

//ByteQue get ByteQue
func (r *RecvBuf) ByteQue() *val.ByteQue {
	return val.Copy(r.buff)
}

//SendData add length to the actual data to judge the integrity of the data
func SendData(q *val.ByteQue) []byte {
	n := q.Len()
	v := make([]byte, n+10)
	i := 0
	for s := n; i < 10; i++ {
		if s <= 0x7f {
			v[i] = byte(s) & 0x7f
			break
		} else {
			v[i] = byte(s)&0x7f | 0x80
		}
		s >>= 7
	}
	q.CopyTo(v[i+1:])
	return v[:i+1+n]
}
