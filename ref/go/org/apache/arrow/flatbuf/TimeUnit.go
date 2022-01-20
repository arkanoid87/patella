// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package flatbuf

import "strconv"

type TimeUnit int16

const (
	TimeUnitSECOND      TimeUnit = 0
	TimeUnitMILLISECOND TimeUnit = 1
	TimeUnitMICROSECOND TimeUnit = 2
	TimeUnitNANOSECOND  TimeUnit = 3
)

var EnumNamesTimeUnit = map[TimeUnit]string{
	TimeUnitSECOND:      "SECOND",
	TimeUnitMILLISECOND: "MILLISECOND",
	TimeUnitMICROSECOND: "MICROSECOND",
	TimeUnitNANOSECOND:  "NANOSECOND",
}

var EnumValuesTimeUnit = map[string]TimeUnit{
	"SECOND":      TimeUnitSECOND,
	"MILLISECOND": TimeUnitMILLISECOND,
	"MICROSECOND": TimeUnitMICROSECOND,
	"NANOSECOND":  TimeUnitNANOSECOND,
}

func (v TimeUnit) String() string {
	if s, ok := EnumNamesTimeUnit[v]; ok {
		return s
	}
	return "TimeUnit(" + strconv.FormatInt(int64(v), 10) + ")"
}
