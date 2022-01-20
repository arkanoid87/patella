// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package flatbuf

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

/// ----------------------------------------------------------------------
/// user defined key value pairs to add custom metadata to arrow
/// key namespacing is the responsibility of the user
type KeyValue struct {
	_tab flatbuffers.Table
}

func GetRootAsKeyValue(buf []byte, offset flatbuffers.UOffsetT) *KeyValue {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &KeyValue{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsKeyValue(buf []byte, offset flatbuffers.UOffsetT) *KeyValue {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &KeyValue{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *KeyValue) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *KeyValue) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *KeyValue) Key() []byte {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return rcv._tab.ByteVector(o + rcv._tab.Pos)
	}
	return nil
}

func (rcv *KeyValue) Value() []byte {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		return rcv._tab.ByteVector(o + rcv._tab.Pos)
	}
	return nil
}

func KeyValueStart(builder *flatbuffers.Builder) {
	builder.StartObject(2)
}
func KeyValueAddKey(builder *flatbuffers.Builder, key flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(0, flatbuffers.UOffsetT(key), 0)
}
func KeyValueAddValue(builder *flatbuffers.Builder, value flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(1, flatbuffers.UOffsetT(value), 0)
}
func KeyValueEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}