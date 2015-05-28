extern crate staticjson;
extern crate getopts;

use std::env;
use staticjson::model;
#[allow(unused_imports)]
use staticjson::cmdlinehandler;
#[allow(unused_imports)]
use staticjson::filehandler;
#[allow(unused_imports)]
use staticjson::generator::htmldoc;

#[allow(dead_code)]
fn main() {
  let args: Vec<String> = env::args().collect();
  let opts = staticjson::cmdlinehandler::parse_commandline(args);

  let mut model: staticjson::model::GeneralModel = staticjson::model::GeneralModel {
    options:&opts,
    code:"".to_string(),
  };

  staticjson::filehandler::read_file(&model.options.filename, &mut model.code);
  
  let mut p:staticjson::parser::Parser = staticjson::parser::Parser::new();
  let result = p.parse(&mut model);
  
  println!("{:?}", result);

  match opts.target_language {
    model::TargetLanguage::HTMLDOC => staticjson::generator::htmldoc::generate(result, &opts.target_folder),
    model::TargetLanguage::C => staticjson::generator::jsoninc::generate(result, &opts.target_folder),
    model::TargetLanguage::SWIFT => staticjson::generator::swift::generate(result, &opts.target_folder),
    _ => {
      println!("ERROR: Code generation to target-language not implemented");
    }
  }
}
