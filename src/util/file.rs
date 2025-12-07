use std::{fs::File, io::Read, path::Path};


pub fn read_file(path: &str) -> String {
    let _path = Path::new(path);

    let mut _file = match File::open(_path) {
        Ok(_file) => _file,
        Err(_) => panic!("could not open {}\n", path),
    };

    let mut _buffer = String::new();
    match _file.read_to_string(&mut _buffer) {
        Ok(_read) => _read,
        Err(_why) => panic!("could not read {}\n, why: {}", path, _why),
    };

    return _buffer;
}
