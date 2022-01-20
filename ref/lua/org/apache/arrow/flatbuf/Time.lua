--[[ org.apache.arrow.flatbuf.Time

  Automatically generated by the FlatBuffers compiler, do not modify.
  Or modify. I'm a message, not a cop.

  flatc version: 2.0.5

  Declared by  : //Schema.fbs
  Rooting type : org.apache.arrow.flatbuf.Schema (//Schema.fbs)

--]]

local flatbuffers = require('flatbuffers')

-- Time is either a 32-bit or 64-bit signed integer type representing an
-- elapsed time since midnight, stored in either of four units: seconds,
-- milliseconds, microseconds or nanoseconds.
--
-- The integer `bitWidth` depends on the `unit` and must be one of the following:
-- * SECOND and MILLISECOND: 32 bits
-- * MICROSECOND and NANOSECOND: 64 bits
--
-- The allowed values are between 0 (inclusive) and 86400 (=24*60*60) seconds
-- (exclusive), adjusted for the time unit (for example, up to 86400000
-- exclusive for the MILLISECOND unit).
-- This definition doesn't allow for leap seconds. Time values from
-- measurements with leap seconds will need to be corrected when ingesting
-- into Arrow (for example by replacing the value 86400 with 86399).
local Time = {}
local mt = {}

function Time.New()
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

function mt:BitWidth()
  local o = self.view:Offset(6)
  if o ~= 0 then
    return self.view:Get(flatbuffers.N.Int32, self.view.pos + o)
  end
  return 32
end

function Time.Start(builder)
  builder:StartObject(2)
end

function Time.AddUnit(builder, unit)
  builder:PrependInt16Slot(0, unit, 1)
end

function Time.AddBitWidth(builder, bitWidth)
  builder:PrependInt32Slot(1, bitWidth, 32)
end

function Time.End(builder)
  return builder:EndObject()
end

return Time