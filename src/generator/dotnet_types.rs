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
    let result = gen_type(typ, tree.clone());
    if typ.is_param_present("cs-namespace") {
      if typ.is_param_present("cs-namespace-const") {
        let package = typ.get_param_value("cs-namespace").replace(&typ.get_param_value("cs-namespace-const"), "").replace(".", "/");      
        let filename = format!("{}/{}/{}.cs", folder, package, typ.typename);      
        filehandler::write_file(filename, result);
      } else {
        let package = typ.get_param_value("cs-namespace").replace(".", "/");      
        let filename = format!("{}/{}/{}.cs", folder, package, typ.typename);      
        filehandler::write_file(filename, result);
      }
    } else {
      let filename = format!("{}/{}.cs", folder, typ.typename);
      filehandler::write_file(filename, result);
    }
  }
    
} 
// 
// Generate code for type
// For details about entity frameworks attributes see https://msdn.microsoft.com/en-us/data/jj591583
//
fn gen_type(typ:&Box<model::Type>, result:model::ParserResult) -> String {
  let mut str:String = String::new(); 

  str.push_str("  \nusing System;\nusing System.Collections.Generic;\nusing System.Linq; \nusing System.ComponentModel.DataAnnotations;\nusing System.ComponentModel.DataAnnotations.Schema;");
if typ.is_param_present("cs-data-contract") || typ.is_attribute_param_present("cs-data-contract") { 
    str.push_str("\nusing System.Runtime.Serialization;");
} 
  str.push_str("\n");
  str.push_str(&get_types_referenced_dotnet_namespaces(&typ, result.types.clone()));
  str.push_str("\n");
  if typ.is_param_present("cs-namespace") {

    str.push_str("namespace ");
    str.push_str(&typ.get_param_value("cs-namespace"));
    str.push_str(" \n{");
} 
  str.push_str("\n\n///\n/// Generated Type for Entity ");
  str.push_str(&typ.typename);
  str.push_str(" \n///");
if typ.is_param_present("ef-table") { 
    str.push_str("\n[Table(\"");
    str.push_str(&typ.get_param_value("ef-table"));
    str.push_str("\")]");
} if typ.is_param_value_present("cs-data-contract", "true") { 
    str.push_str("\n[DataContract]");
} 
  str.push_str("\npublic class ");
  str.push_str(&typ.typename);
  str.push_str(" {\n\n    #region properties");
    for attribut in typ.attributes.iter() { 
         if attribut.is_param_value_present("ef-id", "true") { 
      str.push_str("\n    [Key]");
} if attribut.is_param_value_present("ef-id", "true") { 
      str.push_str("\n    [DatabaseGenerated(DatabaseGeneratedOption.Identity)]");
} if attribut.is_param_value_present("cs-data-contract", "true") { 
      str.push_str("\n    [DataMember]");
} 
    str.push_str("\n    public ");
    str.push_str(&get_dotnet_type(&attribut.attribute_type, attribut.is_array));
    str.push_str(" ");
    str.push_str(&util::lsnake_to_ucamel(&attribut.name));
    str.push_str(" { get; set; }   \n");
    } 
  str.push_str("\n    #endregion\n\n    public ");
  str.push_str(&typ.typename);
  str.push_str("() {");
    for attribut in typ.attributes.iter() {
         // println!("{} -> {}", attribut.attribute_type, result.is_defined_typename(&attribut.attribute_type));
         if result.is_defined_typename(&attribut.attribute_type) 
            || model::Type::is_basic_type(&attribut.attribute_type) {  
      str.push_str("\n        this.");
      str.push_str(&util::lsnake_to_ucamel(&attribut.name));
      str.push_str(" = ");
      str.push_str(&get_dotnet_type_initial(&attribut.attribute_type, attribut.is_array));
      str.push_str(";");
        } 
    str.push_str("        ");
    } 
  str.push_str("\n    }\n\n    ///\n    /// The function IsValid offert a validation function for the\n    /// mandatory attributes and other constraints of staticjson code\n    /// <param name=\"obj\">object to check</param>\n    /// <returns>check result as bool</returns>\n    ///\n    public static bool IsValid(");
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


