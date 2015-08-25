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
    if typ.is_param_present("java-package") {
      let package = typ.get_param_value("java-package").replace(".", "/");
      let filename = format!("{}/{}/{}.java", folder, package, typ.typename);
      filehandler::write_file(filename, result);
    } else {
      let filename = format!("{}/{}.java", folder, typ.typename);
      filehandler::write_file(filename, result);
    }
  }
  
  for ifa in tree.interfaces.iter() {
    let result = gen_proxy(ifa, tree.types.clone());
    if ifa.is_param_present("java-package") {
      let package = ifa.get_param_value("java-package").replace(".", "/");
      let filename = format!("{}/{}/{}Proxy.java", folder, package, ifa.name);
      filehandler::write_file(filename, result);
    } else {
      let filename = format!("{}/{}.java", folder, ifa.name);
      filehandler::write_file(filename, result);
    }
  }
} 
// 
// Generate code for type
//
fn gen_type(typ:&Box<model::Type>, types:Box<Vec<Box<model::Type>>>) -> String {
  let mut str:String = String::new(); 
  if typ.is_param_present("java-package") {

    str.push_str("package ");
    str.push_str(&typ.get_param_value("java-package"));
    str.push_str(";");
} 
  str.push_str("\n\nimport java.util.ArrayList;\nimport java.io.Serializable;");
  str.push_str(&get_types_referenced_java_packages(&typ, types.clone()));
  str.push_str("\n\n/**\n* Generated Type for Entity ");
  str.push_str(&typ.typename);
  str.push_str(" \n*/\npublic class ");
  str.push_str(&typ.typename);
  str.push_str(" implements Serializable {\n\n  private static final long serialVersionUID = 1L;\n");
    for attribut in typ.attributes.iter() { 
    str.push_str("\n    private ");
    str.push_str(&get_java_type(&attribut.attribute_type, attribut.is_array));
    str.push_str(" ");
    str.push_str(&util::lsnake_to_lcamel(&attribut.name));
    str.push_str(";   ");
    } 
  str.push_str("\n\n    public ");
  str.push_str(&typ.typename);
  str.push_str("() {");
    for attribut in typ.attributes.iter() { 
    str.push_str("\n        this.");
    str.push_str(&util::lsnake_to_lcamel(&attribut.name));
    str.push_str(" = ");
    str.push_str(&get_java_type_initial(&attribut.attribute_type, attribut.is_array));
    str.push_str(";");
    } 
  str.push_str("\n    }");
    for attribut in typ.attributes.iter() { 
    str.push_str("\n\n    public ");
    str.push_str(&get_java_type(&attribut.attribute_type, attribut.is_array));
    str.push_str(" get");
    str.push_str(&util::lsnake_to_ucamel(&attribut.name));
    str.push_str("() {\n        return this.");
    str.push_str(&util::lsnake_to_lcamel(&attribut.name));
    str.push_str(";\n    }\n    \n    public void set");
    str.push_str(&util::lsnake_to_ucamel(&attribut.name));
    str.push_str("(");
    str.push_str(&get_java_type(&attribut.attribute_type, attribut.is_array));
    str.push_str(" value) {\n        this.");
    str.push_str(&util::lsnake_to_lcamel(&attribut.name));
    str.push_str(" = value;\n    }");
    } 
  str.push_str("\n\n    /**\n    * The function isValid offert a validation function for the\n    * mandatory attributes and other constraints of staticjson code\n    * @param object to check\n    * @return check result\n    */\n    public static boolean isValid(");
  str.push_str(&typ.typename);
  str.push_str(" obj) {\n        return obj != null");
    for attribut in typ.attributes.iter() { 
      if attribut.is_param_value_present("mandatory", "true") { 
      str.push_str("\n        && obj.");
      str.push_str(&util::lsnake_to_lcamel(&attribut.name));
      str.push_str(" != ");
      str.push_str(&get_java_type_initial(&attribut.attribute_type, attribut.is_array));
      str.push_str("");
      } if attribut.is_param_present("maxlen") && attribut.attribute_type == "string" && !attribut.is_array { 
      str.push_str("\n        && (obj.");
      str.push_str(&util::lsnake_to_lcamel(&attribut.name));
      str.push_str(" != null && \n            obj.");
      str.push_str(&util::lsnake_to_lcamel(&attribut.name));
      str.push_str(".length() <= ");
      str.push_str(&attribut.get_param_value("maxlen"));
      str.push_str(")");
      } if attribut.is_param_present("minlen") && attribut.attribute_type == "string" && !attribut.is_array { 
      str.push_str("\n        && (obj.");
      str.push_str(&util::lsnake_to_lcamel(&attribut.name));
      str.push_str(" != null && \n            obj.");
      str.push_str(&util::lsnake_to_lcamel(&attribut.name));
      str.push_str(".length() >= ");
      str.push_str(&attribut.get_param_value("minlen"));
      str.push_str(")");
      } 
    } 
  str.push_str(";\n    }\n}");
  return str;
} 


