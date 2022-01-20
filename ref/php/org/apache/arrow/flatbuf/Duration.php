<?php
// automatically generated by the FlatBuffers compiler, do not modify

namespace org\apache\arrow\flatbuf;

use \Google\FlatBuffers\Struct;
use \Google\FlatBuffers\Table;
use \Google\FlatBuffers\ByteBuffer;
use \Google\FlatBuffers\FlatBufferBuilder;

class Duration extends Table
{
    /**
     * @param ByteBuffer $bb
     * @return Duration
     */
    public static function getRootAsDuration(ByteBuffer $bb)
    {
        $obj = new Duration();
        return ($obj->init($bb->getInt($bb->getPosition()) + $bb->getPosition(), $bb));
    }

    /**
     * @param int $_i offset
     * @param ByteBuffer $_bb
     * @return Duration
     **/
    public function init($_i, ByteBuffer $_bb)
    {
        $this->bb_pos = $_i;
        $this->bb = $_bb;
        return $this;
    }

    /**
     * @return short
     */
    public function getUnit()
    {
        $o = $this->__offset(4);
        return $o != 0 ? $this->bb->getShort($o + $this->bb_pos) : \org\apache\arrow\flatbuf\TimeUnit::MILLISECOND;
    }

    /**
     * @param FlatBufferBuilder $builder
     * @return void
     */
    public static function startDuration(FlatBufferBuilder $builder)
    {
        $builder->StartObject(1);
    }

    /**
     * @param FlatBufferBuilder $builder
     * @return Duration
     */
    public static function createDuration(FlatBufferBuilder $builder, $unit)
    {
        $builder->startObject(1);
        self::addUnit($builder, $unit);
        $o = $builder->endObject();
        return $o;
    }

    /**
     * @param FlatBufferBuilder $builder
     * @param short
     * @return void
     */
    public static function addUnit(FlatBufferBuilder $builder, $unit)
    {
        $builder->addShortX(0, $unit, 1);
    }

    /**
     * @param FlatBufferBuilder $builder
     * @return int table offset
     */
    public static function endDuration(FlatBufferBuilder $builder)
    {
        $o = $builder->endObject();
        return $o;
    }
}