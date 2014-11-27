/// Filename:    ini_format.rs
/// Author:      Matt Arriola
/// Description: Data structures used to represent the INI file format

pub mod ini_format {
    /// An INI file consists of a list of top-level entries.
    pub type IniFile = Vec<Entry>;

    /// A top-level entry is either a comment or a section.
    pub enum Entry {
        Section(Section),
        Comment(String)
    }

    /// A section has a name and a list of sub-entries.
    pub struct Section {
        pub name: String,
        pub entries: Vec<SectionEntry>
    }

    /// A sub-entry is either a comment or a key (name-value pair).
    pub enum SectionEntry {
        Comment(String),
        Key(String, String)
    }    
}