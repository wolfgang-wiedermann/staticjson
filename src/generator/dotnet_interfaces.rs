use model;
use filehandler;
use util;
use std::collections::HashSet;

//
// This generate Method is the entry point to generation
// of html documentation about the given types
//
pub fn generate(tree:model::ParserResult, folder:&str) {
  
  for ifa in tree.interfaces.iter() {
    let result = gen_interface(ifa, tree.types.clone());
    if ifa.is_param_present("cs-namespace") {
      if ifa.is_param_present("cs-namespace-const") {
        let package = ifa.get_param_value("cs-namespace").replace(&ifa.get_param_value("cs-namespace-const"), "").replace(".", "/");      
        let filename = format!("{}/{}/{}.cs", folder, package, ifa.name);      
        filehandler::write_file(filename, result);
      } else {
        let package = ifa.get_param_value("cs-namespace").replace(".", "/");      
        let filename = format!("{}/{}/{}.cs", folder, package, ifa.name);      
        filehandler::write_file(filename, result);
      }
    } else {
      let filename = format!("{}/{}.cs", folder, ifa.name);
      filehandler::write_file(filename, result);
    }
  }
} 
// 
// Generate code for interface
//
fn gen_interface(ifa:&Box<model::Interface>, types:Box<Vec<Box<model::Type>>>) -> String {
  let mut str:String = String::new(); 

  str.push_str("using System;\nusing System.Collections.Generic;\nusing System.Linq; \nusing System.Web.Http;\nusing System.Security;");
  str.push_str(&get_interfaces_referenced_dotnet_namespaces(&ifa, types.clone()));
  str.push_str("\n\n");
  if ifa.is_param_present("cs-namespace") {

    str.push_str("namespace ");
    str.push_str(&ifa.get_param_value("cs-namespace"));
    str.push_str(" \n{");
} 
  str.push_str("\n\n///\n/// Generated Interface for ");
  str.push_str(&ifa.name);
  str.push_str(" with WebAPI2 Attributes\n/// (this is a partial class for separation of generated and manual written code)\n///");
if ifa.is_param_present("path") { 
    str.push_str("\n[RoutePrefix(\"");
    str.push_str(&util::remove_first_char(&ifa.get_param_value("path")));
    str.push_str("\")]");
} 
  str.push_str("\npublic partial class ");
  str.push_str(&ifa.name);
  str.push_str(" : ApiController {");
for function in ifa.functions.iter() { 
    str.push_str("\n\n    ///");
for param in function.params.iter() { 
      str.push_str("\n    /// <param name=\"");
      str.push_str(&param.name);
      str.push_str("\"></param>");
} 
    str.push_str(" \n    /// <returns>");
    str.push_str(&get_dotnet_type(&function.returntype, function.returntype_is_array));
    str.push_str("</returns>\n    ///");
if function.is_attribute_value_present("method", "GET") { 
      str.push_str("\n    [HttpGet]");
} if function.is_attribute_value_present("method", "PUT") { 
      str.push_str("\n    [HttpPut]");
} if function.is_attribute_value_present("method", "POST") { 
      str.push_str("\n    [HttpPost]");
} if function.is_attribute_value_present("method", "DELETE") { 
      str.push_str("\n    [HttpDelete]");
} if function.is_attribute_present("cs-custom-auth") { 
      str.push_str("\n    [");
      str.push_str(&function.get_attribute_value("cs-custom-auth"));
      str.push_str("]");
} if function.is_attribute_present("cs-authorized-role") { 
      str.push_str("\n    [Authorize(Roles = \"");
      str.push_str(&function.get_attribute_value("cs-authorized-role"));
      str.push_str("\")]");
} if function.is_attribute_present("path") { 
      str.push_str("\n    [Route(\"");
      str.push_str(&util::remove_first_char(&function.get_attribute_value("path")));
      str.push_str("\")]");
} 
    str.push_str("\n    public ");
    str.push_str(&get_dotnet_type(&function.returntype, function.returntype_is_array));
    str.push_str(" ");
    str.push_str(&util::lcamel_to_ucamel(&function.name));
    str.push_str("(");
let mut i = 0;
for param in function.params.iter() { 
  i = i+1;   
  if i > 1 { 
    str.push_str(", "); 
  } if !(param.is_param_present("query-param") || param.is_param_present("path-param")) { 
  
        str.push_str("[FromBody] ");

  } 
      str.push_str("");
      str.push_str(&get_dotnet_type(&param.typename, param.is_array));
      str.push_str(" ");
      str.push_str(&param.name);
      str.push_str("");
} 
    str.push_str(")\n    {\n       ");
if function.returntype != "void" { 
      str.push_str(" return");
} 
    str.push_str(" this.");
    str.push_str(&util::lcamel_to_ucamel(&function.name));
    str.push_str("Impl(");
let mut i = 0;
for param in function.params.iter() { 
  i = i+1;   
  if i > 1 { 
    str.push_str(", "); 
  }  
  
      str.push_str("");
      str.push_str(&param.name);
      str.push_str("");
} 
    str.push_str(");\n    }");
} 
  str.push_str("\n}");
if ifa.is_param_present("cs-namespace") { 
    str.push_str(" \n\n}");
} 
  str.push_str("");
  return str;
} 
// rust utility functions for dotnet webapi2 and entity framework generation 

