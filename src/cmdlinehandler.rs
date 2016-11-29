extern crate getopts;

use model;

/*
* Functions to read and apply the parameters
* from the commandline
*/
pub fn parse_commandline(params:Vec<String>) -> model::CommandlineOptions {
  let mut opt = model::CommandlineOptions {
    filename:"datei.sjs".to_string(), 
    target_language:model::TargetLanguage::JQUERY, 
    target_folder:"".to_string(),
    debug:false,
  };

  //let program = params[0].clone();
  let mut opts = getopts::Options::new();
  opts.optopt("t", "target", "target language", "TARGET_LANGUAGE");
  opts.optopt("o", "output", "output folder", "OUTPUT_FOLDER");
  opts.optflag("d", "debug", "print debug output");

  let matches = match opts.parse(&params[1..]) {
    Ok(m) => m,
    Err(f) => panic!(f.to_string()),
  };

  opt.debug = matches.opt_present("d");

  let input = if !matches.free.is_empty() {
    matches.free[0].clone()
  } else {
    print_usage(opts);
    panic!("no source file specified!");
  };

  let output = match matches.opt_str("o") {
    Some(x) => format!("{}", x),
    None    => format!("{}", "./output"),
  };

  let language_string = match matches.opt_str("t") {
    Some(x) => format!("{}", x),
    None    => format!("{}", "jquery"),
  };
  let language_str:&str = &language_string;

  let language = match language_str {    
    "c"          => model::TargetLanguage::C,
    "rust"       => model::TargetLanguage::RUST,
    "htmldoc"    => model::TargetLanguage::HTMLDOC,
    "jsvalidate" => model::TargetLanguage::JSVALIDATE,
    "jaxrs"      => model::TargetLanguage::JAXRS,
    "javaclient" => model::TargetLanguage::JAVACLIENT,
    "jquery"     => model::TargetLanguage::JQUERY,
    "knockout"   => model::TargetLanguage::KNOCKOUT,
    "dotnet"     => model::TargetLanguage::DOTNET,
    _            => panic!("ERROR: Invalid target language"),
  };

  println!("INPUT_FILE_NAME : {}", input);
  println!("OUTPUT_FOLDER   : {}", output);
  println!("TARGET_LANGUAGE : {}", language_str);
  println!("DEBUG           : {}", opt.debug);

  opt.filename = format!("{}", input);
  opt.target_folder = output;
  opt.target_language = language;

  return opt;
}

/*
* prints the usage instructions for the staticjson commandline tool
*/
fn print_usage(opts: getopts::Options) {
  print!("{}", opts.usage("Usage: staticjson INPUTFILE [options]"));
}
