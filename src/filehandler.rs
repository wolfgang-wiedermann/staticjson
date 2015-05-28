/*
* The module filehandler consists of funtions to write 
* a string to a file and to read a file into a string
*/

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs;

/*
* Reads a file with name @filename into 
* the referenced mutable String @content
*/
pub fn read_file(filename:&String, content: &mut String) {
  let mut success = false;

  let path = Path::new(&filename);
  let pathstr = path.display();

  let mut file = match File::open(&path) {
    Err(why) => panic!("could not open {} : {}", pathstr, Error::description(&why)),
    Ok(file) => file,
  };

  let mut tmpcontent = String::new();

  match file.read_to_string(&mut tmpcontent) {
    Err(why) => panic!("could not read {} : {}", pathstr, Error::description(&why)),
    Ok(file) => {}
  }
  content.push_str(&tmpcontent);
}

/*
* Writes the String @content into a file 
* with the name @filename. It overwrites its
* former content.
*/
pub fn write_file(filename:String, content:String) {
  let path = Path::new(&filename);

  let parent = path.parent().unwrap();
  fs::create_dir_all(parent);

  let f = File::create(&filename);
  let mut file = match f {
    Ok(file) => file, 
    Err(m) => panic!("Datei kann nicht geschrieben werden"),
  };

  file.write_all(content.as_bytes());
}
