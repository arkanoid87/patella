--[[ org.apache.arrow.flatbuf.Buffer

  Automatically generated by the FlatBuffers compiler, do not modify.
  Or modify. I'm a message, not a cop.

  flatc version: 2.0.5

  Declared by  : //Schema.fbs
  Rooting type : org.apache.arrow.flatbuf.Schema (//Schema.fbs)

--]]

local flatbuffers = require('flatbuffers')

-- ----------------------------------------------------------------------
-- A Buffer represents a single contiguous memory segment
local Buffer = {}
local mt = {}

function Buffer.New()
  local o = {}
  setmetatable(o, {__index = mt})
  return o
end

function mt:Init(buf, pos)
  self.view = flatbuffers.view.New(buf, pos)
end

-- The relative offset into the shared memory page where the bytes for this
-- buffer starts
function mt:Offset()
  return self.view:Get(flatbuffers.N.Int64, self.view.pos + 0)
end

-- The absolute length (in bytes) of the memory buffer. The memory is found
-- from offset (inclusive) to offset + length (non-inclusive). When building
-- messages using the encapsulated IPC message, padding bytes may be written
-- after a buffer, but such padding bytes do not need to be accounted for in
-- the size here.
function mt:Length()
  return self.view:Get(flatbuffers.N.Int64, self.view.pos + 8)
end

function Buffer.CreateBuffer(builder, offset, length)
  builder:Prep(8, 16)
  builder:PrependInt64(length)
  builder:PrependInt64(offset)
  return builder:Offset()
end

return Buffer