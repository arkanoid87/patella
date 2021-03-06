<?php
// automatically generated by the FlatBuffers compiler, do not modify

namespace org\apache\arrow\flatbuf;

use \Google\FlatBuffers\Struct;
use \Google\FlatBuffers\Table;
use \Google\FlatBuffers\ByteBuffer;
use \Google\FlatBuffers\FlatBufferBuilder;

/// Date is either a 32-bit or 64-bit signed integer type representing an
/// elapsed time since UNIX epoch (1970-01-01), stored in either of two units:
///
/// * Milliseconds (64 bits) indicating UNIX time elapsed since the epoch (no
///   leap seconds), where the values are evenly divisible by 86400000
/// * Days (32 bits) since the UNIX epoch
class Date extends Table
{
    /**
     * @param ByteBuffer $bb
     * @return Date
     */
    public static function getRootAsDate(ByteBuffer $bb)
    {
        $obj = new Date();
        return ($obj->init($bb->getInt($bb->getPosition()) + $bb->getPosition(), $bb));
    }

    /**
     * @param int $_i offset
     * @param ByteBuffer $_bb
     * @return Date
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
        return $o != 0 ? $this->bb->getShort($o + $this->bb_pos) : \org\apache\arrow\flatbuf\DateUnit::MILLISECOND;
    }

    /**
     * @param FlatBufferBuilder $builder
     * @return void
     */
    public static function startDate(FlatBufferBuilder $builder)
    {
        $builder->StartObject(1);
    }

    /**
     * @param FlatBufferBuilder $builder
     * @return Date
     */
    public static function createDate(FlatBufferBuilder $builder, $unit)
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
    public static function endDate(FlatBufferBuilder $builder)
    {
        $o = $builder->endObject();
        return $o;
    }
}
