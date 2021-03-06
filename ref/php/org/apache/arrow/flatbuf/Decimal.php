<?php
// automatically generated by the FlatBuffers compiler, do not modify

namespace org\apache\arrow\flatbuf;

use \Google\FlatBuffers\Struct;
use \Google\FlatBuffers\Table;
use \Google\FlatBuffers\ByteBuffer;
use \Google\FlatBuffers\FlatBufferBuilder;

/// Exact decimal value represented as an integer value in two's
/// complement. Currently only 128-bit (16-byte) and 256-bit (32-byte) integers
/// are used. The representation uses the endianness indicated
/// in the Schema.
class Decimal extends Table
{
    /**
     * @param ByteBuffer $bb
     * @return Decimal
     */
    public static function getRootAsDecimal(ByteBuffer $bb)
    {
        $obj = new Decimal();
        return ($obj->init($bb->getInt($bb->getPosition()) + $bb->getPosition(), $bb));
    }

    /**
     * @param int $_i offset
     * @param ByteBuffer $_bb
     * @return Decimal
     **/
    public function init($_i, ByteBuffer $_bb)
    {
        $this->bb_pos = $_i;
        $this->bb = $_bb;
        return $this;
    }

    /// Total number of decimal digits
    /**
     * @return int
     */
    public function getPrecision()
    {
        $o = $this->__offset(4);
        return $o != 0 ? $this->bb->getInt($o + $this->bb_pos) : 0;
    }

    /// Number of digits after the decimal point "."
    /**
     * @return int
     */
    public function getScale()
    {
        $o = $this->__offset(6);
        return $o != 0 ? $this->bb->getInt($o + $this->bb_pos) : 0;
    }

    /// Number of bits per value. The only accepted widths are 128 and 256.
    /// We use bitWidth for consistency with Int::bitWidth.
    /**
     * @return int
     */
    public function getBitWidth()
    {
        $o = $this->__offset(8);
        return $o != 0 ? $this->bb->getInt($o + $this->bb_pos) : 128;
    }

    /**
     * @param FlatBufferBuilder $builder
     * @return void
     */
    public static function startDecimal(FlatBufferBuilder $builder)
    {
        $builder->StartObject(3);
    }

    /**
     * @param FlatBufferBuilder $builder
     * @return Decimal
     */
    public static function createDecimal(FlatBufferBuilder $builder, $precision, $scale, $bitWidth)
    {
        $builder->startObject(3);
        self::addPrecision($builder, $precision);
        self::addScale($builder, $scale);
        self::addBitWidth($builder, $bitWidth);
        $o = $builder->endObject();
        return $o;
    }

    /**
     * @param FlatBufferBuilder $builder
     * @param int
     * @return void
     */
    public static function addPrecision(FlatBufferBuilder $builder, $precision)
    {
        $builder->addIntX(0, $precision, 0);
    }

    /**
     * @param FlatBufferBuilder $builder
     * @param int
     * @return void
     */
    public static function addScale(FlatBufferBuilder $builder, $scale)
    {
        $builder->addIntX(1, $scale, 0);
    }

    /**
     * @param FlatBufferBuilder $builder
     * @param int
     * @return void
     */
    public static function addBitWidth(FlatBufferBuilder $builder, $bitWidth)
    {
        $builder->addIntX(2, $bitWidth, 128);
    }

    /**
     * @param FlatBufferBuilder $builder
     * @return int table offset
     */
    public static function endDecimal(FlatBufferBuilder $builder)
    {
        $o = $builder->endObject();
        return $o;
    }
}
