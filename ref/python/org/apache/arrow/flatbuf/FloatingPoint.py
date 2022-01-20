# automatically generated by the FlatBuffers compiler, do not modify

# namespace: flatbuf

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class FloatingPoint(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = FloatingPoint()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsFloatingPoint(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # FloatingPoint
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # FloatingPoint
    def Precision(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int16Flags, o + self._tab.Pos)
        return 0

def FloatingPointStart(builder): builder.StartObject(1)
def Start(builder):
    return FloatingPointStart(builder)
def FloatingPointAddPrecision(builder, precision): builder.PrependInt16Slot(0, precision, 0)
def AddPrecision(builder, precision):
    return FloatingPointAddPrecision(builder, precision)
def FloatingPointEnd(builder): return builder.EndObject()
def End(builder):
    return FloatingPointEnd(builder)