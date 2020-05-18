package val

import (
	"errors"
	"math"
	"reflect"
)

//ByteQue store and restore data queue
type ByteQue struct {
	buff []byte
	head int
	tail int
}

//NewByteQue create a new byteque
func NewByteQue() *ByteQue {
	q := ByteQue{
		buff: make([]byte, 16),
		head: 0,
		tail: 0,
	}
	return &q
}

//Copy copy slice to byteque
func Copy(slice []byte) *ByteQue {
	n := 16
	for n <= len(slice) {
		n <<= 1
	}
	a := make([]byte, n)
	copy(a, slice)
	return &ByteQue{
		buff: a,
		head: 0,
		tail: len(slice),
	}
}

//Len length
func (q *ByteQue) Len() int {
	return (q.tail - q.head) & (len(q.buff) - 1)
}

//CopyTo copy byteque to slice
func (q *ByteQue) CopyTo(dst []byte) {
	switch {
	case q.head < q.tail:
		copy(dst, q.buff[q.head:q.tail])
	case q.head > q.tail:
		copy(dst, q.buff[q.head:])
		copy(dst[q.Len()-q.head:], q.buff[:q.tail])
	}
}

func (q *ByteQue) push(val byte) {
	q.buff[q.tail] = val
	q.tail = (q.tail + 1) & (len(q.buff) - 1)
	if q.tail == q.head {
		a := make([]byte, len(q.buff)<<1)
		copy(a, q.buff[q.head:])
		copy(a[len(q.buff)-q.head:], q.buff[:q.head])
		q.head = 0
		q.tail = len(q.buff)
		q.buff = a
	}
}

func (q *ByteQue) pop() byte {
	if q.head == q.tail {
		return 0
	}
	v := q.buff[q.head]
	q.head = (q.head + 1) & (len(q.buff) - 1)
	return v
}

//PushSize saved data length is not used for the value
func (q *ByteQue) PushSize(val int) {
	for i := 0; i < 10; i++ {
		if val <= 0x7f && val >= 0 {
			q.push((byte)(val & 0x7f))
			break
		} else {
			q.push((byte)(val&0x7f | 0x80))
		}
		val >>= 7
	}
}

//PopSize restore data length is not used for the value
func (q *ByteQue) PopSize() int {
	s := 0
	for i := 0; i < 10; i++ {
		v := q.pop()
		s |= (int(v) & 0x7f) << (7 * i)
		if v <= 0x7f {
			break
		}
	}
	return s
}

//Store rules for storing and restoring data
type Store interface {
	Store(q *ByteQue, val interface{}) error
	Restore(q *ByteQue) (interface{}, error)
}

var stores = make(map[string]Store)

//Regist register a custom data store
func Regist(valType string, store Store) {
	stores[valType] = store
}

var objs = make(map[string]interface{})

//RegistObj register an object for reflection to restore data
func RegistObj(obj interface{}) {
	objs[reflect.TypeOf(obj).String()] = obj
}

