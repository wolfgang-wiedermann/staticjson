use model;

type Attribute = model::Attribute;
type Param = model::Parameter;

#[test]
fn test_is_param_present() {
  let mut t = Attribute {
    name:String::new(),
    attribute_type:String::new(),
    is_array:false,
    params:Vec::new(),
  };

  let p = Param {
    name:"param1".to_string(),
    value:"param1val".to_string(),
  };

  t.params.push(Box::new(p));

  assert!(t.is_param_present("param1"));
  assert!(!t.is_param_present("param2"));
}

#[test]
fn test_is_param_value_present() {
  let mut t = Attribute {
    name:String::new(),
    attribute_type:String::new(),
    is_array:false,
    params:Vec::new(),
  };

  let p = Param {
    name:"param1".to_string(),
    value:"param1val".to_string(),
  };

  t.params.push(Box::new(p));

  assert!(t.is_param_value_present("param1", "param1val"));
  assert!(!t.is_param_value_present("param1", "param1value"));
  assert!(!t.is_param_value_present("param2", "param1val"));
}
  
