package fun

import (
	"testing"
)

func Adder(x, y int32) (int32, error) {
	return x + y, nil
}

func TestFun(t *testing.T) {
	f := NewFun()
	f.Regist("Adder", Adder)
	s, e := Make("Adder", int32(1), int32(2))
	if e != nil {
		t.Error(e)
	}
	r := f.Invoke(s)
	if v, e := r.Pop("bool"); e != nil || v.(bool) {
		t.Error(e)
	}
	if v, e := r.Pop("int32"); e != nil || v.(int32) != 3 {
		t.Error(e)
	}
}
