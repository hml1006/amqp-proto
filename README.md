# amqp-proto
## This is a library which defined amqp types and also can encode and decode these types with bytes.

parser in other library

The field value types are same as qpid/rabbitmq, some types conflicts with amqp-0-9-1
```rust
  
            FieldValueKind::Boolean=> b't',
            FieldValueKind::I8 => b'b',
            FieldValueKind::U8 => b'B',
            FieldValueKind::I16 => b's',
            FieldValueKind::U16 => b'u',
            FieldValueKind::I32 => b'I',
            FieldValueKind::U32 => b'i',
            FieldValueKind::I64 => b'l',
            FieldValueKind::U64 => b'L',
            FieldValueKind::F32 => b'f',
            FieldValueKind::F64 => b'd',
            FieldValueKind::Timestamp => b'T',
            FieldValueKind::Decimal => b'D',
            FieldValueKind::LongStr => b'S',
            FieldValueKind::FieldArray => b'A',
            FieldValueKind::FieldTable => b'F',
            FieldValueKind::BytesArray => b'x',
            FieldValueKind::Void => b'V',
            FieldValueKind::Unknown => 0xff
```
# library content
- class definition
- method definition
- basic amqp types
- error definition, include amqp protocol standard response code and frame decode error 
- method frame arguments definition
- content header frame properties definition
- frame codec
