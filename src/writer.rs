/// Filename:    writer.rs
/// Author:      Matt Arriola
/// Description: An INI file writer

pub mod writer {
    use ini_format::ini_format::Entry;
    use ini_format::ini_format::SectionEntry;
    use ini_format::ini_format::Section;

    pub fn write_ini_file (ini_file: Vec<Entry>) {
        for entry in ini_file.iter() {
            match entry {
                &Entry::Section(ref section) => {
                                                    write_section(section);
                                                    println!("");
                                                },
                &Entry::Comment(ref comment) => println!("; {}", comment),
            }
        }
    }

    fn write_section (section: &Section) {
        println!("[{}]", section.name);

        for sub_entry in section.entries.iter() {
            match sub_entry {
                &SectionEntry::Comment(ref comment)     => println!("; {}", comment),
                &SectionEntry::Key(ref name, ref value) => println!("{} = {}", name, value),
            }
        }
    }
}