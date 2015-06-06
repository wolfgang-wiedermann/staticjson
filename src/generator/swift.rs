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
  if attr.attribute_type == "string" { 
    if attr.is_array == true { 
        str.push_str("\n    case IN_");
        str.push_str(&util::to_upper(&attr.name));
        str.push_str("_ARRAY");
    } 
      str.push_str("\n    case IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_VALUE\n    case IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_STRING");
} else if attr.attribute_type == "date" { 
      str.push_str("\n    case IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_VALUE\n    case IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_DTSTR");
} else { 
      str.push_str("\n    case IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_VALUE");
  }
} 
  str.push_str("\n}\n\n//\n// Parser and serializer utility class\n//\npublic class ");
  str.push_str(&typ.typename);
  str.push_str("Util {\n  private var state:");
  str.push_str(&typ.typename);
  str.push_str("ParserState;\n\n  init() {\n    self.state = ");
  str.push_str(&typ.typename);
  str.push_str("ParserState.INITIAL;\n  }\n\n  //\n  // Parsing-Function for type ");
  str.push_str(&typ.typename);
  str.push_str("\n  //\n  public func parse(json:String) -> ");
  str.push_str(&typ.typename);
  str.push_str(" {\n    var ptr = indices(code).generate();\n    return parse_internal(code, ptr:&ptr);\n  }\n\n  //\n  // Internal parsing function, directly called by same classes parse function\n  // and any other class which has nested objects of this type.\n  // \n  public func parse_internal(code:String, inout ptr:RangeGenerator<String.Index>) -> ");
  str.push_str(&typ.typename);
  str.push_str(" {\n    var obj:");
  str.push_str(&typ.typename);
  str.push_str(" = ");
  str.push_str(&typ.typename);
  str.push_str("();\n    var c:Character = \" \";\n    var charbefore:Character = \" \";\n    var buf = \"\";\n\n    while ptr.startIndex < ptr.endIndex && self.state != ");
  str.push_str(&typ.typename);
  str.push_str("ParserState.FINAL {\n      c = code[ptr.startIndex];\n      switch self.state {\n        case .INITIAL: \n          if c == \"{\" {\n            self.state = ");
  str.push_str(&typ.typename);
  str.push_str("ParserState.INOBJECT;\n          } else if !is_blank(c) {\n            // TODO: Handle syntax error\n          }\n        case .INOBJECT:\n          if c == \"\\\"\" {\n            self.state = ");
  str.push_str(&typ.typename);
  str.push_str("ParserState.IN_FIELDNAME;\n            buf = \"\";\n          } else if !is_blank(c) {\n            // TODO: Handle syntax error\n          }\n        case .IN_FIELDNAME:\n          if c == \"\\\"\" && charbefore != \"\\\\\" {\n            self.state = ");
  str.push_str(&typ.typename);
  str.push_str("ParserState.BEHIND_FIELDNAME;\n          } else {\n            buf.append(c);\n          }\n        case .BEHIND_FIELDNAME:\n          if c == \":\" {\n            if buf == \"\" {\n              // TODO: Handle syntax error, empty names are not allowed");
for attr in typ.attributes.iter() { 
    str.push_str("\n            } else if buf == \"");
    str.push_str(&attr.name);
    str.push_str("\" {\n              self.state = ");
    str.push_str(&typ.typename);
    str.push_str("ParserState.IN_");
    str.push_str(&util::to_upper(&attr.name));
    str.push_str("_VALUE;");
} 
  str.push_str("\n            }\n            // TODO: if Strict-Mode then else with error output\n            // TODO: if flex-Mode then do something to overjump unknown attributes\n            buf = \"\";\n          } else if !is_blank(c) {\n            // TODO: Handle syntax error\n          }\n        default:\n          // This state is not allwoed to be reached\n          println(\"ERROR: ENCOUNTERED INVALID STATE\");\n      }\n      charbefore = c;\n      ptr.next();\n    }\n \n    return obj;\n  }\n\n  //\n  // Function to serialize objects of type ");
  str.push_str(&typ.typename);
  str.push_str("\n  //\n  public func serialize(obj:");
  str.push_str(&typ.typename);
  str.push_str(") -> String {\n    var buf = \"{\";");
for attr in typ.attributes.iter() { 
    str.push_str("\n    buf.append(\"\\\"\");\n    buf.append(\"");
    str.push_str(&attr.name);
    str.push_str("\");\n    buf.append(\"\\\":\");");
  if attr.is_array {  
      str.push_str("\n      buf.append(\"[\");\n      for val in obj.");
      str.push_str(&attr.name);
      str.push_str(" {");
      if attr.attribute_type == "string" { 
        str.push_str("\n        buf.append(\"\\\"\");\n        buf.append(obj.");
        str.push_str(&attr.name);
        str.push_str(");\n        buf.append(\"\\\"\");");
      } else { 
        str.push_str(" \n        buf.append(obj.");
        str.push_str(&attr.name);
        str.push_str(");");
      } 
      str.push_str("       \n      }\n      buf.append(\"]\");");
  } else if attr.attribute_type == "string" {  
      str.push_str("\n    buf.append(\"\\\"\");\n    buf.append(attr.name);\n    buf.append(\"\\\"\");");
  } else { 
      str.push_str("\n    buf.append(obj.");
      str.push_str(&attr.name);
      str.push_str(");");
  } 
    str.push_str("");
} 
  str.push_str("\n    buf.append(\"}\");\n    return buf;\n  }\n\n}\n");
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


