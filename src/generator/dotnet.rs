use model;
use filehandler;
use util;
use std::collections::HashSet;

//
// This generate Method is the entry point to generation
// of html documentation about the given types
//
pub fn generate(tree:model::ParserResult, folder:&str) {
  for typ in tree.types.iter() {
    let result = gen_type(typ, tree.types.clone());
    if typ.is_param_present("cs-namespace") {
      let package = typ.get_param_value("cs-namespace").replace(".", "/");
      let filename = format!("{}/{}/{}.cs", folder, package, typ.typename);
      filehandler::write_file(filename, result);
    } else {
      let filename = format!("{}/{}.cs", folder, typ.typename);
      filehandler::write_file(filename, result);
    }
  }
  
  for ifa in tree.interfaces.iter() {
    let result = gen_interface(ifa, tree.types.clone());
    if ifa.is_param_present("cs-namespace") {
      let package = ifa.get_param_value("cs-namespace").replace(".", "/");
      let filename = format!("{}/{}/{}.cs", folder, package, ifa.name);
      filehandler::write_file(filename, result);
    } else {
      let filename = format!("{}/{}.cs", folder, ifa.name);
      filehandler::write_file(filename, result);
    }
  }
} 
// 
// Generate code for type
//
fn gen_type(typ:&Box<model::Type>, types:Box<Vec<Box<model::Type>>>) -> String {
  let mut str:String = String::new(); 

  str.push_str("  \nusing System;\nusing System.Collections.Generic;\nusing System.Linq; \n");
  str.push_str(&get_types_referenced_dotnet_namespaces(&typ, types.clone()));
  str.push_str("\n");
  if typ.is_param_present("cs-namespace") {

    str.push_str("namespace ");
    str.push_str(&typ.get_param_value("cs-namespace"));
    str.push_str(" \n{");
} 
  str.push_str("\n\n/**\n* Generated Type for Entity ");
  str.push_str(&typ.typename);
  str.push_str(" \n*/");
if typ.is_param_value_present("ef-entity", "true") { 
    str.push_str("\n@Entity");
} if typ.is_param_present("ef-table") { 
    str.push_str("\n@Table(name=\"");
    str.push_str(&typ.get_param_value("ef-table"));
    str.push_str("\")");
} 
  str.push_str("\npublic class ");
  str.push_str(&typ.typename);
  str.push_str(" {\n\n    #region properties");
    for attribut in typ.attributes.iter() { 
    str.push_str("\n    public ");
    str.push_str(&get_dotnet_type(&attribut.attribute_type, attribut.is_array));
    str.push_str(" ");
    str.push_str(&util::lsnake_to_ucamel(&attribut.name));
    str.push_str(" { get; set; }   ");
    } 
  str.push_str("\n    #endregion\n\n    public ");
  str.push_str(&typ.typename);
  str.push_str("() {");
    for attribut in typ.attributes.iter() { 
    str.push_str("\n        this.");
    str.push_str(&util::lsnake_to_ucamel(&attribut.name));
    str.push_str(" = ");
    str.push_str(&get_dotnet_type_initial(&attribut.attribute_type, attribut.is_array));
    str.push_str(";");
    } 
  str.push_str("\n    }\n\n    /**\n    * The function isValid offert a validation function for the\n    * mandatory attributes and other constraints of staticjson code\n    * @param object to check\n    * @return check result\n    */\n    public static boolean isValid(");
  str.push_str(&typ.typename);
  str.push_str(" obj) {\n        return obj != null");
    for attribut in typ.attributes.iter() { 
      if attribut.is_param_value_present("mandatory", "true") { 
      str.push_str("\n        && obj.");
      str.push_str(&util::lsnake_to_ucamel(&attribut.name));
      str.push_str(" != ");
      str.push_str(&get_dotnet_type_initial(&attribut.attribute_type, attribut.is_array));
      str.push_str("");
      } if attribut.is_param_present("maxlen") && attribut.attribute_type == "string" && !attribut.is_array { 
      str.push_str("\n        && (obj.");
      str.push_str(&util::lsnake_to_ucamel(&attribut.name));
      str.push_str(" != null && \n            obj.");
      str.push_str(&util::lsnake_to_ucamel(&attribut.name));
      str.push_str(".Length <= ");
      str.push_str(&attribut.get_param_value("maxlen"));
      str.push_str(")");
      } if attribut.is_param_present("minlen") && attribut.attribute_type == "string" && !attribut.is_array { 
      str.push_str("\n        && (obj.");
      str.push_str(&util::lsnake_to_ucamel(&attribut.name));
      str.push_str(" != null && \n            obj.");
      str.push_str(&util::lsnake_to_ucamel(&attribut.name));
      str.push_str(".Length >= ");
      str.push_str(&attribut.get_param_value("minlen"));
      str.push_str(")");
      } 
    } 
  str.push_str(";\n    }\n}\n");
  if typ.is_param_present("cs-namespace") {

    str.push_str(" \n}");
}
    return str;
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
  str.push_str("\n\n/**\n* Generated Interface for ");
  str.push_str(&ifa.name);
  str.push_str(" with WebAPI2 Attributes\n*/");
if ifa.is_param_present("path") { 
    str.push_str("\n[RoutePrefix(\"");
    str.push_str(&ifa.get_param_value("path"));
    str.push_str("\")]");
} 
  str.push_str("\npublic interface ");
  str.push_str(&ifa.name);
  str.push_str(" : ApiController {");
for function in ifa.functions.iter() { 
    str.push_str("\n\n    /**");
for param in function.params.iter() { 
      str.push_str("\n     * @param ");
      str.push_str(&param.name);
      str.push_str("");
} 
    str.push_str(" \n     * @return ");
    str.push_str(&get_dotnet_type(&function.returntype, function.returntype_is_array));
    str.push_str("\n     */");
if function.is_attribute_value_present("method", "GET") { 
      str.push_str("\n    [HttpGet]");
} if function.is_attribute_value_present("method", "PUT") { 
      str.push_str("\n    [HttpPut]");
} if function.is_attribute_value_present("method", "POST") { 
      str.push_str("\n    [HttpPost]");
} if function.is_attribute_value_present("method", "DELETE") { 
      str.push_str("\n    [HttpDelete]");
} if function.is_attribute_present("path") { 
      str.push_str("\n    [Route(\"");
      str.push_str(&function.get_attribute_value("path"));
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
  } if param.is_param_present("query-param") {
    str.push_str(&format!("@QueryParam(\"{}\") ", param.get_param_value("query-param")));
  } if param.is_param_present("path-param") {
    str.push_str(&format!("@PathParam(\"{}\") ", param.get_param_value("path-param")));
  } 
      str.push_str("");
      str.push_str(&get_dotnet_type(&param.typename, param.is_array));
      str.push_str(" ");
      str.push_str(&param.name);
      str.push_str("");
} 
    str.push_str(");");
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
      jtype = "String";
    }
  } else if sjtype == "decimal" {
    if is_array {
      jtype = "List<decimal>";
    } else {
      jtype = "decimal";
    }
  } else if sjtype == "date" {
    if is_array {
      jtype = "List<Date>";
    } else {
      jtype = "Date";
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
      jtype = "0.0";
    }
  } else if sjtype == "date" {
    if is_array {
      jtype = "new List<Date>()";
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
      package_set.insert(format!("javax.ws.rs.Consumes"));
      for t in types.iter() {
        if t.typename == func.returntype
           && t.is_param_present("cs-namespace") 
           && !(ifa.is_param_present("cs-namespace") 
                && ifa.get_param_value("cs-namespace") == t.get_param_value("cs-namespace")){
           package_set.insert(format!("{}", t.get_param_value("java-package")));
        }
      }
    }
    for param in func.params.iter() {
      if param.is_param_present("path-param") {
        package_set.insert(format!("javax.ws.rs.PathParam"));
      }
      if param.is_param_present("query-param") {
        package_set.insert(format!("javax.ws.rs.QueryParam"));
      }
      if !model::Type::is_basic_type(&param.typename) {
        for t in types.iter() {
          if t.typename == param.typename
             && t.is_param_present("cs-namespace") 
             && !(ifa.is_param_present("cs-namespace") 
                  && ifa.get_param_value("cs-namespace") == t.get_param_value("java-package")){
             package_set.insert(format!("{}", t.get_param_value("java-package")));
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


