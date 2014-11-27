#![feature(macro_rules)]

use ini_format::ini_format::Entry;
use ini_format::ini_format::SubEntry;
use ini_format::ini_format::Section;

use writer::writer::write_ini_file;
use reader::reader::IniReader;
use reader::reader::ParseResult;

mod ini_format;
mod writer;
mod reader;

fn main () {
    let mut ini_reader = IniReader::new("test.ini".to_string());
    println!("Parse {}", match ini_reader.parse() {
        ParseResult::Ok     => "successful".to_string(),
        ParseResult::Err(e) => format!("failed ({})", e)
    });
}

fn test_print () {
    let mut ini_file = Vec::<Entry>::new();
    ini_file.push(Entry::Comment("This is a root comment".to_string()));

    let mut section1 = Section { name: "Section1".to_string(), entries: Vec::<SubEntry>::new() };
    section1.entries.push(SubEntry::Key("key".to_string(), "value".to_string()));
    section1.entries.push(SubEntry::Comment("This is a section comment".to_string()));
    ini_file.push(Entry::Section(section1));

    write_ini_file(ini_file);
}

