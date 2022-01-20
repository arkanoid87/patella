--[[ org.apache.arrow.flatbuf.Date

  Automatically generated by the FlatBuffers compiler, do not modify.
  Or modify. I'm a message, not a cop.

  flatc version: 2.0.5

  Declared by  : //Schema.fbs
  Rooting type : org.apache.arrow.flatbuf.Schema (//Schema.fbs)

--]]

local flatbuffers = require('flatbuffers')

-- Date is either a 32-bit or 64-bit signed integer type representing an
-- elapsed time since UNIX epoch (1970-01-01), stored in either of two units:
--
-- * Milliseconds (64 bits) indicating UNIX time elapsed since the epoch (no
--   leap seconds), where the values are evenly divisible by 86400000
-- * Days (32 bits) since the UNIX epoch
local Date = {}
local mt = {}

function Date.New()
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

function Date.Start(builder)
  builder:StartObject(1)
end

function Date.AddUnit(builder, unit)
  builder:PrependInt16Slot(0, unit, 1)
end

function Date.End(builder)
  return builder:EndObject()
end

return Date