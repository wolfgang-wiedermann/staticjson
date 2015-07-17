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
    
    let test = gen_test(typ);
    let test_filename = format!("{}/Test{}.swift", folder, typ.typename);
    filehandler::write_file(test_filename, test);
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
    str.push_str("");
  if attr.is_array == true && attr.is_param_value_present("mandatory", "true") { 
      str.push_str("\n    public var ");
      str.push_str(&attr.name);
      str.push_str(":[");
      str.push_str(&translate_basic_type(&attr.attribute_type));
      str.push_str("];");
  } else if attr.is_array == true { 
      str.push_str("\n    public var ");
      str.push_str(&attr.name);
      str.push_str(":[");
      str.push_str(&translate_basic_type(&attr.attribute_type));
      str.push_str("]?;");
  } else if attr.is_param_value_present("mandatory", "true") { 
      str.push_str("\n    public var ");
      str.push_str(&attr.name);
      str.push_str(":");
      str.push_str(&translate_basic_type(&attr.attribute_type));
      str.push_str(";");
  } else { 
      str.push_str("\n    public var ");
      str.push_str(&attr.name);
      str.push_str(":");
      str.push_str(&translate_basic_type(&attr.attribute_type));
      str.push_str("?;");
  } 
    str.push_str("");
} 
  str.push_str("\n\n    public init() {");
for attr in typ.attributes.iter() { 
    str.push_str("");
  if attr.is_array == true && attr.is_param_value_present("mandatory", "true") { 
      str.push_str("\n        self.");
      str.push_str(&attr.name);
      str.push_str(" = [];");
  } else if attr.is_array == true { 
      str.push_str("\n        //self.");
      str.push_str(&attr.name);
      str.push_str(" = [];");
  } else if attr.is_param_value_present("mandatory", "true") { 
      str.push_str("\n        self.");
      str.push_str(&attr.name);
      str.push_str(" = ");
      str.push_str(&get_default_value(&attr.attribute_type));
      str.push_str(";");
  } else { 
      str.push_str("\n        //self.");
      str.push_str(&attr.name);
      str.push_str(" = ");
      str.push_str(&get_default_value(&attr.attribute_type));
      str.push_str(";");
  } 
    str.push_str("");
} 
  str.push_str("\n    }\n\n    //\n    // ParserState-Enum for type ");
  str.push_str(&typ.typename);
  str.push_str("\n    //\n    private enum ");
  str.push_str(&typ.typename);
  str.push_str("ParserState {\n        case INITIAL\n        case INOBJECT\n        case IN_FIELDNAME\n        case BEHIND_FIELDNAME\n        case BEHIND_FIELDVALUE\n        case BEHIND_ARRAY\n        case FINAL");
