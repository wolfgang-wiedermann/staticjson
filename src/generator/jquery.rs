use model;
use filehandler;
use util;
use std::collections::HashSet;

//
// This generate Method is the entry point to generation
// of jquery proxies for the given interfaces and types
//
pub fn generate(tree:model::ParserResult, folder:&str) {
/*
  for typ in tree.types.iter() {
    let result = gen_type(typ, tree.types.clone());
    let filename = format!("{}/{}.js", folder, typ.typename);
    filehandler::write_file(filename, result);    
  }
*/
  
  for ifa in tree.interfaces.iter() {
    let result = gen_proxy(ifa, tree.types.clone());
    let filename = format!("{}/{}.js", folder, ifa.name);
    filehandler::write_file(filename, result);    
  }
} 
// 
// Generate code for interface
//
fn gen_proxy(ifa:&Box<model::Interface>, types:Box<Vec<Box<model::Type>>>) -> String {
  let mut str:String = String::new();
  if ifa.is_param_present("java-package") {

    str.push_str("// TODO: Namespace anstelle von \"proxy\" aufbauen: ");
    str.push_str(&ifa.get_param_value("java-package"));
    str.push_str(";");
} 
  str.push_str("\nvar proxy = proxy || {};\n\n/**\n* Generated Proxy for ");
  str.push_str(&ifa.name);
  str.push_str("\n*/\nproxy.");
  str.push_str(&ifa.name);
  str.push_str("Proxy = function(urlBase) {\n    var self = this;\n\n    // URL-Basis aufbauen\n    self.url = urlBase;");
if ifa.is_param_present("path") { 
    str.push_str("\n    self.url += \"");
    str.push_str(&ifa.get_param_value("path"));
    str.push_str("\";  ");
} 
  str.push_str("\n    ");
for function in ifa.functions.iter() { 
    str.push_str("\n\n    /**");
for param in function.params.iter() { 
      str.push_str("\n     * @param ");
      str.push_str(&param.name);
      str.push_str("");
} 
    str.push_str(" \n     * @return ");
    str.push_str(&function.returntype);
    str.push_str("\n     */ \n    self.");
    str.push_str(&function.name);
    str.push_str(" = function(");
for param in function.params.iter() { 
      str.push_str(&param.name);
str.push_str(", ");  } 
    str.push_str("successHandler, errorHandler) { ");
    if function.is_attribute_value_present("method", "GET") {
    str.push_str(&get_impl_for_get_function(&function)) 
  } else if function.is_attribute_value_present("method", "POST") {
    str.push_str(&get_impl_for_post_function(&function)) 
  } else if function.is_attribute_value_present("method", "PUT") {
    str.push_str(&get_impl_for_put_function(&function))
  } else if function.is_attribute_value_present("method", "DELETE") {
    str.push_str(&get_impl_for_delete_function(&function))
  } 
    str.push_str("\n    }");
} 
  str.push_str("\n}");
  return str;
} 

fn get_impl_for_get_function(f:&model::Function) -> String {
  let mut str:String = String::new();

  str.push_str("\n        // HTTP-GET call    \n        var method = \"GET\";\n        var queryParams = \"\";");
if f.is_attribute_present("path") { 
    str.push_str(" \n        var path = self.url + \"");
    str.push_str(&f.get_attribute_value("path"));
    str.push_str("\";");
} else { 
    str.push_str("\n        var path = self.url;");
} 
  str.push_str("\n");
for param in f.params.iter() { 
    if param.is_param_present("path-param") { 
      str.push_str("        \n\t    path = path.replace(\"{");
      str.push_str(&param.get_param_value("path-param"));
      str.push_str("}\", encodeURIComponent(");
      str.push_str(&param.name);
      str.push_str("));");
    }
} 
  for param in f.params.iter() {
    if param.is_param_present("query-param") { 
      str.push_str("\n        if(queryParams.length > 0) {\n            queryParams += \"&\";\n        }                \n        queryParams += \"");
      str.push_str(&param.get_param_value("query-param"));
      str.push_str("=\" + encodeURIComponent(");
      str.push_str(&param.name);
      str.push_str(");            ");
    }
  }  
  str.push_str(" \n        if(queryParams.length > 0) {\n            path = path + \"?\" + queryParams;\n        }        \n        // DEBUG OUTPUT:\n        console.log(method + \" \" + path);\n        \n        $.ajax({\n            \"url\": path,\n            \"method\": method,\n            \"dataType\": \"json\",\n            \"success\": successHandler,\n            \"error\": errorHandler\n        });");

  return str;
}

