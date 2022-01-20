--[[ org.apache.arrow.flatbuf.Schema

  Automatically generated by the FlatBuffers compiler, do not modify.
  Or modify. I'm a message, not a cop.

  flatc version: 2.0.5

  Declared by  : //Schema.fbs
  Rooting type : org.apache.arrow.flatbuf.Schema (//Schema.fbs)

--]]

local __org_apache_arrow_flatbuf_Field = require('org.apache.arrow.flatbuf.Field')
local __org_apache_arrow_flatbuf_KeyValue = require('org.apache.arrow.flatbuf.KeyValue')
local flatbuffers = require('flatbuffers')

-- ----------------------------------------------------------------------
-- A Schema describes the columns in a row batch
local Schema = {}
local mt = {}

function Schema.New()
  local o = {}
  setmetatable(o, {__index = mt})
  return o
end

function Schema.GetRootAsSchema(buf, offset)
  if type(buf) == "string" then
    buf = flatbuffers.binaryArray.New(buf)
  end

  local n = flatbuffers.N.UOffsetT:Unpack(buf, offset)
  local o = Schema.New()
  o:Init(buf, n + offset)
  return o
end

function mt:Init(buf, pos)
  self.view = flatbuffers.view.New(buf, pos)
end

-- endianness of the buffer
-- it is Little Endian by default
-- if endianness doesn't match the underlying system then the vectors need to be converted
function mt:Endianness()
  local o = self.view:Offset(4)
  if o ~= 0 then
    return self.view:Get(flatbuffers.N.Int16, self.view.pos + o)
  end
  return 0
end

function mt:Fields(j)
  local o = self.view:Offset(6)
  if o ~= 0 then
    local x = self.view:Vector(o)
    x = x + ((j-1) * 4)
    x = self.view:Indirect(x)
    local obj = __org_apache_arrow_flatbuf_Field.New()
    obj:Init(self.view.bytes, x)
    return obj
  end
end

function mt:FieldsLength()
  local o = self.view:Offset(6)
  if o ~= 0 then
    return self.view:VectorLen(o)
  end
  return 0
end

function mt:CustomMetadata(j)
  local o = self.view:Offset(8)
  if o ~= 0 then
    local x = self.view:Vector(o)
    x = x + ((j-1) * 4)
    x = self.view:Indirect(x)
    local obj = __org_apache_arrow_flatbuf_KeyValue.New()
    obj:Init(self.view.bytes, x)
    return obj
  end
end

function mt:CustomMetadataLength()
  local o = self.view:Offset(8)
  if o ~= 0 then
    return self.view:VectorLen(o)
  end
  return 0
end

-- Features used in the stream/file.
function mt:Features(j)
  local o = self.view:Offset(10)
  if o ~= 0 then
    local a = self.view:Vector(o)
    return self.view:Get(flatbuffers.N.Int64, a + ((j-1) * 8))
  end
  return 0
end

function mt:FeaturesLength()
  local o = self.view:Offset(10)
  if o ~= 0 then
    return self.view:VectorLen(o)
  end
  return 0
end

function Schema.Start(builder)
  builder:StartObject(4)
end

function Schema.AddEndianness(builder, endianness)
  builder:PrependInt16Slot(0, endianness, 0)
end

function Schema.AddFields(builder, fields)
  builder:PrependUOffsetTRelativeSlot(1, fields, 0)
end

function Schema.StartFieldsVector(builder, numElems)
  return builder:StartVector(4, numElems, 4)
end

function Schema.AddCustomMetadata(builder, customMetadata)
  builder:PrependUOffsetTRelativeSlot(2, customMetadata, 0)
end

function Schema.StartCustomMetadataVector(builder, numElems)
  return builder:StartVector(4, numElems, 4)
end

function Schema.AddFeatures(builder, features)
  builder:PrependUOffsetTRelativeSlot(3, features, 0)
end

function Schema.StartFeaturesVector(builder, numElems)
  return builder:StartVector(8, numElems, 8)
end

function Schema.End(builder)
  return builder:EndObject()
end

return Schema