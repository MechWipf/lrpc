package tcp

import (
	"errors"
	"lrpc-go/buf"
	"lrpc-go/fun"
	"lrpc-go/val"
	"net"
)

//Service use tcp in the standard library to receive data,
//this is a blocking function
func Service(srvFun *fun.Fun, laddr string) error {
	listener, err := net.Listen("tcp", laddr)
	if err != nil {
		return err
	}
	defer listener.Close()
	for {
		conn, err := listener.Accept()
		if err == nil {
			go func() {
				defer conn.Close()
				buff := make([]byte, 1024)
				for {
					recv := buf.NewRecvBuf()
					for {
						if recv.Size() != nil && *recv.Size() == recv.Len() {
							break
						}
						l, e := conn.Read(buff)
						if e != nil {
							return
						}
						recv.Append(buff[:l])
					}
					sdbf := buf.SendData(srvFun.Invoke(recv.ByteQue()))
					x := 0
					for {
						l, e := conn.Write(sdbf[x:])
						if e != nil {
							return
						}
						if l <= 0 {
							return
						}
						x += l
						if x == len(sdbf) {
							break
						}
					}
				}
			}()
		}
	}
}

//Connection connect to the service host
type Connection struct {
	conn net.Conn
}

//NewConnection create a new Connection
func NewConnection(addr string) (*Connection, error) {
	conn, err := net.Dial("tcp", addr)
	if err != nil {
		return nil, err
	}
	return &Connection{
		conn: conn,
	}, nil
}

//Close close connection
func (c *Connection) Close() {
	c.conn.Close()
}

//Invoke use tcp in the standard library to send data,
//the return result is the original function return value except error
func (c *Connection) Invoke(fun *val.ByteQue, retType ...string) ([]interface{}, error) {
	sdbf := buf.SendData(fun)
	x := 0
	for {
		l, e := c.conn.Write(sdbf[x:])
		if e != nil {
			return nil, e
		}
		if l <= 0 {
			return nil, e
		}
		x += l
		if x == len(sdbf) {
			break
		}
	}
	buff := make([]byte, 1024)
	recv := buf.NewRecvBuf()
	for {
		if recv.Size() != nil && *recv.Size() == recv.Len() {
			break
		}
		l, e := c.conn.Read(buff)
		if e != nil {
			return nil, e
		}
		recv.Append(buff[:l])
	}
	rest := recv.ByteQue()
	v, e := rest.Pop("bool")
	if e != nil {
		return nil, e
	}
	if m := v.(bool); m {
		v, e := rest.Pop("string")
		if e != nil {
			return nil, e
		}
		return nil, errors.New(v.(string))
	}
	retu := make([]interface{}, len(retType))
	for i := 0; i < len(retType); i++ {
		v, e := rest.Pop(retType[i])
		if e != nil {
			return nil, e
		}
		retu[i] = v
	}
	return retu, nil
}
