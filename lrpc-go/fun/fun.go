package fun

import (
	"errors"
	"lrpc-go/val"
	"reflect"
	"strconv"
	"strings"
)

//Fun register call function
type Fun struct {
	funs map[string]reflect.Value
}

//NewFun create a new fun
func NewFun() *Fun {
	f := Fun{
		funs: make(map[string]reflect.Value),
	}
	return &f
}

//Regist register the function being called
func (f *Fun) Regist(name string, fun interface{}) {
	f.funs[name] = reflect.ValueOf(fun)
}

func except(msg string) *val.ByteQue {
	que := val.NewByteQue()
	que.Push(true)
	que.Push(msg)
	return que
}

//Invoke call the registered function
func (f *Fun) Invoke(que *val.ByteQue) *val.ByteQue {
	v, e := que.Pop("string")
	if e != nil {
		return except(e.Error())
	}
	name := v.(string)
	fun, ok := f.funs[name]
	if !ok {
		return except(name + " function not found")
	}
	typ := strings.ReplaceAll(fun.Type().String(), " ", "")
	lix := strings.Index(typ, ")")
	pts := strings.Split(typ[strings.Index(typ, "(")+1:lix], ",")
	pas := make([]reflect.Value, len(pts))
	for i := 0; i < len(pas); i++ {
		if que.Len() == 0 {
			return except("error when calling function " + name + " to restore parameters to the " + strconv.Itoa(i) + "th parameter " + pts[i])
		}
		v, e := que.Pop(pts[i])
		if e != nil {
			return except("error when calling function " + name + " to restore parameters to the " + strconv.Itoa(i) + "th parameter " + pts[i] + ": " + e.Error())
		}
		pas[i] = reflect.ValueOf(v)
	}
	if que.Len() != 0 {
		return except("error when calling function " + name + " to restore parameters")
	}
	ret := val.NewByteQue()
	ret.Push(false)
	rts := strings.ReplaceAll(strings.ReplaceAll(typ[lix+1:], "(", ""), ")", "")
	if len(rts) > 0 {
		rts := strings.Split(rts, ",")
		rss := fun.Call(pas)
		for i := 0; i < len(rts); i++ {
			if rts[i] == "error" {
				if rss[i].Interface() != nil {
					return except(rss[i].Interface().(error).Error())
				}
			} else {
				if rss[i].Interface() == nil {
					return except("the function return value cannot have nil except error")
				}
				if e := ret.Push(rss[i].Interface()); e != nil {
					return except(e.Error() + " when calling function " + name + " to store result " + strconv.Itoa(i))
				}
			}
		}
	} else {
		fun.Call(pas)
	}
	return ret
}

//Make call the method serialized into a queue
func Make(name string, args ...interface{}) (*val.ByteQue, error) {
	que := val.NewByteQue()
	if e := que.Push(name); e != nil {
		return nil, e
	}
	for i := 0; i < len(args); i++ {
		if args[i] == nil {
			return nil, errors.New("function parameters cannot have nil")
		}
		if e := que.Push(args[i]); e != nil {
			return nil, e
		}
	}
	return que, nil
}
