pub enum DataType {
    Byte,
    Short,
    Int,
    Long,
    UByte,
    UShort,
    UInt,
    ULong,
    ShortText,
    Text,
    LongText,
    ShortBinary,
    Binary,
    LongBinary,
    DateTime,
    Uuid,
}

pub enum Data {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    UByte(u8),
    UShort(u16),
    UInt(u32),
    ULong(u64),
    ShortText(String),
    Text(String),
    LongText(String),
    ShortBinary(Vec<u8>),
    Binary(Vec<u8>),
    LongBinary(Vec<u8>),
    DateTime(i64),
    Uuid(Vec<u8>),
}

impl DataType {
    pub fn from_code(code: u8) -> Option<DataType> {
        match code {
            1 => Some(DataType::Byte),
            2 => Some(DataType::Short),
            3 => Some(DataType::Int),
            4 => Some(DataType::Long),
            5 => Some(DataType::UByte),
            6 => Some(DataType::UShort),
            7 => Some(DataType::UInt),
            8 => Some(DataType::ULong),
            9 => Some(DataType::ShortText),
            10 => Some(DataType::Text),
            11 => Some(DataType::LongText),
            12 => Some(DataType::ShortBinary),
            13 => Some(DataType::Binary),
            14 => Some(DataType::LongBinary),
            15 => Some(DataType::DateTime),
            16 => Some(DataType::Uuid),
            _ => None,
        }
    }

    pub fn get_code(&self) -> u8 {
        match self {
            DataType::Byte => 1,
            DataType::Short => 2,
            DataType::Int => 3,
            DataType::Long => 4,
            DataType::UByte => 5,
            DataType::UShort => 6,
            DataType::UInt => 7,
            DataType::ULong => 8,
            DataType::ShortText => 9,
            DataType::Text => 10,
            DataType::LongText => 11,
            DataType::ShortBinary => 12,
            DataType::Binary => 13,
            DataType::LongBinary => 14,
            DataType::DateTime => 15,
            DataType::Uuid => 16,
        }
    }

    pub fn get_length(&self) -> usize {
        match self {
            DataType::Byte => 1,
            DataType::Short => 2,
            DataType::Int => 4,
            DataType::Long => 8,
            DataType::UByte => 1,
            DataType::UShort => 2,
            DataType::UInt => 4,
            DataType::ULong => 8,
            DataType::ShortText => u8::MAX as usize,
            DataType::Text => u16::MAX as usize,
            DataType::LongText => u32::MAX as usize,
            DataType::ShortBinary => u8::MAX as usize,
            DataType::Binary => u16::MAX as usize,
            DataType::LongBinary => u32::MAX as usize,
            DataType::DateTime => 8,
            DataType::Uuid => 16,
        }
    }
}

impl Data {
    pub fn serialize(&self) -> Vec<u8> {
        match self {
            Data::Byte(b) => serialize_byte(b),
            Data::Short(s) => serialize_short(s),
            Data::Int(i) => serialize_int(i),
            Data::Long(l) => serialize_long(l),
            Data::UByte(b) => serialize_ubyte(b),
            Data::UShort(s) => serialize_ushort(s),
            Data::UInt(i) => serialize_uint(i),
            Data::ULong(l) => serialize_ulong(l),
            Data::ShortText(s) => serialize_shorttext(s.clone()),
            Data::Text(s) => serialize_text(s.clone()),
            Data::LongText(s) => serialize_longtext(s.clone()),
            Data::ShortBinary(b) => serialize_shortbinary(&mut b.clone()),
            Data::Binary(b) => serialize_binary(&mut b.clone()),
            Data::LongBinary(b) => serialize_longbinary(&mut b.clone()),
            Data::DateTime(dt) => serialize_datetime(dt),
            Data::Uuid(u) => serialize_uuid(&mut u.clone()),
        }
    }

    pub fn deserialize(raw: &[u8], data_type: DataType) -> Result<Data, &'static str> {
        match data_type {
            DataType::Byte => deserialize_byte(raw).map(|r| Data::Byte(r)),
            DataType::Short => deserialize_short(raw).map(|r| Data::Short(r)),
            DataType::Int => deserialize_int(raw).map(|r| Data::Int(r)),
            DataType::Long => deserialize_long(raw).map(|r| Data::Long(r)),
            DataType::UByte => deserialize_ubyte(raw).map(|r| Data::UByte(r)),
            DataType::UShort => deserialize_ushort(raw).map(|r| Data::UShort(r)),
            DataType::UInt => deserialize_uint(raw).map(|r| Data::UInt(r)),
            DataType::ULong => deserialize_ulong(raw).map(|r| Data::ULong(r)),
            DataType::ShortText => deseralize_text(raw).map(|r| Data::ShortText(r)),
            DataType::Text => deseralize_text(raw).map(|r| Data::Text(r)),
            DataType::LongText => deseralize_text(raw).map(|r| Data::LongText(r)),
            DataType::ShortBinary => deserialize_binary(raw).map(|r| Data::ShortBinary(r)),
            DataType::Binary => deserialize_binary(raw).map(|r| Data::Binary(r)),
            DataType::LongBinary => deserialize_binary(raw).map(|r| Data::LongBinary(r)),
            DataType::DateTime => deserialize_long(raw).map(|r| Data::DateTime(r)),
            DataType::Uuid => deserialize_binary(raw).map(|r| Data::Uuid(r)),
        }
    }
}