// 
// Generate code for interface
//
fn gen_proxy(ifa:&Box<model::Interface>, types:Box<Vec<Box<model::Type>>>) -> String {
  let mut str:String = String::new();
  if ifa.is_param_present("java-package") {

    str.push_str("package ");
    str.push_str(&ifa.get_param_value("java-package"));
    str.push_str(";");
} 
  str.push_str("\n\nimport java.util.ArrayList; ");
  str.push_str(&get_proxies_referenced_java_packages(&ifa, types.clone()));
  str.push_str("\n\n/**\n* Generated Proxy for ");
  str.push_str(&ifa.name);
  str.push_str("\n*/\npublic class ");
  str.push_str(&ifa.name);
  str.push_str("Proxy {");
for function in ifa.functions.iter() { 
    str.push_str("\n\n    /**");
for param in function.params.iter() { 
      str.push_str("\n     * @param ");
      str.push_str(&param.name);
      str.push_str("");
} 
    str.push_str(" \n     * @return ");
    str.push_str(&get_java_type(&function.returntype, function.returntype_is_array));
    str.push_str("\n     */ \n    public ");
    str.push_str(&get_java_type(&function.returntype, function.returntype_is_array));
    str.push_str(" ");
    str.push_str(&function.name);
    str.push_str("(");
let mut i = 0;
for param in function.params.iter() { 
  i = i+1;   
  if i > 1 { 
    str.push_str(", "); 
  } 
      str.push_str("");
      str.push_str(&get_java_type(&param.typename, param.is_array));
      str.push_str(" ");
      str.push_str(&param.name);
      str.push_str("");
} 
    str.push_str(") {\n        // ... TODO ...\n    }");
} 
  str.push_str("\n}");
  return str;
} 
// rust utility functions for jaxrs 

fn get_java_type(sjtype:&str, is_array:bool) -> String {
  let mut jtype:&str;
  if !model::Type::is_basic_type(sjtype) {
    if is_array {
      return format!("ArrayList<{}>", sjtype);
    } else {
      jtype = sjtype;
    }
  } else if sjtype == "int" || sjtype == "uint" {
    if is_array {
      jtype = "ArrayList<Integer>";
    } else {
      jtype = "int";
    }
  } else if sjtype == "long" || sjtype == "ulong" {
    if is_array {
      jtype = "ArrayList<Long>";
    } else {
      jtype = "long";
    }
  } else if sjtype == "string" {
    if is_array {
      jtype = "ArrayList<String>";
    } else {
      jtype = "String";
    }
  } else if sjtype == "decimal" {
    if is_array {
      jtype = "ArrayList<double>";
    } else {
      jtype = "double";
    }
  } else if sjtype == "date" {
    if is_array {
      jtype = "ArrayList<java.sql.Date>";
    } else {
      jtype = "java.sql.Date";
    }
  } else {
    jtype = "undef";
  }
  return jtype.to_string();
}

