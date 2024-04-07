use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::{self, Write};
use std::path::Path;

use crate::time_util::{jp_date, unix_time_stamp};
use crate::Migrations;

pub fn create_file(filepath: &str, contents: &str) -> io::Result<()> {
    let mut file = File::create(filepath)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn read_file(filepath: &str) -> io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn clean_up_file(path: &str) -> Result<(), Box<dyn Error>> {
    fs::remove_file(path)?;
    Ok(())
}

pub fn create_migration_file() -> Result<(), Box<dyn Error>> {
    // Create Migrations directory if it does not exist
    let dir_path = Path::new("./Migrations");
    if !dir_path.exists() {
        fs::create_dir_all(dir_path).expect("test");
    }

    // Retrieve common timestamp
    let jp_time = jp_date();
    let unix_time = unix_time_stamp();

    // create empty sql up file
    let filepath_up = format!("./Migrations/{}_{}_up.sql", &jp_time, &unix_time);
    if Path::new(&filepath_up).exists() {
        println!("File already exists: {}", filepath_up);
    } else if let Err(e) = create_file(&filepath_up, "") {
        let _ = clean_up_file(&filepath_up);
        return Err(e.into());
    }

    // create empty sql down file
    let filepath_down = format!("./Migrations/{}_{}_down.sql", &jp_time, &unix_time);
    if Path::new(&filepath_down).exists() {
        println!("File already exists: {}", filepath_down);
    } else if let Err(e) = create_file(&filepath_down, "") {
        let _ = clean_up_file(&filepath_up);
        let _ = clean_up_file(&filepath_down);
        return Err(e.into());
    }

    Ok(())
}

pub fn get_all_migration_files(dir: &str, migration_type: Migrations) -> io::Result<Vec<String>> {
    let mut filenames = vec![];

    let entries = fs::read_dir(dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                match migration_type {
                    Migrations::UP => {
                        if name.ends_with("_up.sql") {
                            filenames.push(name.to_string());
                        }
                    }
                    Migrations::DOWN => {
                        if name.ends_with("_down.sql") {
                            filenames.push(name.to_string());
                        }
                    }
                }
            }
        }
    }

    filenames.sort();
    Ok(filenames)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_all_migration_files() {
        let dir = "./test".to_string();
        let filenames = get_all_migration_files(&dir, Migrations::UP).unwrap();
        assert_eq!(
            filenames,
            vec!["test1_up.sql", "test2_up.sql", "test3_up.sql"]
        );
        let filenames = get_all_migration_files(&dir, Migrations::DOWN).unwrap();
        assert_eq!(
            filenames,
            vec!["test1_down.sql", "test2_down.sql", "test3_down.sql"]
        );
    }

    #[test]
    fn test_clean_up_file() {
        let filepath1 = "./Migrations/test.txt";
        let _ = create_file(&filepath1, "");
        assert!(clean_up_file(&filepath1).is_ok());

        let filepath2 = "./Migrations/test1.txt";
        let _ = create_file(&filepath2, "");
        assert!(clean_up_file(&filepath1).is_err());
        let _ = clean_up_file(&filepath2);
    }

    #[test]
    fn test_read_file() {
        let filepath = "./test/test_read_file.txt";

        // cleansing test file.
        if Path::new(filepath).exists() {
            let _ = fs::remove_file(filepath).expect("File not Exists");
        }

        // create test file.
        let contents = "Read test";
        let _ = create_file(filepath, contents);

        assert_eq!(contents, read_file(filepath).unwrap());

        // cleansing test file.
        if Path::new(filepath).exists() {
            let _ = fs::remove_file(filepath).expect("File not Exists");
        }

        // create empty file.
        let contents = "Read test\nHell Word!";
        let _ = create_file(filepath, contents);

        assert_eq!(contents, read_file(filepath).unwrap());

        // cleansing test file.
        if Path::new(filepath).exists() {
            let _ = fs::remove_file(filepath).expect("File not Exists");
        }
    }

    #[test]
    fn test_create_file() {
        let filepath = "./test/test_file.txt";

        // cleansing test file.
        if Path::new(filepath).exists() {
            let _ = fs::remove_file(filepath).expect("File not Exists");
        }

        // test result to create file
        assert!(create_file(filepath, "").is_ok());

        // test create empty file.
        let metadata = fs::metadata(filepath).unwrap();
        assert_eq!(metadata.len(), 0);

        // cleansing test file.
        if Path::new(filepath).exists() {
            let _ = fs::remove_file(filepath).expect("File not Exists");
        }

        // test create
        // test result to create file
        let contents = "Create Test!";
        assert!(create_file(filepath, contents).is_ok());

        // test file is not empty.
        let metadata = fs::metadata(filepath).unwrap();
        assert_ne!(metadata.len(), 0);

        // test file contents.
        assert_eq!(read_file(filepath).unwrap(), contents);

        // cleansing test file.
        if Path::new(filepath).exists() {
            let _ = fs::remove_file(filepath).expect("File not Exists");
        }
    }
}
