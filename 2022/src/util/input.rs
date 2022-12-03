use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Result as IoResult;

#[allow(dead_code)]
pub(crate) fn get_reader(id: &str) -> IoResult<BufReader<File>> {
    let path = match env::var("INPUT") {
        Ok(val) => val,
        Err(_) => format!("inputs/{}.txt", id),
    };
    Ok(BufReader::new(File::open(path)?))
}
