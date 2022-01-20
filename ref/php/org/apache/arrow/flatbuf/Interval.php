<?php
// automatically generated by the FlatBuffers compiler, do not modify

namespace org\apache\arrow\flatbuf;

use \Google\FlatBuffers\Struct;
use \Google\FlatBuffers\Table;
use \Google\FlatBuffers\ByteBuffer;
use \Google\FlatBuffers\FlatBufferBuilder;

class Interval extends Table
{
    /**
     * @param ByteBuffer $bb
     * @return Interval
     */
    public static function getRootAsInterval(ByteBuffer $bb)
    {
        $obj = new Interval();
        return ($obj->init($bb->getInt($bb->getPosition()) + $bb->getPosition(), $bb));
    }

    /**
     * @param int $_i offset
     * @param ByteBuffer $_bb
     * @return Interval
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
        return $o != 0 ? $this->bb->getShort($o + $this->bb_pos) : \org\apache\arrow\flatbuf\IntervalUnit::YEAR_MONTH;
    }

    /**
     * @param FlatBufferBuilder $builder
     * @return void
     */
    public static function startInterval(FlatBufferBuilder $builder)
    {
        $builder->StartObject(1);
    }

    /**
     * @param FlatBufferBuilder $builder
     * @return Interval
     */
    public static function createInterval(FlatBufferBuilder $builder, $unit)
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
        $builder->addShortX(0, $unit, 0);
    }

    /**
     * @param FlatBufferBuilder $builder
     * @return int table offset
     */
    public static function endInterval(FlatBufferBuilder $builder)
    {
        $o = $builder->endObject();
        return $o;
    }
}