fn get_java_type_initial(sjtype:&str, is_array:bool) -> String {
  let mut jtype:&str;
  if !model::Type::is_basic_type(sjtype) {
    if is_array {
      return format!("new ArrayList<{}>()", sjtype);
    } else {
      jtype = "null";
    }
  } else if sjtype == "int" || sjtype == "uint" {
    if is_array {
      jtype = "new ArrayList<Integer>()";
    } else {
      jtype = "0";
    }
  } else if sjtype == "long" || sjtype == "ulong" {
    if is_array {
      jtype = "new ArrayList<Long>()";
    } else {
      jtype = "0";
    }
  } else if sjtype == "string" {
    if is_array {
      jtype = "new ArrayList<String>()";
    } else {
      jtype = "null";
    }
  } else if sjtype == "decimal" {
    if is_array {
      jtype = "new ArrayList<Double>()";
    } else {
      jtype = "0.0d";
    }
  } else if sjtype == "date" {
    if is_array {
      jtype = "new ArrayList<java.sql.Date>()";
    } else {
      jtype = "null";
    }
  } else {
    jtype = "undef";
  }
  return jtype.to_string();
}

fn get_types_referenced_java_packages(typ:&Box<model::Type>, types:Box<Vec<Box<model::Type>>>) -> String {    
  let mut package_set:HashSet<String> = HashSet::new();
  for attr in typ.attributes.iter() {
    if !model::Type::is_basic_type(&attr.attribute_type) {
      for t in types.iter() {
        if t.typename == attr.attribute_type
           && t.is_param_present("java-package") 
           && !(typ.is_param_present("java-package") 
                && typ.get_param_value("java-package") == t.get_param_value("java-package")){
           package_set.insert(format!("{}.{}", t.get_param_value("java-package"), t.typename));
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

fn get_interfaces_referenced_java_packages(ifa:&Box<model::Interface>, types:Box<Vec<Box<model::Type>>>) -> String {    
  let mut package_set:HashSet<String> = HashSet::new();
  for func in ifa.functions.iter() {
    if !model::Type::is_basic_type(&func.returntype) && func.returntype != "void" {
      package_set.insert(format!("javax.ws.rs.Consumes"));
      for t in types.iter() {
        if t.typename == func.returntype
           && t.is_param_present("java-package") 
           && !(ifa.is_param_present("java-package") 
                && ifa.get_param_value("java-package") == t.get_param_value("java-package")){
           package_set.insert(format!("{}.{}", t.get_param_value("java-package"), t.typename));
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
             && t.is_param_present("java-package") 
             && !(ifa.is_param_present("java-package") 
                  && ifa.get_param_value("java-package") == t.get_param_value("java-package")){
             package_set.insert(format!("{}.{}", t.get_param_value("java-package"), t.typename));
          }
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

fn get_proxies_referenced_java_packages(ifa:&Box<model::Interface>, types:Box<Vec<Box<model::Type>>>) -> String {    
  let mut package_set:HashSet<String> = HashSet::new();
  for func in ifa.functions.iter() {
    if !model::Type::is_basic_type(&func.returntype) && func.returntype != "void" {
      for t in types.iter() {
        if t.typename == func.returntype
           && t.is_param_present("java-package") 
           && !(ifa.is_param_present("java-package") 
                && ifa.get_param_value("java-package") == t.get_param_value("java-package")){
           package_set.insert(format!("{}.{}", t.get_param_value("java-package"), t.typename));
        }
      }
    }
    for param in func.params.iter() {
      if !model::Type::is_basic_type(&param.typename) {
        for t in types.iter() {
          if t.typename == param.typename
             && t.is_param_present("java-package") 
             && !(ifa.is_param_present("java-package") 
                  && ifa.get_param_value("java-package") == t.get_param_value("java-package")){
             package_set.insert(format!("{}.{}", t.get_param_value("java-package"), t.typename));
          }
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


