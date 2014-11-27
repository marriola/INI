/// Filename:    reader.rs
/// Author:      Matt Arriola
/// Description: A recursive descent parser for INI files
///
/// grammar:
///
/// ini          -> comment* section*
/// comment      -> ';' non_newline* '\n'
/// section      -> section_name (comment | key)*
/// section_name -> '[' non_newline+ ']' '\n'
/// key          -> non_newline+ '=' non_newline+ '\n'

pub mod reader {
    use std::char::Char;
    use std::io::fs::File;
    use std::io::BufferedReader;
    use std::path::windows::Path;

    use ini_format::ini_format::IniFile;
    use ini_format::ini_format::Entry;
    use ini_format::ini_format::Section;
    use ini_format::ini_format::SectionEntry;


    //////////////////////////////////////////////////////////////////////////////////////////////
    /// Takes a ParseResult. Throws away if no error, otherwise returns the error (propagating it
    /// up the call stack)

    macro_rules! verify_match (
        ($inp:expr) => (
            match $inp {
                MatchResult::Err(e) => { return ParseResult::Err(e); },
                _                   => { },
            }
        );
    )


    /{95}

    enum IniItem {
        Entry(Entry),
        Section(Section),
        SectionEntry(SectionEntry),
        SectionName(String),
        Key(String, String),
        Comment(String),
    }

    enum MatchResult {
        Ok,
        Err(String),
    }

    pub enum ParseResult {
        Ok,
        StepOk(IniItem),
        Err(String),
    }

    enum ReadResult {
        Ok(String),
        Err(String),
    }

    pub struct IniReader {
        reader: BufferedReader<File>,
        next_char: char,
        pub ini: IniFile,
        good: bool,
    }


    //////////////////////////////////////////////////////////////////////////////////////////////

    impl IniReader {
        pub fn new (filename: String) -> IniReader {
            let mut ini_reader = IniReader {
                reader: BufferedReader::new(match File::open(&Path::new(filename)) {
                        Ok(f)   => f,
                        Err(e)  => panic!("File error: {}", e),
                    }),
                next_char: ' ',
                ini: Vec::<Entry>::new(),
                good: true,
            };

            ini_reader
        }


        //////////////////////////////////////////////////////////////////////////////////////////////
        // Parser functions

        pub fn parse (&mut self) -> ParseResult {
            let mut result;

            self.get_next_char();

            while self.good {
                if self.next_char == ';' {
                    result = self.parse_comment(true);
                } else {
                    result = self.parse_section();
                }

                match result {
                   ParseResult::Err(e) => { return ParseResult::Err(e); },
                   ParseResult::StepOk(item) => match item {
                    IniItem::Entry(entry) => self.ini.push(entry),
                    _ => (),
                   },
                   _ => (),
                }
            }

            ParseResult::Ok
        }

        //////////////////////////////////////////////////////////////////////////////////////////////

        fn parse_comment (&mut self, root: bool) -> ParseResult {
            verify_match!(self.match_token(';', "parse_comment"));

            let mut comment = match self.read_rest() {
                ReadResult::Ok(str) => str,
                ReadResult::Err(e)  => return ParseResult::Err(e)
            };
            comment = comment.trim().to_string();

            if root {
                ParseResult::StepOk(IniItem::Entry(Entry::Comment(comment)))
            } else {
                ParseResult::StepOk(IniItem::SectionEntry(SectionEntry::Comment(comment)))
            }
        }

        //////////////////////////////////////////////////////////////////////////////////////////////

        fn parse_section (&mut self) -> ParseResult {
            let mut result;

            result = self.parse_section_name();
            let mut section_name = String::new();
            match result {
                ParseResult::Err(e) => { return ParseResult::Err(e); },
                ParseResult::StepOk(item) => match item {
                    IniItem::SectionName(name) => section_name = name,
                    _ => (),
                },
                _ => (),
            };

            let mut section = Section { name: section_name, entries: Vec::<SectionEntry>::new() };

            while self.good {
                if self.next_char == ';' {
                    result = self.parse_comment(false);
                } else if self.next_char == '[' {
                    break;
                } else {
                    result = self.parse_key();
                }

                match result {
                    ParseResult::Err(e) => return ParseResult::Err(e),
                    ParseResult::StepOk(item) => match item {
                        IniItem::Comment(comment) =>
                            section.entries.push(SectionEntry::Comment(comment)),
                        IniItem::Key(name, value) =>
                            section.entries.push(SectionEntry::Key(name, value)),
                        _ => (),
                    },
                    _ => (),
                };
            }
            
            ParseResult::StepOk(IniItem::Entry(Entry::Section(section)))
        }

        //////////////////////////////////////////////////////////////////////////////////////////////

        fn parse_section_name (&mut self) -> ParseResult {
            verify_match!(self.match_token('[', "parse_section"));

            let mut section_name = String::new();

            while self.good && self.next_char != ']' {
                section_name.push(self.next_char);
                self.get_next_char();
            }

            verify_match!(self.match_token(']', "parse_section"));
            ParseResult::StepOk(IniItem::SectionName(section_name))
        }

        //////////////////////////////////////////////////////////////////////////////////////////////

        fn parse_key (&mut self) -> ParseResult {
            let mut key_name = String::new();

            while self.good && self.next_char != '=' {
                key_name.push(self.next_char);
                self.get_next_char();
            }

            verify_match!(self.match_token('=', "parse_key"));
            let mut key_value = match self.read_rest() {
                ReadResult::Ok(str) => str,
                ReadResult::Err(e)  => return ParseResult::Err(e)
            };
            key_value = key_value.trim().to_string();

            ParseResult::StepOk(IniItem::Key(key_name, key_value))
        }


        //////////////////////////////////////////////////////////////////////////////////////////////
        // Helper functions

        fn match_token (&mut self, match_char: char, fun: &str) -> MatchResult {
            if self.next_char != match_char {
                return MatchResult::Err(format!("In {}: expected '{}', got '{}'", fun,
                                                match_char, self.next_char));
            }

            self.get_next_char();            
            MatchResult::Ok
        }

        fn get_next_char (&mut self) {
            self.next_char = ' ';
            while self.next_char.is_whitespace() {
                self.next_char = match self.reader.read_char() {
                    Ok(c)   => c,
                    Err(e)  => {
                                    self.good = false;
                                    return;
                               }
                }
            }
        }

        /// Returns the rest of the current line
        fn read_rest (&mut self) -> ReadResult {
            while self.good && self.next_char.is_whitespace() {
                self.get_next_char();
            }

            let line = match self.reader.read_line() {
                    Ok(str) => str,
                    Err(e) => return ReadResult::Err(format!("IO error: {}", e))
            };
            let out = format!("{}{}", self.next_char, line);

            self.get_next_char();
            ReadResult::Ok(out)
        }
    }
}