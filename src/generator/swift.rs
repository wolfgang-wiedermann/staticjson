// Template for generation of swift based json parsers
use model;
use filehandler;
use util;

// 
// This generat Method is the entry point of code generation
//
pub fn generate(types:&Box<Vec<Box<model::Type>>>, folder:&str) { 
  // generate Classes
  for typ in (*types).iter() {
    let result = gen_type(typ);
    let filename = format!("{}/{}.swift", folder, typ.typename);
    filehandler::write_file(filename, result);
  }
  // generate Parser and Stringifier
}

// Generate single types of the data model
fn gen_type(typ:&Box<model::Type>) -> String {
  let mut str:String = String::new(); 

str.push_str("//\n// ");
str.push_str(&typ.typename);
str.push_str(".swift\n// \n\nimport Foundation\n\n//\n// Datastructure for ");
str.push_str(&typ.typename);
str.push_str("\n//\npublic struct ");
str.push_str(&typ.typename);
str.push_str(" {");
for attr in typ.attributes.iter() { 
str.push_str("\n    public var ");
str.push_str(&attr.name);
str.push_str(":");
str.push_str(&translate_basic_type(&attr.attribute_type));
str.push_str(";");
} 
str.push_str("\n\n    public init() {");
for attr in typ.attributes.iter() { 
str.push_str("\n        self.");
str.push_str(&attr.name);
str.push_str(" = ");
str.push_str(&get_default_value(&attr.attribute_type));
str.push_str(";");
} 
str.push_str("\n    }\n}\n\n//\n// ParserState-Enum for type ");
str.push_str(&typ.typename);
str.push_str("\n//\nenum ");
str.push_str(&typ.typename);
str.push_str("ParserState {\n    case INITIAL\n    case INOBJECT\n    case INFIELDNAME\n    case BEHINDFIELDNAME\n    case BEHINDFIELDVALUE\n    case FINAL");
for attr in typ.attributes.iter() { 
str.push_str("\n    // TODO: Typabhängige Parserstates einbauen\n    case IN_");
str.push_str(&util::to_upper(&attr.name));
str.push_str("_VALUE\n    case IN_");
str.push_str(&util::to_upper(&attr.name));
str.push_str("_STRING");
} 
str.push_str("\n}\n\n//\n// Parsing-Function for type ");
str.push_str(&typ.typename);
str.push_str("\n//\nfunc parse_");
str.push_str(&util::ucamel_to_lsnake(&typ.typename));
str.push_str(" -> ");
str.push_str(&typ.typename);
str.push_str(" {\n    var entity:");
str.push_str(&typ.typename);
str.push_str(" = ");
str.push_str(&typ.typename);
str.push_str("();\n\n    // TODO: continue here with generation of parser code\n\n    return entity;\n} \n");
  return str;
} 

// Translate staticjsons basic types into swift types
fn translate_basic_type(tname:&str) -> String {
  let mut result = String::new();
  if !model::Type::is_basic_type(tname) {
    result.push_str(tname);
  } else if tname == "string" {
    result.push_str("String");
  } else if tname == "int" {
    result.push_str("Int32");
  } else if tname == "uint" {
    result.push_str("UInt32");
  } else if tname == "decimal" {
    result.push_str("decimal");
  } else if tname == "byte" {
    result.push_str("UInt8");
  } else if tname == "char" {
    result.push_str("Character");
  } else if tname == "long" {
    result.push_str("Int64");
  } else if tname == "ulong" {
    result.push_str("UInt64");
  } else if tname == "date" {
    result.push_str("NSDate");
  } else {
    result.push_str("XXXXXXXX");
  }
  return result;
}

// Offer swift initial values for staticjson basic types
fn get_default_value(tname:&str) -> String {
  let mut result = String::new();
  if !model::Type::is_basic_type(tname) {
    result.push_str(tname);
    result.push_str("()");
  } else if tname == "string" {
    result.push_str("\"\"");
  } else if tname == "int" {
    result.push_str("0");
  } else if tname == "uint" {
    result.push_str("0");
  } else if tname == "decimal" {
    result.push_str("0.0");
  } else if tname == "byte" {
    result.push_str("0");
  } else if tname == "char" {
    result.push_str("' '");
  } else if tname == "long" {
    result.push_str("0");
  } else if tname == "ulong" {
    result.push_str("0");
  } else if tname == "date" {
    result.push_str("NSDate()");
  } else {
    result.push_str("XXXXXXXX");
  }
  return result;
}


