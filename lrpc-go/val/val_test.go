package val

import (
	"database/sql"
	"errors"
	"math"
	"reflect"
	"testing"
)

type nullStringStore struct{}

func (p nullStringStore) Store(q *ByteQue, val interface{}) error {
	s, o := val.(sql.NullString)
	if !o {
		return errors.New("not a string")
	}
	if s.Valid {
		if e := q.Push(true); e != nil {
			return e
		}
		return q.Push(s.String)
	}
	return q.Push(false)
}
func (p nullStringStore) Restore(q *ByteQue) (interface{}, error) {
	v, e := q.Pop("bool")
	if e != nil {
		return nil, e
	}
	var s sql.NullString
	s.Valid = v.(bool)
	if s.Valid {
		v, e := q.Pop("string")
		if e != nil {
			return nil, e
		}
		s.String = v.(string)
	}
	return s, nil
}

func TestVal(t *testing.T) {
	q := NewByteQue()

	if e := q.Push(uint8(math.MaxUint8)); e != nil {
		t.Error(e)
	}
	if v, e := q.Pop(reflect.Uint8.String()); e != nil || v.(uint8) != math.MaxUint8 {
		t.Error(e)
	}
	if e := q.Push(uint8(0)); e != nil {
		t.Error(e)
	}
	if v, e := q.Pop(reflect.Uint8.String()); e != nil || v.(uint8) != 0 {
		t.Error(e)
	}

	if e := q.Push(int8(math.MaxInt8)); e != nil {
		t.Error(e)
	}
	if v, e := q.Pop(reflect.Int8.String()); e != nil || v.(int8) != math.MaxInt8 {
		t.Error(e)
	}
	if e := q.Push(int8(math.MinInt8)); e != nil {
		t.Error(e)
	}
	if v, e := q.Pop(reflect.Int8.String()); e != nil || v.(int8) != math.MinInt8 {
		t.Error(e)
	}
	if e := q.Push(int8(0)); e != nil {
		t.Error(e)
	}
	if v, e := q.Pop(reflect.Int8.String()); e != nil || v.(int8) != 0 {
		t.Error(e)
	}

	if e := q.Push(uint16(math.MaxUint16)); e != nil {
		t.Error(e)
	}
	if v, e := q.Pop(reflect.Uint16.String()); e != nil || v.(uint16) != math.MaxUint16 {
		t.Error(e)
	}
	if e := q.Push(uint16(0)); e != nil {
		t.Error(e)
	}
	if v, e := q.Pop(reflect.Uint16.String()); e != nil || v.(uint16) != 0 {
		t.Error(e)
	}

	if e := q.Push(int16(math.MaxInt16)); e != nil {
		t.Error(e)
	}
	if v, e := q.Pop(reflect.Int16.String()); e != nil || v.(int16) != math.MaxInt16 {
		t.Error(e)
	}
	if e := q.Push(int16(math.MinInt16)); e != nil {
		t.Error(e)
	}
	if v, e := q.Pop(reflect.Int16.String()); e != nil || v.(int16) != math.MinInt16 {
		t.Error(e)
	}
	if e := q.Push(int16(0)); e != nil {
		t.Error(e)
	}
	if v, e := q.Pop(reflect.Int16.String()); e != nil || v.(int16) != 0 {
		t.Error(e)
	}

	if e := q.Push(uint32(math.MaxUint32)); e != nil {
		t.Error(e)
	}
	if v, e := q.Pop(reflect.Uint32.String()); e != nil || v.(uint32) != math.MaxUint32 {
		t.Error(e)
	}
	if e := q.Push(uint32(0)); e != nil {
		t.Error(e)
	}
	if v, e := q.Pop(reflect.Uint32.String()); e != nil || v.(uint32) != 0 {
		t.Error(e)
	}

	if e := q.Push(int32(math.MaxInt32)); e != nil {
		t.Error(e)
	}
	if v, e := q.Pop(reflect.Int32.String()); e != nil || v.(int32) != math.MaxInt32 {
		t.Error(e)
	}
	if e := q.Push(int32(math.MinInt32)); e != nil {
		t.Error(e)
	}
	if v, e := q.Pop(reflect.Int32.String()); e != nil || v.(int32) != math.MinInt32 {
		t.Error(e)
	}
	if e := q.Push(int32(0)); e != nil {
		t.Error(e)
	}
	if v, e := q.Pop(reflect.Int32.String()); e != nil || v.(int32) != 0 {
		t.Error(e)
	}

	RegistObj([]byte{})
	Regist("sql.NullString", nullStringStore{})
	var v1 sql.NullString
	v1.Valid = true
	v1.String = "hello"
	if e := q.Push(v1); e != nil {
		t.Error(e)
	}
	v2, e := q.Pop("sql.NullString")
	if e != nil {
		t.Error(e)
	}
	if v1 != v2.(sql.NullString) {
		t.Fail()
	}

	type Abc struct {
		A int32
		B string
		C map[string]int32
	}
	RegistObj(make(map[string]int32, 0))
	RegistObj(Abc{})
	a := Abc{
		A: 123,
		B: "hello",
		C: map[string]int32{"a1": 1, "a2": 2, "b3": 3, "b4": 4}}
	if e := q.Push(a); e != nil {
		t.Error(e)
	}
	v, e := q.Pop("val.Abc")
	if e != nil {
		t.Error(e)
	}
	b := v.(Abc)
	if a.A != b.A || a.B != b.B || len(a.C) != len(b.C) {
		t.Fail()
	}
	for k, v := range a.C {
		if b.C[k] != v {
			t.Fail()
		}
	}
}