//Push to store data to the queue
func (q *ByteQue) Push(val interface{}) error {
	switch v := val.(type) {
	case uint8:
		q.push(v)
	case int8:
		q.push(byte(v))
	case uint16:
		q.push(byte(v))
		q.push(byte(v >> 8))
	case int16:
		q.push(byte(v))
		q.push(byte(v >> 8))
	case uint32:
		q.push(byte(v))
		q.push(byte(v >> 8))
		q.push(byte(v >> 16))
		q.push(byte(v >> 24))
	case int32:
		q.push(byte(v))
		q.push(byte(v >> 8))
		q.push(byte(v >> 16))
		q.push(byte(v >> 24))
	case uint64:
		q.push(byte(v))
		q.push(byte(v >> 8))
		q.push(byte(v >> 16))
		q.push(byte(v >> 24))
		q.push(byte(v >> 32))
		q.push(byte(v >> 40))
		q.push(byte(v >> 48))
		q.push(byte(v >> 56))
	case int64:
		q.push(byte(v))
		q.push(byte(v >> 8))
		q.push(byte(v >> 16))
		q.push(byte(v >> 24))
		q.push(byte(v >> 32))
		q.push(byte(v >> 40))
		q.push(byte(v >> 48))
		q.push(byte(v >> 56))
	case float32:
		t := math.Float32bits(v)
		q.push(byte(t))
		q.push(byte(t >> 8))
		q.push(byte(t >> 16))
		q.push(byte(t >> 24))
	case float64:
		t := math.Float64bits(v)
		q.push(byte(t))
		q.push(byte(t >> 8))
		q.push(byte(t >> 16))
		q.push(byte(t >> 24))
		q.push(byte(t >> 32))
		q.push(byte(t >> 40))
		q.push(byte(t >> 48))
		q.push(byte(t >> 56))
	case complex64:
		t := math.Float32bits(real(v))
		q.push(byte(t))
		q.push(byte(t >> 8))
		q.push(byte(t >> 16))
		q.push(byte(t >> 24))
		t = math.Float32bits(imag(v))
		q.push(byte(t))
		q.push(byte(t >> 8))
		q.push(byte(t >> 16))
		q.push(byte(t >> 24))
	case complex128:
		t := math.Float64bits(real(v))
		q.push(byte(t))
		q.push(byte(t >> 8))
		q.push(byte(t >> 16))
		q.push(byte(t >> 24))
		q.push(byte(t >> 32))
		q.push(byte(t >> 40))
		q.push(byte(t >> 48))
		q.push(byte(t >> 56))
		t = math.Float64bits(imag(v))
		q.push(byte(t))
		q.push(byte(t >> 8))
		q.push(byte(t >> 16))
		q.push(byte(t >> 24))
		q.push(byte(t >> 32))
		q.push(byte(t >> 40))
		q.push(byte(t >> 48))
		q.push(byte(t >> 56))
	case bool:
		if v {
			q.push(1)
		} else {
			q.push(0)
		}
	case string:
		s := []byte(v)
		q.PushSize(len(s))
		for _, c := range s {
			q.push(c)
		}
	default:
		tp := reflect.TypeOf(val).String()
		if st, ok := stores[tp]; ok {
			return st.Store(q, val)
		} else if _, ok := objs[tp]; ok {
			ro := reflect.ValueOf(val)
			if ro.Kind() == reflect.Ptr {
				ro = ro.Elem()
			}
			switch ro.Kind() {
			case reflect.Slice:
				n := ro.Len()
				q.PushSize(n)
				for i := 0; i < n; i++ {
					if e := q.Push(ro.Index(i).Interface()); e != nil {
						return e
					}
				}
			case reflect.Map:
				ks := ro.MapKeys()
				q.PushSize(len(ks))
				for _, k := range ks {
					if e := q.Push(k.Interface()); e != nil {
						return e
					}
					if e := q.Push(ro.MapIndex(k).Interface()); e != nil {
						return e
					}
				}
			case reflect.Struct:
				for i := 0; i < ro.NumField(); i++ {
					if e := q.Push(ro.Field(i).Interface()); e != nil {
						return e
					}
				}
			}
		} else {
			return errors.New(tp + " is not registered so data cannot be stored")
		}
	}
	return nil
}

