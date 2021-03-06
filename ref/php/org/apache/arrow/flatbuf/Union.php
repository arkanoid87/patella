<?php
// automatically generated by the FlatBuffers compiler, do not modify

namespace org\apache\arrow\flatbuf;

use \Google\FlatBuffers\Struct;
use \Google\FlatBuffers\Table;
use \Google\FlatBuffers\ByteBuffer;
use \Google\FlatBuffers\FlatBufferBuilder;

/// A union is a complex type with children in Field
/// By default ids in the type vector refer to the offsets in the children
/// optionally typeIds provides an indirection between the child offset and the type id
/// for each child `typeIds[offset]` is the id used in the type vector
class Union extends Table
{
    /**
     * @param ByteBuffer $bb
     * @return Union
     */
    public static function getRootAsUnion(ByteBuffer $bb)
    {
        $obj = new Union();
        return ($obj->init($bb->getInt($bb->getPosition()) + $bb->getPosition(), $bb));
    }

    /**
     * @param int $_i offset
     * @param ByteBuffer $_bb
     * @return Union
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
    public function getMode()
    {
        $o = $this->__offset(4);
        return $o != 0 ? $this->bb->getShort($o + $this->bb_pos) : \org\apache\arrow\flatbuf\UnionMode::Sparse;
    }

    /**
     * @param int offset
     * @return int
     */
    public function getTypeIds($j)
    {
        $o = $this->__offset(6);
        return $o != 0 ? $this->bb->getInt($this->__vector($o) + $j * 4) : 0;
    }

    /**
     * @return int
     */
    public function getTypeIdsLength()
    {
        $o = $this->__offset(6);
        return $o != 0 ? $this->__vector_len($o) : 0;
    }

    /**
     * @param FlatBufferBuilder $builder
     * @return void
     */
    public static function startUnion(FlatBufferBuilder $builder)
    {
        $builder->StartObject(2);
    }

    /**
     * @param FlatBufferBuilder $builder
     * @return Union
     */
    public static function createUnion(FlatBufferBuilder $builder, $mode, $typeIds)
    {
        $builder->startObject(2);
        self::addMode($builder, $mode);
        self::addTypeIds($builder, $typeIds);
        $o = $builder->endObject();
        return $o;
    }

    /**
     * @param FlatBufferBuilder $builder
     * @param short
     * @return void
     */
    public static function addMode(FlatBufferBuilder $builder, $mode)
    {
        $builder->addShortX(0, $mode, 0);
    }

    /**
     * @param FlatBufferBuilder $builder
     * @param VectorOffset
     * @return void
     */
    public static function addTypeIds(FlatBufferBuilder $builder, $typeIds)
    {
        $builder->addOffsetX(1, $typeIds, 0);
    }

    /**
     * @param FlatBufferBuilder $builder
     * @param array offset array
     * @return int vector offset
     */
    public static function createTypeIdsVector(FlatBufferBuilder $builder, array $data)
    {
        $builder->startVector(4, count($data), 4);
        for ($i = count($data) - 1; $i >= 0; $i--) {
            $builder->putInt($data[$i]);
        }
        return $builder->endVector();
    }

    /**
     * @param FlatBufferBuilder $builder
     * @param int $numElems
     * @return void
     */
    public static function startTypeIdsVector(FlatBufferBuilder $builder, $numElems)
    {
        $builder->startVector(4, $numElems, 4);
    }

    /**
     * @param FlatBufferBuilder $builder
     * @return int table offset
     */
    public static function endUnion(FlatBufferBuilder $builder)
    {
        $o = $builder->endObject();
        return $o;
    }
}
