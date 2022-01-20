# automatically generated by the FlatBuffers compiler, do not modify

# namespace: flatbuf

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

# ----------------------------------------------------------------------
# user defined key value pairs to add custom metadata to arrow
# key namespacing is the responsibility of the user
class KeyValue(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = KeyValue()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsKeyValue(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # KeyValue
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # KeyValue
    def Key(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # KeyValue
    def Value(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

def KeyValueStart(builder): builder.StartObject(2)
def Start(builder):
    return KeyValueStart(builder)
def KeyValueAddKey(builder, key): builder.PrependUOffsetTRelativeSlot(0, flatbuffers.number_types.UOffsetTFlags.py_type(key), 0)
def AddKey(builder, key):
    return KeyValueAddKey(builder, key)
def KeyValueAddValue(builder, value): builder.PrependUOffsetTRelativeSlot(1, flatbuffers.number_types.UOffsetTFlags.py_type(value), 0)
def AddValue(builder, value):
    return KeyValueAddValue(builder, value)
def KeyValueEnd(builder): return builder.EndObject()
def End(builder):
    return KeyValueEnd(builder)