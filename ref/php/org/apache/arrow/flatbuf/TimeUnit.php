<?php
// automatically generated by the FlatBuffers compiler, do not modify

namespace org\apache\arrow\flatbuf;

class TimeUnit
{
    const SECOND = 0;
    const MILLISECOND = 1;
    const MICROSECOND = 2;
    const NANOSECOND = 3;

    private static $names = array(
        TimeUnit::SECOND=>"SECOND",
        TimeUnit::MILLISECOND=>"MILLISECOND",
        TimeUnit::MICROSECOND=>"MICROSECOND",
        TimeUnit::NANOSECOND=>"NANOSECOND",
    );

    public static function Name($e)
    {
        if (!isset(self::$names[$e])) {
            throw new \Exception();
        }
        return self::$names[$e];
    }
}