for attr in typ.attributes.iter() {
    if attr.is_array == true { 
      str.push_str("\n        case IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_ARRAY");
   } 
    if attr.attribute_type == "string" { 
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
  str.push_str("\n    }\n\n\n  //\n  // Parsing-Function for type ");
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
  str.push_str("ParserState.FINAL {\n      c = code[ptr.startIndex];\n      switch state {\n        // static part of parsers automaton\n        case .INITIAL: \n          if c == \"{\" {\n            state = ");
  str.push_str(&typ.typename);
  str.push_str("ParserState.INOBJECT;\n          } else if !is_blank(c) {\n            // TODO: Handle syntax error\n            raise_error(\"Invalid character found at ...\", c:c);\n          }\n        case .INOBJECT:\n          if c == \"\\\"\" {\n            state = ");
  str.push_str(&typ.typename);
  str.push_str("ParserState.IN_FIELDNAME;\n            buf = \"\";\n          } else if !is_blank(c) {\n            // TODO: Handle syntax error\n            raise_error(\"Invalid character found at ...\", c:c);\n          }\n        case .IN_FIELDNAME:\n          if c == \"\\\"\" && charbefore != \"\\\\\" {\n            state = ");
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
} else if attr.is_array == true { 
      str.push_str("\n              state = ");
      str.push_str(&typ.typename);
      str.push_str("ParserState.IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_ARRAY;");
} else { 
      str.push_str("\n              state = ");
      str.push_str(&typ.typename);
      str.push_str("ParserState.IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_VALUE;");
} 
} 
  str.push_str("\n            }\n            // TODO: if Strict-Mode then else with error output\n            // TODO: if flex-Mode then do something to overjump unknown attributes\n            buf = \"\";\n          } else if !is_blank(c) {\n            // TODO: Handle syntax error\n            raise_error(\"Invalid character found at ...\", c:c);\n          }\n        case .BEHIND_FIELDVALUE:\n          if c == \",\" {\n            state = ");
  str.push_str(&typ.typename);
  str.push_str("ParserState.INOBJECT;\n          } else if c == \"}\" {\n            state = ");
  str.push_str(&typ.typename);
  str.push_str("ParserState.FINAL;\n          } else if !is_blank(c) {\n            // TODO: Handle syntax error\n            raise_error(\"Invalid character found at ...\", c:c);\n          }\n        case .BEHIND_ARRAY:\n          if c == \",\" {\n            state = ");
  str.push_str(&typ.typename);
  str.push_str("ParserState.INOBJECT;\n          } else if c == \"}\" {\n            state = ");
  str.push_str(&typ.typename);
  str.push_str("ParserState.FINAL;\n          } else if !is_blank(c) {\n            // TODO: Handle syntax error\n            raise_error(\"Invalid character found at ...\", c:c);\n          }\n\n        // attribute dependent part of parsers automaton");
  //
  // Cases per attribute of the json object
  // 
  str.push_str("");
