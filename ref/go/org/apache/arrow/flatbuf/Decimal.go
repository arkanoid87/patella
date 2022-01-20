// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package flatbuf

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

/// Exact decimal value represented as an integer value in two's
/// complement. Currently only 128-bit (16-byte) and 256-bit (32-byte) integers
/// are used. The representation uses the endianness indicated
/// in the Schema.
type Decimal struct {
	_tab flatbuffers.Table
}

func GetRootAsDecimal(buf []byte, offset flatbuffers.UOffsetT) *Decimal {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &Decimal{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsDecimal(buf []byte, offset flatbuffers.UOffsetT) *Decimal {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &Decimal{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *Decimal) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *Decimal) Table() flatbuffers.Table {
	return rcv._tab
}

/// Total number of decimal digits
func (rcv *Decimal) Precision() int32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return rcv._tab.GetInt32(o + rcv._tab.Pos)
	}
	return 0
}

/// Total number of decimal digits
func (rcv *Decimal) MutatePrecision(n int32) bool {
	return rcv._tab.MutateInt32Slot(4, n)
}

/// Number of digits after the decimal point "."
func (rcv *Decimal) Scale() int32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		return rcv._tab.GetInt32(o + rcv._tab.Pos)
	}
	return 0
}

/// Number of digits after the decimal point "."
func (rcv *Decimal) MutateScale(n int32) bool {
	return rcv._tab.MutateInt32Slot(6, n)
}

/// Number of bits per value. The only accepted widths are 128 and 256.
/// We use bitWidth for consistency with Int::bitWidth.
func (rcv *Decimal) BitWidth() int32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(8))
	if o != 0 {
		return rcv._tab.GetInt32(o + rcv._tab.Pos)
	}
	return 128
}

/// Number of bits per value. The only accepted widths are 128 and 256.
/// We use bitWidth for consistency with Int::bitWidth.
func (rcv *Decimal) MutateBitWidth(n int32) bool {
	return rcv._tab.MutateInt32Slot(8, n)
}

func DecimalStart(builder *flatbuffers.Builder) {
	builder.StartObject(3)
}
func DecimalAddPrecision(builder *flatbuffers.Builder, precision int32) {
	builder.PrependInt32Slot(0, precision, 0)
}
func DecimalAddScale(builder *flatbuffers.Builder, scale int32) {
	builder.PrependInt32Slot(1, scale, 0)
}
func DecimalAddBitWidth(builder *flatbuffers.Builder, bitWidth int32) {
	builder.PrependInt32Slot(2, bitWidth, 128)
}
func DecimalEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}
