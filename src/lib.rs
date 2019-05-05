extern crate regex;
#[allow(unused_imports)]
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use regex::Regex;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Converter {
    infile: File,
    infile_size: u64,

    out_filename: String,
    out_header_guards: Vec<String>, // #ifndef ... #define ... #endif
    out_array_name: String,
}

#[derive(Debug, PartialEq)]
pub struct FtaError(pub String);

impl From<std::io::Error> for FtaError {
    fn from(error: std::io::Error) -> Self {
        FtaError{0: error.to_string()}
    }
}

fn filename(s: String) -> String {
    let mut name_reverse = String::new();

    /* search for directory slashes
     * only want filename when creating new header file
     */
    for c in s.chars().rev() {
        if c == '/' || c == '\\' { 
            break;
        }
        if c == '.' { // hit a file extension - don't want to include in header name
            name_reverse.truncate(0);
            continue;
        }
        name_reverse.push(c);
    }

    let mut name_proper = String::new();
    for c in name_reverse.chars().rev() {
        name_proper.push(c);
    }

    name_proper
}

fn header_guards(s: String) -> Vec<String> {
    let mut ifndef = "#ifndef ".to_owned();
    let name = s.clone().replace(".", "_").to_uppercase();
    ifndef.push_str(&name);

    let mut def = "#define ".to_owned();
    def.push_str(&name);

    let endif = "#endif".to_owned();

    let mut guard_vec: Vec<String> = vec!();

    guard_vec.push(ifndef);
    guard_vec.push(def);
    guard_vec.push(endif);
    guard_vec
}

fn arrayname(s: String) -> String {
    let r = Regex::new(r"[[:^alnum:]]").unwrap();
    let result = r.replace_all(&s, "_");
    let mut name = result.to_string();
    name.push_str("_h");
    name
}

fn add_file_ending(s: &mut String, fend: String) -> String {
    s.push_str(".");
    s.push_str(&fend);
    s.to_string()
}

static HEX_CHARS: &'static [&'static str] = &[
    "0", "1", "2", "3",
    "4", "5", "6", "7",
    "8", "9", "A", "B",
    "C", "D", "E", "F",
];

impl Converter {
    pub fn new(infile: String) -> Result<Converter, FtaError> {
        let inf = File::open(infile.clone())?;
        let md  = File::metadata(&inf)?;

        let filename = filename(infile);
        Ok(Converter {
            infile: inf,
            infile_size: md.len(),

            out_filename: add_file_ending(&mut filename.clone(), "h".to_owned()),
            out_header_guards: header_guards(add_file_ending(&mut filename.clone(), "h".to_owned())),
            out_array_name: arrayname(filename),
        })
    }

    pub fn make_header(&self) -> Result<(), FtaError> {
        let mut out_file = File::create(self.out_filename.clone())?;

        let array_start = format!("unsigned char {}[] = {{\n", self.out_array_name);
        let array_finish = "};".to_owned();

        let mut array_content = String::new();
        let array_max_width = 6; // number of columns

        array_content.push_str(&array_start);

        let mut buffer: Vec<u8> = vec!();

        // needed because .take() consumes self
        let fh = self.infile.try_clone()?;
        let mut reader = fh.take(65536);

        let n = reader.read_to_end(&mut buffer)?;

        for (idx, byte) in buffer.iter().enumerate() {
            array_content.push_str("0x");
            array_content.push_str(HEX_CHARS[(byte >> 4) as usize]);
            array_content.push_str(HEX_CHARS[(byte << 4 >> 4) as usize]);

            array_content.push(',');

            if (idx+1) % array_max_width == 0 {
                array_content.push('\n');
            }
        }

        let _ = array_content.pop();
        array_content.push('\n');

        array_content.push_str(&array_finish);

        out_file.write_all(&array_content.into_bytes())?;

        Ok(())
    }
}
