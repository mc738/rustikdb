use crate::core::blocks::Block;
use crate::core::data_types::{Data, DataType};

mod core;

fn print_data(data: Data) {
    match data {
        Data::Byte(v) => println!("Type: Byte. Value: {}", v),
        Data::Short(v) => println!("Type: Short. Value: {}", v),
        Data::Int(v) => println!("Type: Int. Value: {}", v),
        Data::Long(v) => println!("Type: Long. Value: {}", v),
        Data::UByte(v) => println!("Type: UByte. Value: {}", v),
        Data::UShort(v) => println!("Type: UShort. Value: {}", v),
        Data::UInt(v) => println!("Type: UInt. Value: {}", v),
        Data::ULong(v) => println!("Type: ULong. Value: {}", v),
        Data::ShortText(v) => println!("Type: ShortText. Value: {}", v),
        Data::Text(v) => println!("Type: Text. Value: {}", v),
        Data::LongText(v) => println!("Type: LongText. Value: {}", v),
        Data::ShortBinary(v) => println!("Type: ShortBinary. Value: {:?}", v),
        Data::Binary(v) => println!("Type: Binary. Value: {:?}", v),
        Data::LongBinary(v) => println!("Type: LongBinary. Value: {:?}", v),
        Data::DateTime(v) => println!("Type: DateTime. Value: {:?}", v),
        Data::Uuid(v) => println!("Type: Uuid. Value: {:?}", v),
    }
}

fn main() {
    let mut block = Block::create();

    let data = Data::ShortText("Hello, World!".to_string()).serialize();

    match block.try_write(&data) {
        Ok(_) => {
            println!("Data written to block, New tip: {}", block.get_tip());
            // Read data.

            match DataType::from_code(block.raw_read_at(256)) {
                Some(data_type) => {
                    match Data::deserialize(
                        block.raw_read_span(257, data_type.get_length()),
                        data_type,
                    ) {
                        Ok(d) => print_data(d),
                        Err(e) => println!("Error deserializning data: {}", e),
                    }
                }
                None => println!("Unknown data type."),
            }
        }
        Err(e) => println!("Error writing data: {}", e),
    };

    println!("Hello, world!");
}