for attr in typ.attributes.iter() { 
    str.push_str("");
   if attr.is_array == true { 
      str.push_str("\n        case .IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_ARRAY:\n          if c == \"[\" {");
  if !attr.is_param_value_present("mandatory", "true") { 
        str.push_str("\n            obj.");
        str.push_str(&attr.name);
        str.push_str(" = [];");
  } 
      str.push_str("");
  if !model::Type::is_basic_type(&attr.attribute_type) { 
        str.push_str("\n            state = ");
        str.push_str(&typ.typename);
        str.push_str("ParserState.IN_");
        str.push_str(&util::to_upper(&attr.name));
        str.push_str("_OBJECT;\n          } else if c == \",\" {\n            state = ");
        str.push_str(&typ.typename);
        str.push_str("ParserState.IN_");
        str.push_str(&util::to_upper(&attr.name));
        str.push_str("_OBJECT;");
  } else { 
        str.push_str("\n            state = ");
        str.push_str(&typ.typename);
        str.push_str("ParserState.IN_");
        str.push_str(&util::to_upper(&attr.name));
        str.push_str("_VALUE;");
  } 
      str.push_str("\n          } else if c == \"]\" {\n            state = ");
      str.push_str(&typ.typename);
      str.push_str("ParserState.BEHIND_ARRAY;");
// special case: array with elements in "
    if attr.attribute_type == "string"
      || attr.attribute_type == "char"
      || attr.attribute_type == "date"
      || attr.attribute_type == "time"
      || attr.attribute_type == "datetime" { 
        str.push_str("\n          } else if c == \",\" {\n            state = ");
        str.push_str(&typ.typename);
        str.push_str("ParserState.IN_");
        str.push_str(&util::to_upper(&attr.name));
        str.push_str("_VALUE;");
} 
      str.push_str("\n          } else if !is_blank(c) {\n            // TODO: Handle syntax error\n            // raise_error(\"Invalid character found at ...\", c:c);\n          }");
   } 
    str.push_str("");
   if !model::Type::is_basic_type(&attr.attribute_type) { 
      str.push_str("\n        // Nested objects\n        case .IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_OBJECT:\n          if c == \"{\" {");
if attr.is_array == true && attr.is_param_value_present("mandatory", "true") { 
        str.push_str("\n            obj.");
        str.push_str(&attr.name);
        str.push_str(".append(");
        str.push_str(&attr.attribute_type);
        str.push_str(".parse_internal(code, ptr:&ptr));\n            state = ");
        str.push_str(&typ.typename);
        str.push_str("ParserState.IN_");
        str.push_str(&util::to_upper(&attr.name));
        str.push_str("_ARRAY;");
} else if attr.is_array == true { 
        str.push_str("\n            if obj.");
        str.push_str(&attr.name);
        str.push_str(" == nil {\n                obj.");
        str.push_str(&attr.name);
        str.push_str(" = [];\n            }\n            obj.");
        str.push_str(&attr.name);
        str.push_str("!.append(");
        str.push_str(&attr.attribute_type);
        str.push_str(".parse_internal(code, ptr:&ptr));\n            state = ");
        str.push_str(&typ.typename);
        str.push_str("ParserState.IN_");
        str.push_str(&util::to_upper(&attr.name));
        str.push_str("_ARRAY;");
} else { 
        str.push_str("\n            obj.");
        str.push_str(&attr.name);
        str.push_str(" = ");
        str.push_str(&attr.attribute_type);
        str.push_str(".parse_internal(code, ptr:&ptr);\n            state = ");
        str.push_str(&typ.typename);
        str.push_str("ParserState.BEHIND_FIELDVALUE;");
} 
      str.push_str("\n          } else if !is_blank(c) {\n            // TODO: Handle syntax error\n            // raise_error(\"Invalid character found at ...\", c:c);\n          }");
} else if attr.attribute_type == "string"
        || attr.attribute_type == "char" { 
      str.push_str("\n        // Strings and other values enclosed by \"\n        case .IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_VALUE:\n          if c == \"\\\"\" {\n            state = ");
      str.push_str(&typ.typename);
      str.push_str("ParserState.IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_STRING;\n            buf = \"\";\n          } else if !is_blank(c) {\n            // TODO: Handle syntax error\n            raise_error(\"Invalid character found at ...\", c:c);\n          }\n        case .IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_STRING:\n          if c == \"\\\"\" && charbefore != \"\\\\\" {");
if attr.is_array == true && (
      attr.attribute_type == "string"
      || attr.attribute_type == "char"
      || attr.attribute_type == "date"
      || attr.attribute_type == "time"
      || attr.attribute_type == "datetime") { 
        str.push_str("\n            state = ");
        str.push_str(&typ.typename);
        str.push_str("ParserState.IN_");
        str.push_str(&util::to_upper(&attr.name));
        str.push_str("_ARRAY;");
} else { 
        str.push_str("\n            state = ");
        str.push_str(&typ.typename);
        str.push_str("ParserState.BEHIND_FIELDVALUE;");
} 
      str.push_str("");
if attr.is_array == true && attr.is_param_value_present("mandatory", "true") { 
        str.push_str("\n            obj.");
        str.push_str(&attr.name);
        str.push_str(".append(buf);");
} else if attr.is_array == true { 
        str.push_str("\n            if let x = obj.");
        str.push_str(&attr.name);
        str.push_str(" {\n                obj.");
        str.push_str(&attr.name);
        str.push_str("!.append(buf);\n            } else {\n                if obj.");
        str.push_str(&attr.name);
        str.push_str(" != nil { \n                    obj.");
        str.push_str(&attr.name);
        str.push_str(" = [];\n                }\n                obj.");
        str.push_str(&attr.name);
        str.push_str("!.append(buf);\n            }");
} else { 
        str.push_str("\n            obj.");
        str.push_str(&attr.name);
        str.push_str(" = buf;");
} 
      str.push_str("\n            buf = \"\";\n          } else {\n            buf.append(c);\n          }");
} else if  attr.attribute_type == "date"
        || attr.attribute_type == "time"
        || attr.attribute_type == "datetime" { 
      str.push_str("\n        // Strings and other values enclosed by \"\n        case .IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_VALUE:\n          if c == \"\\\"\" {\n            state = ");
      str.push_str(&typ.typename);
      str.push_str("ParserState.IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_STRING;\n            buf = \"\";\n          } else if !is_blank(c) {\n            // TODO: Handle syntax error\n            raise_error(\"Invalid character found at ...\", c:c);\n          }\n        case .IN_");
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
      str.push_str("_VALUE:");
    if attr.is_array == true { 
        str.push_str("\n          if obj.");
        str.push_str(&attr.name);
        str.push_str(" == nil {  \n            obj.");
        str.push_str(&attr.name);
        str.push_str(" = [];\n          }");
    } 
      str.push_str("\n          if c == \",\" {");
    if attr.is_array == false { 
        str.push_str("\n            state = ");
        str.push_str(&typ.typename);
        str.push_str("ParserState.INOBJECT;");
    } 
      str.push_str(" ");
    if !attr.is_param_value_present("mandatory", "true") { 
        str.push_str("");
    // Make string to int conversion dependent to target type 
        str.push_str("");
    if attr.attribute_type == "int" && attr.is_array == true { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str("!.append(Int32(buf.toInt()!));");
    } else if attr.attribute_type == "int" { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(" = Int32(buf.toInt()!);");
    } else if attr.attribute_type == "uint" && attr.is_array == true { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str("!.append(UInt32(buf.toInt()!));");
    } else if attr.attribute_type == "uint" { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(" = UInt32(buf.toInt()!);");
    } else if attr.attribute_type == "long" && attr.is_array == true { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str("!.append(Int64(buf.toInt()!));");
    } else if attr.attribute_type == "long" { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(" = Int64(buf.toInt()!);");
    } else if attr.attribute_type == "ulong" && attr.is_array == true { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str("!.append(UInt64(buf.toInt()!));");
    } else if attr.attribute_type == "ulong" { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(" = UInt64(buf.toInt()!);");
    } 
        str.push_str("\n");
    } else { 
        str.push_str("");
    // Make string to int conversion dependent to target type 
        str.push_str("");
    if attr.attribute_type == "int" && attr.is_array == true { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(".append(Int32(buf.toInt()!));");
    } else if attr.attribute_type == "int" { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(" = Int32(buf.toInt()!);");
    } else if attr.attribute_type == "uint" && attr.is_array == true { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(".append(UInt32(buf.toInt()!));");
    } else if attr.attribute_type == "uint" { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(" = UInt32(buf.toInt()!);");
    } else if attr.attribute_type == "long" && attr.is_array == true { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(".append(Int64(buf.toInt()!));");
    } else if attr.attribute_type == "long" { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(" = Int64(buf.toInt()!);");
    } else if attr.attribute_type == "ulong" && attr.is_array == true { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(".append(UInt64(buf.toInt()!));");
    } else if attr.attribute_type == "ulong" { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(" = UInt64(buf.toInt()!);");
    } 
        str.push_str("\n");
    } 
      str.push_str("");
    if attr.is_array == true { 
        str.push_str("\n            buf = \"\";\n          } else if c == \"]\" {\n            state = ");
        str.push_str(&typ.typename);
        str.push_str("ParserState.BEHIND_ARRAY;");
    } else { 
        str.push_str("\n          } else if c == \"}\" {\n            state = ");
        str.push_str(&typ.typename);
        str.push_str("ParserState.FINAL;");
    } 
      str.push_str("");
    if !attr.is_param_value_present("mandatory", "true") { 
        str.push_str("");
    // Make string to int conversion dependent to target type 
        str.push_str("");
    if attr.attribute_type == "int" && attr.is_array == true { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str("!.append(Int32(buf.toInt()!));");
    } else if attr.attribute_type == "int" { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(" = Int32(buf.toInt()!);");
    } else if attr.attribute_type == "uint" && attr.is_array == true { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str("!.append(UInt32(buf.toInt()!));");
    } else if attr.attribute_type == "uint" { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(" = UInt32(buf.toInt()!);");
    } else if attr.attribute_type == "long" && attr.is_array == true { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str("!.append(Int64(buf.toInt()!));");
    } else if attr.attribute_type == "long" { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(" = Int64(buf.toInt()!);");
    } else if attr.attribute_type == "ulong" && attr.is_array == true { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str("!.append(UInt64(buf.toInt()!));");
    } else if attr.attribute_type == "ulong" { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(" = UInt64(buf.toInt()!);");
    } 
        str.push_str("\n");
    } else { 
        str.push_str("");
    // Make string to int conversion dependent to target type 
        str.push_str("");
    if attr.attribute_type == "int" && attr.is_array == true { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(".append(Int32(buf.toInt()!));");
    } else if attr.attribute_type == "int" { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(" = Int32(buf.toInt()!);");
    } else if attr.attribute_type == "uint" && attr.is_array == true { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(".append(UInt32(buf.toInt()!));");
    } else if attr.attribute_type == "uint" { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(" = UInt32(buf.toInt()!);");
    } else if attr.attribute_type == "long" && attr.is_array == true { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(".append(Int64(buf.toInt()!));");
    } else if attr.attribute_type == "long" { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(" = Int64(buf.toInt()!);");
    } else if attr.attribute_type == "ulong" && attr.is_array == true { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(".append(UInt64(buf.toInt()!));");
    } else if attr.attribute_type == "ulong" { 
          str.push_str("\n            obj.");
          str.push_str(&attr.name);
          str.push_str(" = UInt64(buf.toInt()!);");
    } 
        str.push_str("\n");
    } 
      str.push_str("");
    if attr.is_array == true { 
        str.push_str("\n            buf = \"\";");
    } 
      str.push_str("\n          } else if c >= \"0\" && c <= \"9\" {\n            buf.append(c);");
    if attr.attribute_type == "int" || attr.attribute_type == "long" { 
        str.push_str("\n          } else if c == \"-\" && buf == \"\" {\n            buf.append(c);");
    } 
      str.push_str("\n          } else if c == \" \" && buf == \"\" {\n            // ignore if first char is blank\n          } else {\n            // TODO: Handle syntax error\n            raise_error(\"Invalid character found at ...\", c:c);\n          }");
  } else if attr.attribute_type == "decimal" { 
      str.push_str("\n        case .IN_");
      str.push_str(&util::to_upper(&attr.name));
      str.push_str("_VALUE:\n          // parse decimal values with dot as decimal sign\n          if c == \",\" {\n            state = ");
      str.push_str(&typ.typename);
      str.push_str("ParserState.INOBJECT;\n            obj.");
      str.push_str(&attr.name);
      str.push_str(" = (buf as NSString).doubleValue;\n            buf = \"\";\n          } else if c == \"}\" {\n            state = ");
      str.push_str(&typ.typename);
      str.push_str("ParserState.FINAL;\n            obj.");
      str.push_str(&attr.name);
      str.push_str(" = (buf as NSString).doubleValue;\n            buf = \"\";\n          } else if c >= \"0\" && c <= \"9\" {\n            buf.append(c);\n          } else if c == \".\" || c == \"-\" {\n            buf.append(c);\n          } else {\n            // TODO: Handle syntax error\n            raise_error(\"Invalid character found at ...\", c:c);\n          }");
  } 
    str.push_str("");
} 
  str.push_str("\n        default:\n          // This state is not allwoed to be reached\n          println(\"ERROR: ENCOUNTERED INVALID STATE\");\n      }\n      charbefore = c;\n      ptr.next();\n    }\n\n    validate_mandatory(obj);\n\n    return obj;\n  }\n\n  // Validation of mandatory attributes\n  private static func validate_mandatory(obj:");
  str.push_str(&typ.typename);
  str.push_str(") -> String {\n    var is_valid = true;\n    var message = \"\";");
