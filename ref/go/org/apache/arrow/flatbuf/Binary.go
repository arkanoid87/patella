// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package flatbuf

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

/// Opaque binary data
type Binary struct {
	_tab flatbuffers.Table
}

func GetRootAsBinary(buf []byte, offset flatbuffers.UOffsetT) *Binary {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &Binary{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsBinary(buf []byte, offset flatbuffers.UOffsetT) *Binary {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &Binary{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *Binary) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *Binary) Table() flatbuffers.Table {
	return rcv._tab
}

func BinaryStart(builder *flatbuffers.Builder) {
	builder.StartObject(0)
}
func BinaryEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}
