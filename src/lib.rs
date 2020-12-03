use std::fs;

pub fn get_resource(resource: &str) -> String {
    let path = format!("resources/{}", resource);
    if let Ok(file) = fs::read_to_string(path) {
        return file;
    } else {
        panic!(format!("Could not find resource {}", resource));
    }
}