for attr in typ.attributes.iter() {
    if attr.is_param_value_present("mandatory", "true") { 
      str.push_str("\n    if obj.");
      str.push_str(&attr.name);
      str.push_str(" == ");
      str.push_str(&get_default_value(&attr.attribute_type));
      str.push_str(" {\n      is_valid = is_valid && false;\n      message += \"ERROR: ");
      str.push_str(&attr.name);
      str.push_str(" is mandatory, but has empty value\\n\"; \n    } else {\n      is_valid = is_valid && true;\n    }");
  } 
} 
  str.push_str("\n    return message;\n  }\n\n  // Raise parser errors\n  private static func raise_error(message:String, c:Character) {\n    var debugMsg = \"ERROR: \"+message+\" \";\n    debugMsg.append(c);\n    println(debugMsg);\n    NSException(name: \"Syntax-Error\", reason: \"Invalid character found at ...\", userInfo: nil).raise()\n  }\n\n  //\n  // Function to serialize objects of type ");
  str.push_str(&typ.typename);
  str.push_str("\n  //\n  public static func serialize(obj:");
  str.push_str(&typ.typename);
  str.push_str(") -> String {\n    var idx = 0;\n    var max_idx = 0;\n    var buf = \"{\";");
let mut r_idx = 0;
let mut r_max_idx = typ.attributes.len();
for attr in typ.attributes.iter() {
  r_idx += 1; 
    str.push_str("");
if !attr.is_param_value_present("mandatory", "true") { 
      str.push_str("\n// ------ Attribute: ");
      str.push_str(&attr.name);
      str.push_str("\n    if obj.");
      str.push_str(&attr.name);
      str.push_str(" != nil {\n// ------\n    buf += \"\\\"\";\n    buf += \"");
      str.push_str(&attr.name);
      str.push_str("\";\n    buf += \"\\\":\";");
  if attr.is_array {  
        str.push_str("\n      buf += \"[\";\n      idx = 0;\n      max_idx = obj.");
        str.push_str(&attr.name);
        str.push_str("!.count;\n      for val in obj.");
        str.push_str(&attr.name);
        str.push_str("! {\n        idx++;");
      if attr.attribute_type == "string"
         || attr.attribute_type == "date"
         || attr.attribute_type == "datetime"
         || attr.attribute_type == "time"  { 
          str.push_str("\n        buf += \"\\\"\";\n        buf += \"\\(val)\";\n        buf += \"\\\"\";");
      } else if !model::Type::is_basic_type(&attr.attribute_type) { 
          str.push_str("\n        buf += ");
          str.push_str(&attr.attribute_type);
          str.push_str(".serialize(val);");
      } else { 
          str.push_str(" \n        buf += \"\\(val)\";");
      } 
        str.push_str("\n        if idx < max_idx {\n          buf += \", \";\n        }\n      }\n      buf += \"]\";");
  } else if attr.attribute_type == "string"
     || attr.attribute_type == "date"
     || attr.attribute_type == "datetime"
     || attr.attribute_type == "time" {  
        str.push_str("\n    buf += \"\\\"\";\n    buf += \"\\(obj.");
        str.push_str(&attr.name);
        str.push_str("!)\";\n    buf += \"\\\"\";");
  } else if !model::Type::is_basic_type(&attr.attribute_type) { 
        str.push_str("\n    buf += ");
        str.push_str(&attr.attribute_type);
        str.push_str(".serialize(obj.");
        str.push_str(&attr.name);
        str.push_str("!);");
  } else { 
        str.push_str("\n    buf += \"\\(obj.");
        str.push_str(&attr.name);
        str.push_str("!)\";");
  } 
      str.push_str("");
if r_idx < r_max_idx { 
        str.push_str("\n    buf += \", \";");
 } 
      str.push_str("\n// ------\n     }\n// ------");
} else { 
      str.push_str("\n// ------ Attribute: ");
      str.push_str(&attr.name);
      str.push_str("\n    buf += \"\\\"\";\n    buf += \"");
      str.push_str(&attr.name);
      str.push_str("\";\n    buf += \"\\\":\";");
  if attr.is_array {  
        str.push_str("\n      buf += \"[\";\n      idx = 0;\n      max_idx = obj.");
        str.push_str(&attr.name);
        str.push_str(".count;\n      for val in obj.");
        str.push_str(&attr.name);
        str.push_str(" {\n        idx++;");
      if attr.attribute_type == "string"
         || attr.attribute_type == "date"
         || attr.attribute_type == "datetime"
         || attr.attribute_type == "time"  { 
          str.push_str("\n        buf += \"\\\"\";\n        buf += \"\\(val)\";\n        buf += \"\\\"\";");
      } else if !model::Type::is_basic_type(&attr.attribute_type) { 
          str.push_str("\n        buf += ");
          str.push_str(&attr.attribute_type);
          str.push_str(".serialize(val);");
      } else { 
          str.push_str(" \n        buf += \"\\(val)\";");
      } 
        str.push_str("\n        if idx < max_idx {\n          buf += \", \";\n        }\n      }\n      buf += \"]\";");
  } else if attr.attribute_type == "string"
     || attr.attribute_type == "date"
     || attr.attribute_type == "datetime"
     || attr.attribute_type == "time" {  
        str.push_str("\n    buf += \"\\\"\";\n    buf += \"\\(obj.");
        str.push_str(&attr.name);
        str.push_str(")\";\n    buf += \"\\\"\";");
  } else if !model::Type::is_basic_type(&attr.attribute_type) { 
        str.push_str("\n    buf += ");
        str.push_str(&attr.attribute_type);
        str.push_str(".serialize(obj.");
        str.push_str(&attr.name);
        str.push_str(");");
  } else { 
        str.push_str("\n    buf += \"\\(obj.");
        str.push_str(&attr.name);
        str.push_str(")\";");
  } 
      str.push_str("");
if r_idx < r_max_idx { 
        str.push_str("\n    buf += \", \";");
 } 
      str.push_str("");
}
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


// Generate single types of the data model
fn gen_test(typ:&Box<model::Type>) -> String {
  let mut str:String = String::new(); 

  str.push_str("\n//\n//  Test");
  str.push_str(&typ.typename);
  str.push_str(".swift\n//  generated by staticjson parser generator\n// \n\nimport Cocoa\nimport XCTest\n\npublic class Test");
  str.push_str(&typ.typename);
  str.push_str(" : XCTestCase {\n\n    private let correctJson = self.getPositiveStr()\n    private let wrongJson = self.getNegativeStr()\n\n    override func setUp() {\n        super.setUp()\n        // nothing to set up\n    }\n    \n    override func tearDown() {\n        // nothing to tear down\n        super.tearDown()\n    }\n    \n    // Test-Cases\n    func testParsing_Success() {\n        var obj = ");
  str.push_str(&typ.typename);
  str.push_str(".parse(self.correctJson);");
for attr in typ.attributes.iter() { 
    str.push_str("\n        XCTAssert(\"\\(obj.");
    str.push_str(&attr.name);
    str.push_str(")\" == \"");
    str.push_str(&get_test_value(&attr.attribute_type));
    str.push_str("\", \"Field: ");
    str.push_str(&attr.name);
    str.push_str(" - \\(obj.");
    str.push_str(&attr.name);
    str.push_str(")\")");
} 
  str.push_str("\n    }\n    \n    // TODO: insert a negative Test-Case\n\n    // Helper Functions\n    // Generate a json string for successful parsing unittests\n    public static func getPositiveStr() -> String {\n        var json = \"{\";");
let mut r_idx = 0;
let mut r_max_idx = typ.attributes.len(); 
for attr in typ.attributes.iter() { 
    str.push_str("\n        json += \"\\\"");
    str.push_str(&attr.name);
    str.push_str("\\\"\";\n        json += \":\";");
if attr.is_array { 
      str.push_str("\n        json += \"[\";");
} if model::Type::is_basic_type(&attr.attribute_type) { 
      str.push_str("\n        json += \"");
      str.push_str(&get_test_value(&attr.attribute_type));
      str.push_str("\";");
} else { 
      str.push_str("\n        json += Test");
      str.push_str(&attr.attribute_type);
      str.push_str(".getPositiveStr();");
} if attr.is_array { 
      str.push_str("\n        json += \"]\";");
} 
    str.push_str("");
  r_idx += 1;
  if r_idx < r_max_idx { 
      str.push_str("\n        json += \",\";");
  }
} 
  str.push_str("\n        json += \"}\";\n        return json;\n    }\n\n    // Generate a json string to produce errors parsing unittests\n    public static func getNegativeStr() -> String {\n        var json = \"{\";");
let mut r_idx = 0;
let mut r_max_idx = typ.attributes.len(); 
for attr in typ.attributes.iter() { 
    str.push_str("\n        json += \"\\\"");
    str.push_str(&attr.name);
    str.push_str("\\\"\";\n        json += \":\";\n        json += \"");
    str.push_str(&get_wrong_value(&attr.attribute_type));
    str.push_str("\";");
  r_idx += 1;
  if r_idx < r_max_idx { 
      str.push_str("\n        json += \",\";");
  }
} 
  str.push_str("\n        json += \"}\";\n        return json;\n    }\n}");
  

  // Offer correct json values for staticjson basic types
fn get_test_value(tname:&str) -> String {
  let mut result = String::new();
  if !model::Type::is_basic_type(tname) {
    //result.push_str(tname);
    result.push_str("null");
    // TODO: correct it here, call the function of subtype
  } else if tname == "string" {
    result.push_str("\\\"abcd\\\"");
  } else if tname == "int" {
    result.push_str("-123");
  } else if tname == "uint" {
    result.push_str("123");
  } else if tname == "decimal" {
    result.push_str("123.32");
  } else if tname == "byte" {
    result.push_str("127");
  } else if tname == "char" {
    result.push_str("\"a\"");
  } else if tname == "long" {
    result.push_str("-1234567891230");
  } else if tname == "ulong" {
    result.push_str("1023219832123");
  } else if tname == "date" {
    result.push_str("\\\"2012-12-21\\\"");
  } else {
    result.push_str("XXXXXXXX");
  }
  return result;
}


// Offer invalid json values for staticjson basic types
fn get_wrong_value(tname:&str) -> String {
  let mut result = String::new();
  if !model::Type::is_basic_type(tname) {
    //result.push_str(tname);
    result.push_str("%%%");
    // TODO: correct it here, call the function of subtype
  } else if tname == "string" {
    result.push_str("!sE4$");
  } else if tname == "int" {
    result.push_str("-*123ua");
  } else if tname == "uint" {
    result.push_str("-123");
  } else if tname == "decimal" {
    result.push_str("123,32");
  } else if tname == "byte" {
    result.push_str("1+27");
  } else if tname == "char" {
    result.push_str("\"abc\"");
  } else if tname == "long" {
    result.push_str("-1234c567891230");
  } else if tname == "ulong" {
    result.push_str("102321c9832123");
  } else if tname == "date" {
    result.push_str("\\\"2012-12+21\\\"");
  } else {
    result.push_str("XXXXXXXX");
  }
  return result;
}
  
  return str;
} 

