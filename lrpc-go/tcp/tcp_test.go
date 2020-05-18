package tcp

import (
	"lrpc-go/fun"
	"lrpc-go/val"
	"reflect"
	"testing"
	"time"
)

func plus(x int64, y float64) int64 {
	return x + int64(y)
}

func TestTcp(t *testing.T) {
	fn := fun.NewFun()
	fn.Regist("plus", plus)
	go func() {
		Service(fn, "0.0.0.0:9009")
	}()
	time.Sleep(time.Microsecond)
	con, e := NewConnection("127.0.0.1:9009")
	if e != nil {
		t.Error(e)
	}
	x := int64(1)
	y := 3.14
	q, e := fun.Make("plus", x, y)
	if e != nil {
		t.Error(e)
	}
	v, e := con.Invoke(q, reflect.Int64.String())
	if e != nil || len(v) != 1 {
		t.Error(e)
	}
	r := v[0].(int64)
	if r != 4 {
		t.Error(r)
	}
}

func bubbleSort(arr []int32) {
	for i := 0; i < len(arr)-1; i++ {
		for j := 0; j < len(arr)-1-i; j++ {
			if arr[j] > arr[j+1] {
				temp := arr[j+1]
				arr[j+1] = arr[j]
				arr[j] = temp
			}
		}
	}
}

func BenchmarkTcp1(t *testing.B) {
	var p []int32
	val.RegistObj(p)
	fn := fun.NewFun()
	fn.Regist("bubbleSort", bubbleSort)
	go func() {
		Service(fn, "0.0.0.0:9009")
	}()
	time.Sleep(time.Microsecond)
	arr := make([]int32, 50000)
	for i := 0; i < 50000; i++ {
		arr[i] = int32(i)
	}
	con, e := NewConnection("127.0.0.1:9009")
	if e != nil {
		t.Error(e)
	}
	q, e := fun.Make("bubbleSort", arr)
	if e != nil {
		t.Error(e)
	}
	_, e = con.Invoke(q)
	if e != nil {
		t.Error(e)
	}
}

func BenchmarkTcp2(t *testing.B) {
	arr := make([]int32, 50000)
	for i := 0; i < 50000; i++ {
		arr[i] = int32(i)
	}
	bubbleSort(arr)
}