fn get_dotnet_type(sjtype:&str, is_array:bool) -> String {
  let mut jtype:&str;
  if !model::Type::is_basic_type(sjtype) {
    if is_array {
      return format!("List<{}>", sjtype);
    } else {
      jtype = sjtype;
    }
  } else if sjtype == "bool" {
    if is_array {
      jtype = "List<bool>";
    } else {
      jtype = "bool";
    }
  } else if sjtype == "int" || sjtype == "uint" {
    if is_array {
      jtype = "List<int>";
    } else {
      jtype = "int";
    }
  } else if sjtype == "long" || sjtype == "ulong" {
    if is_array {
      jtype = "List<long>";
    } else {
      jtype = "long";
    }
  } else if sjtype == "string" {
    if is_array {
      jtype = "List<string>";
    } else {
      jtype = "string";
    }
  } else if sjtype == "decimal" {
    if is_array {
      jtype = "List<decimal>";
    } else {
      jtype = "decimal";
    }
  } else if sjtype == "date" {
    if is_array {
      jtype = "List<DateTime>";
    } else {
      jtype = "DateTime?";
    }
  } else {
    jtype = "undef";
  }
  return jtype.to_string();
}

fn get_dotnet_type_initial(sjtype:&str, is_array:bool) -> String {
  let mut jtype:&str;
  if !model::Type::is_basic_type(sjtype) {
    if is_array {
      return format!("new List<{}>()", sjtype);
    } else {
      jtype = "null";
    }
  } else if sjtype == "bool" {
    if is_array {
      jtype = "new List<bool>()";
    } else {
      jtype = "false";
    }
  } else if sjtype == "int" || sjtype == "uint" {
    if is_array {
      jtype = "new List<int>()";
    } else {
      jtype = "0";
    }
  } else if sjtype == "long" || sjtype == "ulong" {
    if is_array {
      jtype = "new List<long>()";
    } else {
      jtype = "0";
    }
  } else if sjtype == "string" {
    if is_array {
      jtype = "new List<string>()";
    } else {
      jtype = "null";
    }
  } else if sjtype == "decimal" {
    if is_array {
      jtype = "new List<decimal>()";
    } else {
      jtype = "0.0m";
    }
  } else if sjtype == "date" {
    if is_array {
      jtype = "new List<DateTime>()";
    } else {
      jtype = "null";
    }
  } else {
    jtype = "undef";
  }
  return jtype.to_string();
}

fn get_types_referenced_dotnet_namespaces(typ:&Box<model::Type>, types:Box<Vec<Box<model::Type>>>) -> String {    
  let mut package_set:HashSet<String> = HashSet::new();
  for attr in typ.attributes.iter() {
    if !model::Type::is_basic_type(&attr.attribute_type) {
      for t in types.iter() {
        if t.typename == attr.attribute_type
           && t.is_param_present("cs-namespace") 
           && !(typ.is_param_present("cs-namespace") 
                && typ.get_param_value("cs-namespace") == t.get_param_value("cs-namespace")){                
           package_set.insert(format!("{}", t.get_param_value("cs-namespace")));
        }
      }
    }
  }
  let mut ret = String::new();
  for package in &package_set {
    ret.push_str(&format!("\nimport {};", package));
  }
  return ret.clone();
}

#[allow(dead_code)]
fn get_interfaces_referenced_dotnet_namespaces(ifa:&Box<model::Interface>, types:Box<Vec<Box<model::Type>>>) -> String {    
  let mut package_set:HashSet<String> = HashSet::new();
  for func in ifa.functions.iter() {
    if !model::Type::is_basic_type(&func.returntype) && func.returntype != "void" {      
      for t in types.iter() {
        if t.typename == func.returntype
           && t.is_param_present("cs-namespace") 
           && !(ifa.is_param_present("cs-namespace") 
                && ifa.get_param_value("cs-namespace") == t.get_param_value("cs-namespace")){
           package_set.insert(format!("{}", t.get_param_value("cs-namespace")));
        }
      }
    }
    for param in func.params.iter() {      
      if !model::Type::is_basic_type(&param.typename) {
        for t in types.iter() {
          if t.typename == param.typename
             && t.is_param_present("cs-namespace") 
             && !(ifa.is_param_present("cs-namespace") 
                  && ifa.get_param_value("cs-namespace") == t.get_param_value("cs-namespace")){
             package_set.insert(format!("{}", t.get_param_value("cs-namespace")));
          }
        }
      }
    }
  }
  let mut ret = String::new();
  for package in &package_set {
    ret.push_str(&format!("\nusing {};", package));
  }
  return ret.clone();
}


