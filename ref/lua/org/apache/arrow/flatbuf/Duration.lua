--[[ org.apache.arrow.flatbuf.Duration

  Automatically generated by the FlatBuffers compiler, do not modify.
  Or modify. I'm a message, not a cop.

  flatc version: 2.0.5

  Declared by  : //Schema.fbs
  Rooting type : org.apache.arrow.flatbuf.Schema (//Schema.fbs)

--]]

local flatbuffers = require('flatbuffers')

local Duration = {}
local mt = {}

function Duration.New()
  local o = {}
  setmetatable(o, {__index = mt})
  return o
end

function mt:Init(buf, pos)
  self.view = flatbuffers.view.New(buf, pos)
end

function mt:Unit()
  local o = self.view:Offset(4)
  if o ~= 0 then
    return self.view:Get(flatbuffers.N.Int16, self.view.pos + o)
  end
  return 1
end

function Duration.Start(builder)
  builder:StartObject(1)
end

function Duration.AddUnit(builder, unit)
  builder:PrependInt16Slot(0, unit, 1)
end

function Duration.End(builder)
  return builder:EndObject()
end

return Duration