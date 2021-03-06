// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package flatbuf

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type FixedSizeList struct {
	_tab flatbuffers.Table
}

func GetRootAsFixedSizeList(buf []byte, offset flatbuffers.UOffsetT) *FixedSizeList {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &FixedSizeList{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsFixedSizeList(buf []byte, offset flatbuffers.UOffsetT) *FixedSizeList {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &FixedSizeList{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *FixedSizeList) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *FixedSizeList) Table() flatbuffers.Table {
	return rcv._tab
}

/// Number of list items per value
func (rcv *FixedSizeList) ListSize() int32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return rcv._tab.GetInt32(o + rcv._tab.Pos)
	}
	return 0
}

/// Number of list items per value
func (rcv *FixedSizeList) MutateListSize(n int32) bool {
	return rcv._tab.MutateInt32Slot(4, n)
}

func FixedSizeListStart(builder *flatbuffers.Builder) {
	builder.StartObject(1)
}
func FixedSizeListAddListSize(builder *flatbuffers.Builder, listSize int32) {
	builder.PrependInt32Slot(0, listSize, 0)
}
func FixedSizeListEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}
