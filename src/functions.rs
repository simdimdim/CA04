use std::fs::File;

pub fn read_file(name: String) -> File {
    if let Ok(file) = File::open(&name) {
        file
    } else {
        match File::create(&name) {
            Ok(file2) => file2,
            Err(e) => panic!("{}", e),
        }
    }
}
pub fn from_json(name: String) -> String {
    use std::io::Read;
    let mut contents = String::new();
    let mut file = read_file(name);
    file.read_to_string(&mut contents)
        .expect("Couldn't read json file.");
    contents
}
