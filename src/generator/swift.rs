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
  str.push_str("\n    }\n\n    //\n    // ParserState-Enum for type ");
  str.push_str(&typ.typename);
  str.push_str("\n    //\n    private enum ");
  str.push_str(&typ.typename);
  str.push_str("ParserState {\n        case INITIAL\n        case INOBJECT\n        case IN_FIELDNAME\n        case BEHIND_FIELDNAME\n        case BEHIND_FIELDVALUE\n        case FINAL");
for attr in typ.attributes.iter() {
  if attr.attribute_type == "string" { 
    if attr.is_array == true { 
        str.push_str("\n        case IN_");
        str.push_str(&util::to_upper(&attr.name));
        str.push_str("_ARRAY");
    } 
      str.push_str("\n        case IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_VALUE\n        case IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_STRING");
} else if attr.attribute_type == "date" { 
      str.push_str("\n        case IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_VALUE\n        case IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_STRING");
} else if !model::Type::is_basic_type(&attr.attribute_type) { 
      str.push_str("\n        case IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_OBJECT");
} else { 
      str.push_str("\n        case IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_VALUE");
  }
} 
  str.push_str("\n    }\n\n  //\n  // Parsing-Function for type ");
  str.push_str(&typ.typename);
  str.push_str("\n  //\n  public static func parse(code:String) -> ");
  str.push_str(&typ.typename);
  str.push_str(" {\n    var ptr = indices(code).generate();\n    return parse_internal(code, ptr:&ptr);\n  }\n\n  //\n  // Internal parsing function, directly called by same classes parse function\n  // and any other class which has nested objects of this type.\n  // \n  public static func parse_internal(code:String, inout ptr:RangeGenerator<String.Index>) -> ");
  str.push_str(&typ.typename);
  str.push_str(" {\n    var obj:");
  str.push_str(&typ.typename);
  str.push_str(" = ");
  str.push_str(&typ.typename);
  str.push_str("();\n    var c:Character = \" \";\n    var charbefore:Character = \" \";\n    var buf = \"\";\n    var state = ");
  str.push_str(&typ.typename);
  str.push_str("ParserState.INITIAL;\n\n    while ptr.startIndex < ptr.endIndex && state != ");
  str.push_str(&typ.typename);
  str.push_str("ParserState.FINAL {\n      c = code[ptr.startIndex];\n      switch state {\n        case .INITIAL: \n          if c == \"{\" {\n            state = ");
  str.push_str(&typ.typename);
  str.push_str("ParserState.INOBJECT;\n          } else if !is_blank(c) {\n            // TODO: Handle syntax error\n          }\n        case .INOBJECT:\n          if c == \"\\\"\" {\n            state = ");
  str.push_str(&typ.typename);
  str.push_str("ParserState.IN_FIELDNAME;\n            buf = \"\";\n          } else if !is_blank(c) {\n            // TODO: Handle syntax error\n          }\n        case .IN_FIELDNAME:\n          if c == \"\\\"\" && charbefore != \"\\\\\" {\n            state = ");
  str.push_str(&typ.typename);
  str.push_str("ParserState.BEHIND_FIELDNAME;\n          } else {\n            buf.append(c);\n          }\n        case .BEHIND_FIELDNAME:\n          if c == \":\" {\n            if buf == \"\" {\n              // TODO: Handle syntax error, empty names are not allowed");
for attr in typ.attributes.iter() { 
    str.push_str("\n            } else if buf == \"");
    str.push_str(&attr.name);
    str.push_str("\" {");
if !model::Type::is_basic_type(&attr.attribute_type) { 
      str.push_str("\n              state = ");
      str.push_str(&typ.typename);
      str.push_str("ParserState.IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_OBJECT;");
} else { 
      str.push_str("\n              state = ");
      str.push_str(&typ.typename);
      str.push_str("ParserState.IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_VALUE;");
} 
} 
  str.push_str("\n            }\n            // TODO: if Strict-Mode then else with error output\n            // TODO: if flex-Mode then do something to overjump unknown attributes\n            buf = \"\";\n          } else if !is_blank(c) {\n            // TODO: Handle syntax error\n          }\n        case .BEHIND_FIELDVALUE:\n          if c == \",\" {\n            state = ");
  str.push_str(&typ.typename);
  str.push_str("ParserState.INOBJECT;\n          } else if c == \"}\" {\n            state = ");
  str.push_str(&typ.typename);
  str.push_str("ParserState.FINAL;\n          } else if !is_blank(c) {\n            // TODO: Handle syntax error\n          }");
