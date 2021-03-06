--[[ org.apache.arrow.flatbuf.KeyValue

  Automatically generated by the FlatBuffers compiler, do not modify.
  Or modify. I'm a message, not a cop.

  flatc version: 2.0.5

  Declared by  : //Schema.fbs
  Rooting type : org.apache.arrow.flatbuf.Schema (//Schema.fbs)

--]]

local flatbuffers = require('flatbuffers')

-- ----------------------------------------------------------------------
-- user defined key value pairs to add custom metadata to arrow
-- key namespacing is the responsibility of the user
local KeyValue = {}
local mt = {}

function KeyValue.New()
  local o = {}
  setmetatable(o, {__index = mt})
  return o
end

function mt:Init(buf, pos)
  self.view = flatbuffers.view.New(buf, pos)
end

function mt:Key()
  local o = self.view:Offset(4)
  if o ~= 0 then
    return self.view:String(self.view.pos + o)
  end
end

function mt:Value()
  local o = self.view:Offset(6)
  if o ~= 0 then
    return self.view:String(self.view.pos + o)
  end
end

function KeyValue.Start(builder)
  builder:StartObject(2)
end

function KeyValue.AddKey(builder, key)
  builder:PrependUOffsetTRelativeSlot(0, key, 0)
end

function KeyValue.AddValue(builder, value)
  builder:PrependUOffsetTRelativeSlot(1, value, 0)
end

function KeyValue.End(builder)
  return builder:EndObject()
end

return KeyValue