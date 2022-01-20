// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package flatbuf

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type FixedSizeBinary struct {
	_tab flatbuffers.Table
}

func GetRootAsFixedSizeBinary(buf []byte, offset flatbuffers.UOffsetT) *FixedSizeBinary {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &FixedSizeBinary{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsFixedSizeBinary(buf []byte, offset flatbuffers.UOffsetT) *FixedSizeBinary {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &FixedSizeBinary{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *FixedSizeBinary) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *FixedSizeBinary) Table() flatbuffers.Table {
	return rcv._tab
}

/// Number of bytes per value
func (rcv *FixedSizeBinary) ByteWidth() int32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return rcv._tab.GetInt32(o + rcv._tab.Pos)
	}
	return 0
}

/// Number of bytes per value
func (rcv *FixedSizeBinary) MutateByteWidth(n int32) bool {
	return rcv._tab.MutateInt32Slot(4, n)
}

func FixedSizeBinaryStart(builder *flatbuffers.Builder) {
	builder.StartObject(1)
}
func FixedSizeBinaryAddByteWidth(builder *flatbuffers.Builder, byteWidth int32) {
	builder.PrependInt32Slot(0, byteWidth, 0)
}
func FixedSizeBinaryEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}