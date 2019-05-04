use std::env;
use std::fs::File;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Converter {
    in_file:  File,

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

trait OutNames {
    fn filename(&self) -> String;
    fn header_guards(&self) -> Vec<String>;
    fn arrayname(&self) -> String;
    fn add_file_ending(&mut self, fend: String) -> Self;
}

impl OutNames for String {
    fn filename(&self) -> String {
        let mut name_reverse = String::new();

        /* search for directory slashes
         * only want filename when creating new header file
         */
        for c in self.chars().rev() {
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

    fn header_guards(&self) -> Vec<String> {
        let mut ifndef = "#ifndef ".to_owned();
        let name = self.clone().replace(".", "_").to_uppercase();
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
    fn arrayname(&self) -> String {
        "arrayname".to_owned()
    }
    fn add_file_ending(&mut self, fend: String) -> Self {
        self.push_str(&fend);
        self.to_string()
    }
}

impl Converter {
    pub fn new(infile: String) -> Result<Converter, FtaError> {
        let inf = File::open(infile.clone())?;

        let filename = infile.filename();
        let header_guards = filename.clone().add_file_ending(".h".to_string()).header_guards();
        Ok(Converter {
            in_file: inf,
            out_filename: filename.clone().add_file_ending(".h".to_string()),
            out_header_guards: header_guards,
            out_array_name: filename.arrayname(),
        })
    }
}
