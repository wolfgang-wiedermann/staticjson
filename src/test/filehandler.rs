use filehandler;
use std::env;

#[test]
fn test_read_file() {
  // Debug-Ausgabe:
  let currentWD = env::current_dir().unwrap();
  println!("Current Directory: {}", currentWD.display());

  let mut content:String = String::new();
  let mut filename:String;
  filename = currentWD.display().to_string();
  filename.push_str("/doc/language_reference.txt");
  filehandler::read_file(&filename, &mut content);
  assert!(content.starts_with("=== "));
}

#[test]
fn test_write_file() {
  let mut content:String = String::new();
  let filename:String;
  filename = "../temp/temp.txt".to_string();
  filehandler::write_file(filename, content);
}
