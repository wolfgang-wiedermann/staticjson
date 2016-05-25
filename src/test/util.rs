use util;

#[test]
fn test_lcamel_to_lsnake() {
  let base = "customRootSupport";     // lower camel case String
  let target = "custom_root_support"; // lower snake case String

  let result = util::lcamel_to_lsnake(base);
  assert_eq!(result, target);
}

#[test]
fn test_ucamel_to_lsnake() {
  let base = "CustomRootSupport";
  let target = "custom_root_support";

  let result = util::ucamel_to_lsnake(base);
  assert_eq!(result, target);
}

#[test]
fn test_lsnake_to_lcamel() {
  let base = "custom_snake_sample";
  let target = "customSnakeSample";
  
  let result = util::lsnake_to_lcamel(base);
  assert_eq!(result, target);
}

#[test]
fn test_lsnake_to_ucamel() {
  let base = "custom_snake_sample";
  let target = "CustomSnakeSample";

  let result = util::lsnake_to_ucamel(base);
  assert_eq!(result, target);
}

#[test]
fn test_lcamel_to_ucamel() {
  let base = "customCamelSample";
  let target = "CustomCamelSample";

  let result = util::lcamel_to_ucamel(base);
  assert_eq!(result, target);
}

#[test]
fn test_ucamel_to_lcamel() {
  let base = "CustomCamelSample";
  let target = "customCamelSample";

  let result = util::ucamel_to_lcamel(base);
  assert_eq!(result, target);
}

#[test]
fn test_remove_first_char() {
  let base = "/bla/blu/blub/{abc}";
  let target = "bla/blu/blub/{abc}";
  
  let result = util::remove_first_char(base);
  assert_eq!(result, target);
}
