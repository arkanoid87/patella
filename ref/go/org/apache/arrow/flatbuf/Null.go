// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package flatbuf

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

/// These are stored in the flatbuffer in the Type union below
type Null struct {
	_tab flatbuffers.Table
}

func GetRootAsNull(buf []byte, offset flatbuffers.UOffsetT) *Null {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &Null{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsNull(buf []byte, offset flatbuffers.UOffsetT) *Null {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &Null{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *Null) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *Null) Table() flatbuffers.Table {
	return rcv._tab
}

func NullStart(builder *flatbuffers.Builder) {
	builder.StartObject(0)
}
func NullEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}
