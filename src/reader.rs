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


    ///////////////////////////////////////////////////////////////////////

    /// Takes a ParseResult. Throws away if no error, otherwise returns the error (propagating it up the call stack)
    macro_rules! verify (
        ($inp:expr) => (
            match $inp {
                ParseResult::Err(e) => { return ParseResult::Err(e); },
                _                   => { },
            }
        );
    )


    ///////////////////////////////////////////////////////////////////////

    pub enum ParseResult {
        Ok,
        Err(String),
    }

    enum ReadResult {
        Ok(String),
        Err(String),
    }

    pub struct IniReader {
        reader: BufferedReader<File>,
        next_char: char,
        ini: IniFile,
        good: bool,
    }


    ///////////////////////////////////////////////////////////////////////

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


        ///////////////////////////////////////////////////////////////////////
        // Parser functions

        pub fn parse (&mut self) -> ParseResult {
            let mut result = ParseResult::Ok;

            self.get_next_char();

            while self.good {
                if self.next_char == ';' {
                    result = self.parse_comment();
                } else {
                    result = self.parse_section();
                }

                verify!(result);
            }

            ParseResult::Ok
        }

        fn parse_comment (&mut self) -> ParseResult {
            verify!(self.match_token(';', "parse_comment"));

            let mut comment = match self.read_rest() {
                ReadResult::Ok(str) => str,
                ReadResult::Err(e)  => return ParseResult::Err(e)
            };
            comment = comment.trim().to_string();
            println!("; {}", comment);

            self.match_token('\n', "parse_comment");
            ParseResult::Ok
        }

        fn parse_section (&mut self) -> ParseResult {
            verify!(self.parse_section_name());

            while self.good && self.next_char != '\n' {
                if (self.next_char == ';') {
                    verify!(self.parse_comment());
                } else {
                    verify!(self.parse_key());
                }
            }
            
            self.match_token('\n', "parse_section");
            ParseResult::Ok
        }

        fn parse_section_name (&mut self) -> ParseResult {
            verify!(self.match_token('[', "parse_section"));

            let mut section_name = String::new();

            while self.good && self.next_char != ']' {
                section_name.push(self.next_char);
                self.get_next_char();
            }

            self.match_token(']', "parse_section");
            println!("[{}]", section_name);

            self.match_token('\n', "parse_section");
            ParseResult::Ok            
        }

        fn parse_key (&mut self) -> ParseResult {
            let mut key_name = String::new();

            while self.good && self.next_char != '=' {
                key_name.push(self.next_char);
                self.get_next_char();
            }

            self.match_token('=', "parse_key");
            let mut key_value = match self.read_rest() {
                ReadResult::Ok(str) => str,
                ReadResult::Err(e)  => return ParseResult::Err(e)
            };
            key_value = key_value.trim().to_string();

            println!("{} = {}", key_name, key_value);

            ParseResult::Ok
        }


        ///////////////////////////////////////////////////////////////////////
        // Helper functions

        fn match_token (&mut self, match_char: char, fun: &str) -> ParseResult {
            if self.next_char != match_char {
                return ParseResult::Err(format!("In {}: expected '{}', got '{}'", fun, match_char, self.next_char));
            }

            self.get_next_char();            
            ParseResult::Ok
        }

        fn get_next_char (&mut self) -> ParseResult {
            self.next_char = ' ';
            while self.next_char == ' ' || self.next_char == '\t' || self.next_char == '\r' {
                self.next_char = match self.reader.read_char() {
                    Ok(c)   => c,
                    Err(e)  => {
                                    self.good = false;
                                    return ParseResult::Err(format!("IO error: {}", e));
                               }
                }
            }

            ParseResult::Ok
        }

        /// Returns the rest of the current line
        fn read_rest (&mut self) -> ReadResult {
            while self.good && self.next_char == ' ' {
                self.get_next_char();
            }

            let line = match self.reader.read_line() { Ok(str) => str, Err(e) => return ReadResult::Err(format!("IO error: {}", e)) };
            let out = format!("{}{}", self.next_char, line);

            self.get_next_char();
            ReadResult::Ok(out)
        }
    }
}