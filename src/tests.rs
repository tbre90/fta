use std::fs::File;

const TEMP_DIR:            &'static str = "./tempdir/subdir";

const TEST_FILE:           &'static str = "example_file.jpg";
const TEST_FILE_OUT_NAME:  &'static str = "example_file.h";
const TEST_FILE_ARRAYNAME: &'static str = "example_file_h";

const HEADER_GUARD_IFNDEF: &'static str = "#ifndef EXAMPLE_FILE_H";
const HEADER_GUARD_DEFINE: &'static str = "#define EXAMPLE_FILE_H";
const HEADER_GUARD_ENDIF:  &'static str = "#endif";

fn make_temp_file_path(f: String) -> String {
    let mut temp = std::env::temp_dir();
    temp.push(f);
    temp.to_str().unwrap().to_string()
}

#[test]
fn file_opens_correctly() {
    let file_name = make_temp_file_path(TEST_FILE.to_string());
    let _ = File::create(file_name.clone());
    let new_convert = super::Converter::new(file_name);
    assert_eq!(true, {
        let res = 
        match new_convert {
            Ok(_) => true,
            Err(err) => {
                println!("{:?}", err);
                false
            }
        };
        res
    });
}

#[test]
fn filename_generates_correctly() {
    let file_name = make_temp_file_path(TEST_FILE.to_string());
    let _ = File::create(file_name.clone());
    let new_convert = super::Converter::new(file_name);

    assert_eq!(true, {
        let res = 
            match new_convert {
                Ok(cnv) => {
                    if cnv.out_filename == TEST_FILE_OUT_NAME {
                        true
                    } else {
                        println!("Expected '{}', got: {}", TEST_FILE_OUT_NAME, cnv.out_filename);
                        false
                    }
                },
                Err(err) => {
                    println!("{:?}", err);
                    false
                },
            };
        res
    });
}

#[test]
fn filename_with_dir_slashes() {
    let tmp_dir = make_temp_file_path(TEMP_DIR.to_owned());
    let result = std::fs::create_dir_all(tmp_dir);
    match result {
        Ok(_) => (),
        Err(err) => { println!("{}", err); return },
    };

    // if an absolute path is pushed onto a path buffer (e.g. "/tmp/subdir")
    // its data is replaced by the new absolute path, so give it a relative '.'
    let file_name = make_temp_file_path(TEMP_DIR.to_owned() + "/" + TEST_FILE);
    let _ = File::create(file_name.clone());
    let new_convert = super::Converter::new(file_name);

    assert_eq!(true, {
        let res = 
            match new_convert.as_ref() {
                Ok(cnv) => {
                    if cnv.out_filename == TEST_FILE_OUT_NAME {
                        true
                    } else {
                        println!("Expected '{}', got: {}", TEST_FILE_OUT_NAME, cnv.out_filename);
                        false
                    }
                },
                Err(err) => {
                    println!("{:?}", err);
                    false
                },
            };
        res
    });
}

#[test]
fn generate_header_guards_correctly() {
    let file_name = make_temp_file_path(TEST_FILE.to_string());
    let _ = File::create(file_name.clone());
    let new_convert = super::Converter::new(file_name);

    assert_eq!(true, {
        let res =
            match new_convert {
                Ok(cnv) => {
                    if cnv.out_header_guards[0] == HEADER_GUARD_IFNDEF {
                        if cnv.out_header_guards[1] == HEADER_GUARD_DEFINE {
                            if cnv.out_header_guards[2] == HEADER_GUARD_ENDIF {
                                true
                            } else {
                                println!("Expected '{}', got: {}", HEADER_GUARD_ENDIF, cnv.out_header_guards[2]);
                                false
                            }
                        } else {
                            println!("Expected '{}', got: {}", HEADER_GUARD_DEFINE, cnv.out_header_guards[1]);
                            false
                        }
                    } else {
                        println!("Expected '{}', got: {}", HEADER_GUARD_IFNDEF, cnv.out_header_guards[0]);
                        false
                    }
                },
                Err(err) => {
                    println!("{:?}", err);
                    false
                }
            };
        res
    });
}

#[test]
fn generate_array_name_correctly() {
    let file_name = make_temp_file_path(TEST_FILE.to_string());
    let _ = File::create(file_name.clone());
    let new_convert = super::Converter::new(file_name);

    assert_eq!(true, {
        let res =
            match new_convert {
                Ok(cnv) => {
                    if cnv.out_array_name == TEST_FILE_ARRAYNAME {
                        true
                    } else {
                        println!("Expected '{}', got: {}", TEST_FILE_ARRAYNAME, cnv.out_array_name);
                        false
                    }
                },
                Err(err) => {
                    println!("{:?}", err);
                    false
                }
            };
        res
    });
}
