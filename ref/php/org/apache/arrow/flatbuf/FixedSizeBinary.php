<?php
// automatically generated by the FlatBuffers compiler, do not modify

namespace org\apache\arrow\flatbuf;

use \Google\FlatBuffers\Struct;
use \Google\FlatBuffers\Table;
use \Google\FlatBuffers\ByteBuffer;
use \Google\FlatBuffers\FlatBufferBuilder;

class FixedSizeBinary extends Table
{
    /**
     * @param ByteBuffer $bb
     * @return FixedSizeBinary
     */
    public static function getRootAsFixedSizeBinary(ByteBuffer $bb)
    {
        $obj = new FixedSizeBinary();
        return ($obj->init($bb->getInt($bb->getPosition()) + $bb->getPosition(), $bb));
    }

    /**
     * @param int $_i offset
     * @param ByteBuffer $_bb
     * @return FixedSizeBinary
     **/
    public function init($_i, ByteBuffer $_bb)
    {
        $this->bb_pos = $_i;
        $this->bb = $_bb;
        return $this;
    }

    /// Number of bytes per value
    /**
     * @return int
     */
    public function getByteWidth()
    {
        $o = $this->__offset(4);
        return $o != 0 ? $this->bb->getInt($o + $this->bb_pos) : 0;
    }

    /**
     * @param FlatBufferBuilder $builder
     * @return void
     */
    public static function startFixedSizeBinary(FlatBufferBuilder $builder)
    {
        $builder->StartObject(1);
    }

    /**
     * @param FlatBufferBuilder $builder
     * @return FixedSizeBinary
     */
    public static function createFixedSizeBinary(FlatBufferBuilder $builder, $byteWidth)
    {
        $builder->startObject(1);
        self::addByteWidth($builder, $byteWidth);
        $o = $builder->endObject();
        return $o;
    }

    /**
     * @param FlatBufferBuilder $builder
     * @param int
     * @return void
     */
    public static function addByteWidth(FlatBufferBuilder $builder, $byteWidth)
    {
        $builder->addIntX(0, $byteWidth, 0);
    }

    /**
     * @param FlatBufferBuilder $builder
     * @return int table offset
     */
    public static function endFixedSizeBinary(FlatBufferBuilder $builder)
    {
        $o = $builder->endObject();
        return $o;
    }
}
