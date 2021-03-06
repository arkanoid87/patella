<?php
// automatically generated by the FlatBuffers compiler, do not modify

namespace org\apache\arrow\flatbuf;

use \Google\FlatBuffers\Struct;
use \Google\FlatBuffers\Table;
use \Google\FlatBuffers\ByteBuffer;
use \Google\FlatBuffers\FlatBufferBuilder;

/// A Map is a logical nested type that is represented as
///
/// List<entries: Struct<key: K, value: V>>
///
/// In this layout, the keys and values are each respectively contiguous. We do
/// not constrain the key and value types, so the application is responsible
/// for ensuring that the keys are hashable and unique. Whether the keys are sorted
/// may be set in the metadata for this field.
///
/// In a field with Map type, the field has a child Struct field, which then
/// has two children: key type and the second the value type. The names of the
/// child fields may be respectively "entries", "key", and "value", but this is
/// not enforced.
///
/// Map
/// ```text
///   - child[0] entries: Struct
///     - child[0] key: K
///     - child[1] value: V
/// ```
/// Neither the "entries" field nor the "key" field may be nullable.
///
/// The metadata is structured so that Arrow systems without special handling
/// for Map can make Map an alias for List. The "layout" attribute for the Map
/// field must have the same contents as a List.
class Map extends Table
{
    /**
     * @param ByteBuffer $bb
     * @return Map
     */
    public static function getRootAsMap(ByteBuffer $bb)
    {
        $obj = new Map();
        return ($obj->init($bb->getInt($bb->getPosition()) + $bb->getPosition(), $bb));
    }

    /**
     * @param int $_i offset
     * @param ByteBuffer $_bb
     * @return Map
     **/
    public function init($_i, ByteBuffer $_bb)
    {
        $this->bb_pos = $_i;
        $this->bb = $_bb;
        return $this;
    }

    /// Set to true if the keys within each value are sorted
    /**
     * @return bool
     */
    public function getKeysSorted()
    {
        $o = $this->__offset(4);
        return $o != 0 ? $this->bb->getBool($o + $this->bb_pos) : false;
    }

    /**
     * @param FlatBufferBuilder $builder
     * @return void
     */
    public static function startMap(FlatBufferBuilder $builder)
    {
        $builder->StartObject(1);
    }

    /**
     * @param FlatBufferBuilder $builder
     * @return Map
     */
    public static function createMap(FlatBufferBuilder $builder, $keysSorted)
    {
        $builder->startObject(1);
        self::addKeysSorted($builder, $keysSorted);
        $o = $builder->endObject();
        return $o;
    }

    /**
     * @param FlatBufferBuilder $builder
     * @param bool
     * @return void
     */
    public static function addKeysSorted(FlatBufferBuilder $builder, $keysSorted)
    {
        $builder->addBoolX(0, $keysSorted, false);
    }

    /**
     * @param FlatBufferBuilder $builder
     * @return int table offset
     */
    public static function endMap(FlatBufferBuilder $builder)
    {
        $o = $builder->endObject();
        return $o;
    }
}
