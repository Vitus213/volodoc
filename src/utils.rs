
pub fn log_info(message: &str) {
    println!("[INFO] {}", message);
}

pub fn log_error(message: &str) {
    eprintln!("[ERROR] {}", message);
}
pub fn read_idl_file(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("Failed to read IDL file: {}", e))
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::env::temp_dir;

    #[test]
    fn test_read_idl_file_success() {
        let mut path = temp_dir();
        path.push("test_idl_file.idl");
        let test_content = "service Test {}";
        {
            let mut file = File::create(&path).unwrap();
            write!(file, "{}", test_content).unwrap();
        }
        let result = read_idl_file(path.to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_content);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_read_idl_file_not_found() {
        let result = read_idl_file("non_existent_file.idl");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to read IDL file"));
    }
}