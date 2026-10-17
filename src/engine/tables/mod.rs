//
// TurtKey - Engine Tables Root Module
// Modular export of phonetics, code_tables, and key_mapping
//

pub mod code_tables;
pub mod key_mapping;
pub mod phonetics;

pub use code_tables::get_code_table_entry;
pub use key_mapping::{char_to_key_code, key_code_to_char};
pub use phonetics::*;
