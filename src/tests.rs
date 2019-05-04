use std::fs::File;

fn make_temp_file_path(f: String) -> String {
    let mut temp = std::env::temp_dir();
    temp.push(f);
    temp.to_str().unwrap().to_string()
}

#[test]
fn file_opens_correctly() {
    let file_name = make_temp_file_path("temptest.txt".to_string());
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
    let file_name = make_temp_file_path("example_file.jpg".to_string());
    let _ = File::create(file_name.clone());
    let new_convert = super::Converter::new(file_name);

    assert_eq!(true, {
        let res = 
            match new_convert {
                Ok(cnv) => {
                    if cnv.out_filename == "example_file.h" {
                        true
                    } else {
                        println!("Expected 'example_file.h', got: {}", cnv.out_filename);
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
    let tmp_dir = make_temp_file_path("./tempdir/subdir".to_owned());
    let result = std::fs::create_dir_all(tmp_dir);
    match result {
        Ok(_) => (),
        Err(err) => { println!("{}", err); return },
    };

    // if an absolute path is pushed onto a path buffer (e.g. "/tmp/subdir")
    // its data is replaced by the new absolute path, so give it a relative '.'
    let file_name = make_temp_file_path("./tempdir/subdir/example_file.bin".to_owned());
    let _ = File::create(file_name.clone());
    let new_convert = super::Converter::new(file_name);

    assert_eq!(true, {
        let res = 
            match new_convert.as_ref() {
                Ok(cnv) => {
                    if cnv.out_filename == "example_file.h" {
                        true
                    } else {
                        println!("Expected 'example_file.h', got: {}", cnv.out_filename);
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
    let file_name = make_temp_file_path("example_file.jpg".to_string());
    let _ = File::create(file_name.clone());
    let new_convert = super::Converter::new(file_name);

    assert_eq!(true, {
        let res =
            match new_convert {
                Ok(cnv) => {
                    if cnv.out_header_guards[0] == "#ifndef EXAMPLE_FILE_H" {
                        if cnv.out_header_guards[1] == "#define EXAMPLE_FILE_H" {
                            if cnv.out_header_guards[2] == "#endif" {
                                true
                            } else {
                                println!("Expected '#endif EXAMPLE_FILE_H', got: {}", cnv.out_header_guards[2]);
                                false
                            }
                        } else {
                            println!("Expected '#define EXAMPLE_FILE_H', got: {}", cnv.out_header_guards[1]);
                            false
                        }
                    } else {
                        println!("Expected '#ifndef EXAMPLE_FILE_H', got: {}", cnv.out_header_guards[0]);
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