for attr in typ.attributes.iter() { 
    if !model::Type::is_basic_type(&attr.attribute_type) { 
      str.push_str("\n        // Nested objects\n        case .IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_OBJECT:\n          if c == \"{\" {\n            obj.");
      str.push_str(&attr.name);
      str.push_str(" = ");
      str.push_str(&attr.attribute_type);
      str.push_str(".parse_internal(code, ptr:&ptr);\n            state = ");
      str.push_str(&typ.typename);
      str.push_str("ParserState.BEHIND_FIELDVALUE;\n          } else if !is_blank(c) {\n            // TODO: Handle syntax error\n          }");
} else if attr.attribute_type == "string"
        || attr.attribute_type == "char" { 
      str.push_str("\n        // Strings and other values enclosed by \"\n        case .IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_VALUE:\n          if c == \"\\\"\" {\n            state = ");
      str.push_str(&typ.typename);
      str.push_str("ParserState.IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_STRING;\n            buf = \"\";\n          } else if !is_blank(c) {\n            // TODO: Handle syntax error\n          }\n        case .IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_STRING:\n          if c == \"\\\"\" && charbefore != \"\\\\\" {\n            state = ");
      str.push_str(&typ.typename);
      str.push_str("ParserState.BEHIND_FIELDVALUE;\n            obj.");
      str.push_str(&attr.name);
      str.push_str(" = buf;\n            buf = \"\";\n          } else {\n            buf.append(c);\n          }");
} else if  attr.attribute_type == "date"
        || attr.attribute_type == "time"
        || attr.attribute_type == "datetime" { 
      str.push_str("\n        // Strings and other values enclosed by \"\n        case .IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_VALUE:\n          if c == \"\\\"\" {\n            state = ");
      str.push_str(&typ.typename);
      str.push_str("ParserState.IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_STRING;\n            buf = \"\";\n          } else if !is_blank(c) {\n            // TODO: Handle syntax error\n          }\n        case .IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_STRING:\n          if c == \"\\\"\" && charbefore != \"\\\\\" {\n            state = ");
      str.push_str(&typ.typename);
      str.push_str("ParserState.BEHIND_FIELDVALUE;\n            obj.");
      str.push_str(&attr.name);
      str.push_str(" = NSDate(string:buf)!; // TODO: prepare for other date types too\n            buf = \"\";\n          } else {\n            buf.append(c);\n          }");
  } else if attr.attribute_type == "int"
            || attr.attribute_type == "uint"
            || attr.attribute_type == "long"
            || attr.attribute_type == "ulong" {  
      str.push_str("\n        // int-type values without \"\n        case .IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_VALUE:\n          if c == \",\" { \n            state = ");
      str.push_str(&typ.typename);
      str.push_str("ParserState.INOBJECT;");
    // Make string to int conversion dependent to target type 
      str.push_str("");
    if attr.attribute_type == "int" { 
        str.push_str("\n            obj.");
        str.push_str(&attr.name);
        str.push_str(" = Int32(buf.toInt()!);");
    } else if attr.attribute_type == "uint" { 
        str.push_str("\n            obj.");
        str.push_str(&attr.name);
        str.push_str(" = UInt32(buf.toInt()!);");
    } else if attr.attribute_type == "long" { 
        str.push_str("\n            obj.");
        str.push_str(&attr.name);
        str.push_str(" = Int64(buf.toInt()!);");
    } else if attr.attribute_type == "ulong" { 
        str.push_str("\n            obj.");
        str.push_str(&attr.name);
        str.push_str(" = UInt64(buf.toInt()!);");
    } 
      str.push_str("\n          } else if c == \"}\" {\n            state = ");
      str.push_str(&typ.typename);
      str.push_str("ParserState.FINAL;\n            // TODO: code it out\n          } else if c >= \"0\" && c <= \"9\" {\n            // TODO: also allow - for int and long (not for uint and ulong)\n            buf.append(c);");
    if attr.attribute_type == "int" || attr.attribute_type == "long" { 
        str.push_str("\n          } else if c == \"-\" && buf == \"\" {\n            buf.append(c);");
    } 
      str.push_str("\n          } else {\n            // TODO: Handle syntax error\n          }");
  } else { 
      str.push_str("\n        // Other numeric values whithout \"\n        // TODO: code it out !!!");
  } 
    str.push_str("");
} 
  str.push_str("\n        default:\n          // This state is not allwoed to be reached\n          println(\"ERROR: ENCOUNTERED INVALID STATE\");\n      }\n      charbefore = c;\n      ptr.next();\n    }\n \n    return obj;\n  }\n\n  //\n  // Function to serialize objects of type ");
  str.push_str(&typ.typename);
  str.push_str("\n  //\n  public static func serialize(obj:");
  str.push_str(&typ.typename);
  str.push_str(") -> String {\n    var buf = \"{\";");
for attr in typ.attributes.iter() { 
    str.push_str("\n    buf += \"\\\"\";\n    buf += \"");
    str.push_str(&attr.name);
    str.push_str("\";\n    buf += \"\\\":\";");
  if attr.is_array {  
      str.push_str("\n      buf += \"[\";\n      for val in obj.");
      str.push_str(&attr.name);
      str.push_str(" {");
      if attr.attribute_type == "string" { 
        str.push_str("\n        buf += \"\\\"\";\n        buf += \"\\(obj.");
        str.push_str(&attr.name);
        str.push_str(")\";\n        buf += \"\\\"\";");
      } else { 
        str.push_str(" \n        buf += \"\\(obj.");
        str.push_str(&attr.name);
        str.push_str(")\";");
      } 
      str.push_str("       \n      }\n      buf += \"]\";");
  } else if attr.attribute_type == "string" {  
      str.push_str("\n    buf += \"\\\"\";\n    buf += \"\\(obj.");
      str.push_str(&attr.name);
      str.push_str(")\";\n    buf += \"\\\"\";");
  } else { 
      str.push_str("\n    buf += \"\\(obj.");
      str.push_str(&attr.name);
      str.push_str(")\";");
  } 
    str.push_str("");
} 
  str.push_str("\n    buf += \"}\";\n    return buf;\n  }\n\n}\n");
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
    result.push_str("Double");
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


