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

  if model.options.debug {
    // Print the parsed results for debugging purposes  
    println!("\nInterfaces\n----------\n");
    println!("{:?}\n", result.interfaces);
    println!("Types\n-----\n");
    println!("{:?}", result.types);
  }

  match opts.target_language {
    model::TargetLanguage::HTMLDOC => staticjson::generator::htmldoc::generate(&result.types, &opts.target_folder),
    model::TargetLanguage::C => staticjson::generator::jsoninc::generate(&result.types, &opts.target_folder),    
    model::TargetLanguage::JAXRS => staticjson::generator::jaxrs::generate(result, &opts.target_folder),
    model::TargetLanguage::JAVACLIENT => staticjson::generator::java_client::generate(result, &opts.target_folder),
    model::TargetLanguage::JQUERY => staticjson::generator::jquery::generate(result, &opts.target_folder),
    model::TargetLanguage::KNOCKOUT => staticjson::generator::knockout::generate(result, &opts.target_folder),
    model::TargetLanguage::DOTNET => staticjson::generator::dotnet::generate(result, &opts.target_folder),
    _ => {
      println!("ERROR: Code generation to target-language not implemented");
    }
  }
}
