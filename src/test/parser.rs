use parser;
use filehandler;
use model;
use std::env;

#[test]
fn test_parse_demofile() {
  let mut currentWD = env::current_dir().unwrap().display().to_string();
  currentWD.push_str("/doc/samples/simple.sjs");

  let opts = model::CommandlineOptions {
    filename: currentWD.clone(),
    target_language:model::TargetLanguage::SWIFT,
    target_folder:"".to_string(),
  };

  let mut model = model::GeneralModel {
    options:&opts,
    code:"".to_string(),
  };

  filehandler::read_file(&model.options.filename, &mut model.code);

  let mut p:parser::Parser = parser::Parser::new();

  let result = p.parse(&mut model);

  println!("{:?}", result);

  // Testfaelle einbauen:
  // panic!("TEST");
}

#[test]
fn test_is_whitespace_or_newline() {
  let success = parser::Parser::is_whitespace_or_newline(&' ');
  let error = parser::Parser::is_whitespace_or_newline(&'/');
 
  assert!(success);
  assert!(!error);
}

#[test]
fn test_is_valid_name_character() {
  let success = parser::Parser::is_valid_name_character(&'u');
  let error = parser::Parser::is_valid_name_character(&'{');

  assert!(success);
  assert!(!error);
}
