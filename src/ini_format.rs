pub mod ini_format {
    /// An INI file consists of a list of entries, each being either a comment or a section.
    pub enum Entry {
        Section(Section),
        Comment(String)
    }

    /// A section has a name and a list of sub-entries.
    pub struct Section {
        pub name: String,
        pub entries: Vec<SubEntry>
    }

    /// A sub-entry is either a comment or a key (name-value pair).
    pub enum SubEntry {
        Comment(String),
        Key(String, String)
    }    
}