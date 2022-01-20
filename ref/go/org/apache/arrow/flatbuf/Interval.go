// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package flatbuf

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type Interval struct {
	_tab flatbuffers.Table
}

func GetRootAsInterval(buf []byte, offset flatbuffers.UOffsetT) *Interval {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &Interval{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsInterval(buf []byte, offset flatbuffers.UOffsetT) *Interval {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &Interval{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *Interval) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *Interval) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *Interval) Unit() IntervalUnit {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return IntervalUnit(rcv._tab.GetInt16(o + rcv._tab.Pos))
	}
	return 0
}

func (rcv *Interval) MutateUnit(n IntervalUnit) bool {
	return rcv._tab.MutateInt16Slot(4, int16(n))
}

func IntervalStart(builder *flatbuffers.Builder) {
	builder.StartObject(1)
}
func IntervalAddUnit(builder *flatbuffers.Builder, unit IntervalUnit) {
	builder.PrependInt16Slot(0, int16(unit), 0)
}
func IntervalEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}