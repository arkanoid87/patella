--[[ org.apache.arrow.flatbuf.Map

  Automatically generated by the FlatBuffers compiler, do not modify.
  Or modify. I'm a message, not a cop.

  flatc version: 2.0.5

  Declared by  : //Schema.fbs
  Rooting type : org.apache.arrow.flatbuf.Schema (//Schema.fbs)

--]]

local flatbuffers = require('flatbuffers')

-- A Map is a logical nested type that is represented as
--
-- List<entries: Struct<key: K, value: V>>
--
-- In this layout, the keys and values are each respectively contiguous. We do
-- not constrain the key and value types, so the application is responsible
-- for ensuring that the keys are hashable and unique. Whether the keys are sorted
-- may be set in the metadata for this field.
--
-- In a field with Map type, the field has a child Struct field, which then
-- has two children: key type and the second the value type. The names of the
-- child fields may be respectively "entries", "key", and "value", but this is
-- not enforced.
--
-- Map
-- ```text
--   - child[0] entries: Struct
--     - child[0] key: K
--     - child[1] value: V
-- ```
-- Neither the "entries" field nor the "key" field may be nullable.
--
-- The metadata is structured so that Arrow systems without special handling
-- for Map can make Map an alias for List. The "layout" attribute for the Map
-- field must have the same contents as a List.
local Map = {}
local mt = {}

function Map.New()
  local o = {}
  setmetatable(o, {__index = mt})
  return o
end

function mt:Init(buf, pos)
  self.view = flatbuffers.view.New(buf, pos)
end

-- Set to true if the keys within each value are sorted
function mt:KeysSorted()
  local o = self.view:Offset(4)
  if o ~= 0 then
    return (self.view:Get(flatbuffers.N.Bool, self.view.pos + o) ~=0)
  end
  return false
end

function Map.Start(builder)
  builder:StartObject(1)
end

function Map.AddKeysSorted(builder, keysSorted)
  builder:PrependBoolSlot(0, keysSorted, false)
end

function Map.End(builder)
  return builder:EndObject()
end

return Map