use model;
use cmdlinehandler;

#[test]
fn test_parse_commandline() {
  let mut args:Vec<String> = Vec::new();
  args.push("staticjson".to_string());
  args.push("./model.sjs".to_string());
  args.push("-o".to_string());
  args.push("./generated".to_string());
  args.push("-t".to_string());
  args.push("c".to_string());

  let opts = cmdlinehandler::parse_commandline(args);

  assert_eq!("./model.sjs", opts.filename);

  match opts.target_language {
    model::TargetLanguage::C => assert!(true),
    _ => assert!(false),
  }
  assert_eq!("./generated", opts.target_folder);
}
