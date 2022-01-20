--[[ org.apache.arrow.flatbuf.DictionaryEncoding

  Automatically generated by the FlatBuffers compiler, do not modify.
  Or modify. I'm a message, not a cop.

  flatc version: 2.0.5

  Declared by  : //Schema.fbs
  Rooting type : org.apache.arrow.flatbuf.Schema (//Schema.fbs)

--]]

local __org_apache_arrow_flatbuf_Int = require('org.apache.arrow.flatbuf.Int')
local flatbuffers = require('flatbuffers')

local DictionaryEncoding = {}
local mt = {}

function DictionaryEncoding.New()
  local o = {}
  setmetatable(o, {__index = mt})
  return o
end

function mt:Init(buf, pos)
  self.view = flatbuffers.view.New(buf, pos)
end

-- The known dictionary id in the application where this data is used. In
-- the file or streaming formats, the dictionary ids are found in the
-- DictionaryBatch messages
function mt:Id()
  local o = self.view:Offset(4)
  if o ~= 0 then
    return self.view:Get(flatbuffers.N.Int64, self.view.pos + o)
  end
  return 0
end

-- The dictionary indices are constrained to be non-negative integers. If
-- this field is null, the indices must be signed int32. To maximize
-- cross-language compatibility and performance, implementations are
-- recommended to prefer signed integer types over unsigned integer types
-- and to avoid uint64 indices unless they are required by an application.
function mt:IndexType()
  local o = self.view:Offset(6)
  if o ~= 0 then
    local x = self.view:Indirect(self.view.pos + o)
    local obj = __org_apache_arrow_flatbuf_Int.New()
    obj:Init(self.view.bytes, x)
    return obj
  end
end

-- By default, dictionaries are not ordered, or the order does not have
-- semantic meaning. In some statistical, applications, dictionary-encoding
-- is used to represent ordered categorical data, and we provide a way to
-- preserve that metadata here
function mt:IsOrdered()
  local o = self.view:Offset(8)
  if o ~= 0 then
    return (self.view:Get(flatbuffers.N.Bool, self.view.pos + o) ~=0)
  end
  return false
end

function mt:DictionaryKind()
  local o = self.view:Offset(10)
  if o ~= 0 then
    return self.view:Get(flatbuffers.N.Int16, self.view.pos + o)
  end
  return 0
end

function DictionaryEncoding.Start(builder)
  builder:StartObject(4)
end

function DictionaryEncoding.AddId(builder, id)
  builder:PrependInt64Slot(0, id, 0)
end

function DictionaryEncoding.AddIndexType(builder, indexType)
  builder:PrependStructSlot(1, indexType, 0)
end

function DictionaryEncoding.AddIsOrdered(builder, isOrdered)
  builder:PrependBoolSlot(2, isOrdered, false)
end

function DictionaryEncoding.AddDictionaryKind(builder, dictionaryKind)
  builder:PrependInt16Slot(3, dictionaryKind, 0)
end

function DictionaryEncoding.End(builder)
  return builder:EndObject()
end

return DictionaryEncoding