//Pop to restore data from the queue
func (q *ByteQue) Pop(valType string) (interface{}, error) {
	switch valType {
	case "uint8":
		return q.pop(), nil
	case "int8":
		return int8(q.pop()), nil
	case "uint16":
		return uint16(q.pop()) |
			uint16(q.pop())<<8, nil
	case "int16":
		return int16(q.pop()) |
			int16(q.pop())<<8, nil
	case "uint32":
		return uint32(q.pop()) |
			uint32(q.pop())<<8 |
			uint32(q.pop())<<16 |
			uint32(q.pop())<<24, nil
	case "int32":
		return int32(q.pop()) |
			int32(q.pop())<<8 |
			int32(q.pop())<<16 |
			int32(q.pop())<<24, nil
	case "uint64":
		return uint64(q.pop()) |
			uint64(q.pop())<<8 |
			uint64(q.pop())<<16 |
			uint64(q.pop())<<24 |
			uint64(q.pop())<<32 |
			uint64(q.pop())<<40 |
			uint64(q.pop())<<48 |
			uint64(q.pop())<<56, nil
	case "int64":
		return int64(q.pop()) |
			int64(q.pop())<<8 |
			int64(q.pop())<<16 |
			int64(q.pop())<<24 |
			int64(q.pop())<<32 |
			int64(q.pop())<<40 |
			int64(q.pop())<<48 |
			int64(q.pop())<<56, nil
	case "float32":
		return math.Float32frombits(uint32(q.pop()) |
			uint32(q.pop())<<8 |
			uint32(q.pop())<<16 |
			uint32(q.pop())<<24), nil
	case "float64":
		return math.Float64frombits(uint64(q.pop()) |
			uint64(q.pop())<<8 |
			uint64(q.pop())<<16 |
			uint64(q.pop())<<24 |
			uint64(q.pop())<<32 |
			uint64(q.pop())<<40 |
			uint64(q.pop())<<48 |
			uint64(q.pop())<<56), nil
	case "complex64":
		return complex(
			math.Float32frombits(uint32(q.pop())|
				uint32(q.pop())<<8|
				uint32(q.pop())<<16|
				uint32(q.pop())<<24),
			math.Float32frombits(uint32(q.pop())|
				uint32(q.pop())<<8|
				uint32(q.pop())<<16|
				uint32(q.pop())<<24)), nil
	case "complex128":
		return complex(
			math.Float64frombits(uint64(q.pop())|
				uint64(q.pop())<<8|
				uint64(q.pop())<<16|
				uint64(q.pop())<<24|
				uint64(q.pop())<<32|
				uint64(q.pop())<<40|
				uint64(q.pop())<<48|
				uint64(q.pop())<<56),
			math.Float64frombits(uint64(q.pop())|
				uint64(q.pop())<<8|
				uint64(q.pop())<<16|
				uint64(q.pop())<<24|
				uint64(q.pop())<<32|
				uint64(q.pop())<<40|
				uint64(q.pop())<<48|
				uint64(q.pop())<<56)), nil
	case "bool":
		return q.pop() != 0, nil
	case "string":
		v := make([]byte, q.PopSize())
		for i := 0; i < len(v); i++ {
			v[i] = q.pop()
		}
		return string(v), nil
	default:
		if st, ok := stores[valType]; ok {
			return st.Restore(q)
		}
		if ob, ok := objs[valType]; ok {
			tp := reflect.TypeOf(ob)
			if tp.Kind() == reflect.Ptr {
				tp = tp.Elem()
			}
			switch tp.Kind() {
			case reflect.Slice:
				n := q.PopSize()
				ro := reflect.MakeSlice(tp, 0, n)
				for i := 0; i < n; i++ {
					v, e := q.Pop(tp.Elem().String())
					if e != nil {
						return nil, e
					}
					ro = reflect.Append(ro, reflect.ValueOf(v))
				}
				return ro.Interface(), nil
			case reflect.Map:
				n := q.PopSize()
				ro := reflect.MakeMapWithSize(tp, n)
				for i := 0; i < n; i++ {
					k, e := q.Pop(tp.Key().String())
					if e != nil {
						return nil, e
					}
					v, e := q.Pop(tp.Elem().String())
					if e != nil {
						return nil, e
					}
					ro.SetMapIndex(reflect.ValueOf(k), reflect.ValueOf(v))
				}
				return ro.Interface(), nil
			case reflect.Struct:
				ro := reflect.New(tp)
				if ro.Kind() == reflect.Ptr {
					ro = ro.Elem()
				}
				for i := 0; i < tp.NumField(); i++ {
					v, e := q.Pop(tp.Field(i).Type.String())
					if e != nil {
						return nil, e
					}
					ro.Field(i).Set(reflect.ValueOf(v))
				}
				return ro.Interface(), nil
			}
		}
		return nil, errors.New(valType + " is not registered so data cannot be restored")
	}
}