fn serialize_byte(b: &i8) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::with_capacity(2);
    v.push(1);
    v.extend_from_slice(&b.to_be_bytes());
    v
}

fn serialize_short(s: &i16) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::with_capacity(3);
    v.push(2);
    v.extend_from_slice(&s.to_be_bytes());
    v
}

fn serialize_int(i: &i32) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::with_capacity(5);
    v.push(3);
    v.extend_from_slice(&i.to_be_bytes());
    v
}

fn serialize_long(l: &i64) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::with_capacity(9);
    v.push(4);
    v.extend_from_slice(&l.to_be_bytes());
    v
}

fn serialize_ubyte(b: &u8) -> Vec<u8> {
    vec![5, *b]
}

fn serialize_ushort(s: &u16) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::with_capacity(3);
    v.push(6);
    v.extend_from_slice(&s.to_be_bytes());
    v
}

fn serialize_uint(i: &u32) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::with_capacity(5);
    v.push(7);
    v.extend_from_slice(&i.to_be_bytes());
    v
}

fn serialize_ulong(l: &u64) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::with_capacity(9);
    v.push(8);
    v.extend_from_slice(&l.to_be_bytes());
    v
}

fn serialize_shorttext(s: String) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::with_capacity(256);
    v.push(9);
    v.extend_from_slice(&s.into_bytes());
    v
}

fn serialize_text(s: String) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::with_capacity(u16::MAX as usize + 1);
    v.push(10);
    v.extend_from_slice(&s.into_bytes());
    v
}

fn serialize_longtext(s: String) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::with_capacity(u32::MAX as usize + 1);
    v.push(11);
    v.extend_from_slice(&s.into_bytes());
    v
}

fn serialize_shortbinary(b: &mut Vec<u8>) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::with_capacity(u8::MAX as usize + 1);
    v.push(12);
    v.append(b);
    v
}

fn serialize_binary(b: &mut Vec<u8>) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::with_capacity(u16::MAX as usize + 1);
    v.push(13);
    v.append(b);
    v
}

fn serialize_longbinary(b: &mut Vec<u8>) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::with_capacity(u32::MAX as usize + 1);
    v.push(14);
    v.append(b);
    v
}

fn serialize_datetime(dt: &i64) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::with_capacity(i64::MAX as usize + 1);
    v.push(15);
    v.extend_from_slice(&dt.to_be_bytes());
    v
}

fn serialize_uuid(u: &mut Vec<u8>) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::with_capacity(17);
    v.push(16);
    v.append(u);
    v
}

fn deserialize_byte(raw: &[u8]) -> Result<i8, &'static str> {
    match raw.try_into() as Result<[u8; 1], _> {
        Ok(bytes) => Ok(i8::from_be_bytes(bytes)),
        Err(_) => Err("Could not deseralize."),
    }
}

fn deserialize_short(raw: &[u8]) -> Result<i16, &'static str> {
    match raw.try_into() as Result<[u8; 2], _> {
        Ok(bytes) => Ok(i16::from_be_bytes(bytes)),
        Err(_) => Err("Could not deseralize."),
    }
}

fn deserialize_int(raw: &[u8]) -> Result<i32, &'static str> {
    match raw.try_into() as Result<[u8; 4], _> {
        Ok(bytes) => Ok(i32::from_be_bytes(bytes)),
        Err(_) => Err("Could not deseralize."),
    }
}

fn deserialize_long(raw: &[u8]) -> Result<i64, &'static str> {
    match raw.try_into() as Result<[u8; 8], _> {
        Ok(bytes) => Ok(i64::from_be_bytes(bytes)),
        Err(_) => Err("Could not deseralize."),
    }
}

fn deserialize_ubyte(raw: &[u8]) -> Result<u8, &'static str> {
    match raw.try_into() as Result<[u8; 1], _> {
        Ok(bytes) => Ok(u8::from_be_bytes(bytes)),
        Err(_) => Err("Could not deseralize."),
    }
}

fn deserialize_ushort(raw: &[u8]) -> Result<u16, &'static str> {
    match raw.try_into() as Result<[u8; 2], _> {
        Ok(bytes) => Ok(u16::from_be_bytes(bytes)),
        Err(_) => Err("Could not deseralize."),
    }
}

fn deserialize_uint(raw: &[u8]) -> Result<u32, &'static str> {
    match raw.try_into() as Result<[u8; 4], _> {
        Ok(bytes) => Ok(u32::from_be_bytes(bytes)),
        Err(_) => Err("Could not deseralize."),
    }
}

fn deserialize_ulong(raw: &[u8]) -> Result<u64, &'static str> {
    match raw.try_into() as Result<[u8; 8], _> {
        Ok(bytes) => Ok(u64::from_be_bytes(bytes)),
        Err(_) => Err("Could not deseralize."),
    }
}

fn deseralize_text(raw: &[u8]) -> Result<String, &'static str> {
    match String::from_utf8(raw.to_vec()) {
        Ok(s) => Ok(s),
        Err(_) => Err("Could not serialize string."),
    }
}

fn deserialize_binary(raw: &[u8]) -> Result<Vec<u8>, &'static str> {
    Ok(raw.to_vec())
}
