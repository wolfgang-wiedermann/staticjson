extern crate getopts;

use model;

/*
* Functions to read and apply the parameters
* from the commandline
*/
pub fn parse_commandline(params:Vec<String>) -> model::CommandlineOptions {
  let mut opt = model::CommandlineOptions {
    filename:"datei.sjs".to_string(), 
    target_language:model::TargetLanguage::SWIFT, 
    target_folder:"".to_string(),
  };

  let program = params[0].clone();
  let mut opts = getopts::Options::new();
  opts.optopt("t", "target", "target language", "TARGET_LANGUAGE");
  opts.optopt("o", "output", "output folder", "OUTPUT_FOLDER");

  let matches = match opts.parse(&params[1..]) {
    Ok(m) => m,
    Err(f) => panic!(f.to_string()),
  };

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

  let languageString = match matches.opt_str("t") {
    Some(x) => format!("{}", x),
    None    => format!("{}", "swift"),
  };
  let languageStr:&str = &languageString;

  let language = match languageStr {
    "swift"      => model::TargetLanguage::SWIFT,
    "c"          => model::TargetLanguage::C,
    "rust"       => model::TargetLanguage::RUST,
    "htmldoc"    => model::TargetLanguage::HTMLDOC,
    "jsvalidate" => model::TargetLanguage::JSVALIDATE,
    _            => panic!("ERROR: Invalid target language"),
  };

  println!("INPUT_FILE_NAME : {}", input);
  println!("OUTPUT_FOLDER   : {}", output);
  println!("TARGET_LANGUAGE : {}", languageStr);

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