fn get_impl_for_post_function(f:&model::Function) -> String {
  let mut str:String = String::new();

  str.push_str("\n        // HTTP-POST call  \n        var method = \"POST\";");
if f.is_attribute_present("path") { 
    str.push_str(" \n        var path = self.url + \"");
    str.push_str(&f.get_attribute_value("path"));
    str.push_str("\";");
} 
  str.push_str("");
for param in f.params.iter() { 
    if param.is_param_present("path-param") { 
      str.push_str("\n            // TODO:\n\t    \t// path = path.replaceAll(\"\\\\{");
      str.push_str(&param.get_param_value("path-param"));
      str.push_str("\\\\}\",\"\"+");
      str.push_str(&param.name);
      str.push_str(");");
    }
} 
  for param in f.params.iter() {
    if param.is_param_present("query-param") { 
      str.push_str("\n            // path.addQueryParameter(\"");
      str.push_str(&param.get_param_value("query-param"));
      str.push_str("\", ");
      str.push_str(&param.name);
      str.push_str(");");
    }
  }  
  return str;
}

fn get_impl_for_put_function(f:&model::Function) -> String {
  let mut str:String = String::new();

  str.push_str("\n        // HTTP-PUT call  \n        var method = \"PUT\";");
if f.is_attribute_present("path") { 
    str.push_str(" \n        var path = self.url + \"");
    str.push_str(&f.get_attribute_value("path"));
    str.push_str("\";");
} 
  str.push_str("");
for param in f.params.iter() { 
    if param.is_param_present("path-param") { 
      str.push_str("\n            // TODO:\n\t    \t// path = path.replaceAll(\"\\\\{");
      str.push_str(&param.get_param_value("path-param"));
      str.push_str("\\\\}\",\"\"+");
      str.push_str(&param.name);
      str.push_str(");");
    }
} 
  for param in f.params.iter() {
    if param.is_param_present("query-param") { 
      str.push_str("\n            // path.addQueryParameter(\"");
      str.push_str(&param.get_param_value("query-param"));
      str.push_str("\", ");
      str.push_str(&param.name);
      str.push_str(");");
    }
  }  
  return str;
}

fn get_impl_for_delete_function(f:&model::Function) -> String {
  let mut str:String = String::new();

  str.push_str("\n        // HTTP-DELETE call    \t\n        var method = \"DELETE\";");
if f.is_attribute_present("path") { 
    str.push_str(" \n        var path = self.url + \"");
    str.push_str(&f.get_attribute_value("path"));
    str.push_str("\";");
} 
  str.push_str("");
for param in f.params.iter() { 
    if param.is_param_present("path-param") { 
      str.push_str("\n            // TODO:\n\t    \t// path = path.replaceAll(\"\\\\{");
      str.push_str(&param.get_param_value("path-param"));
      str.push_str("\\\\}\",\"\"+");
      str.push_str(&param.name);
      str.push_str(");");
    }
} 
  for param in f.params.iter() {
    if param.is_param_present("query-param") { 
      str.push_str("\n            // path.addQueryParameter(\"");
      str.push_str(&param.get_param_value("query-param"));
      str.push_str("\", ");
      str.push_str(&param.name);
      str.push_str(");");
    }
  }  
  return str;
}


