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

  str.push_str("// Namespace generieren\n");
  str.push_str(&buildup_js_namespace_from_ifa(ifa));
  str.push_str("\n\n/**\n* Generated Proxy for ");
  str.push_str(&ifa.name);
  str.push_str("\n*/\n");
  str.push_str(&get_js_namespace_from_ifa(ifa));
  str.push_str(".");
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

  str.push_str("\n        // HTTP-POST call  \n        var method = \"POST\";\n        var queryParams = \"\";");
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
  str.push_str(" \n        if(queryParams.length > 0) {\n            path = path + \"?\" + queryParams;\n        }        \n        // DEBUG OUTPUT:\n        console.log(method + \" \" + path);\n        \n        $.ajax({\n            \"url\": path,\n            \"method\": method,\n            \"contentType\":'application/json; charset=UTF-8',\n           ");
if(f.has_serialized_functionparam()) { 
            
    str.push_str(" \"data\": JSON.stringify(");
    str.push_str(&f.get_serialized_functionparam_name());
    str.push_str(")");
str.push_str(", ");  
               } 
  str.push_str("\n            \"dataType\": \"json\",\n            \"success\": successHandler,\n            \"error\": errorHandler\n        });");

  return str;
}



fn get_impl_for_put_function(f:&model::Function) -> String {
  let mut str:String = String::new();

  str.push_str("\n        // HTTP-PUT call  \n        var method = \"PUT\";\n        var queryParams = \"\";");
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
  str.push_str(" \n        if(queryParams.length > 0) {\n            path = path + \"?\" + queryParams;\n        }        \n        // DEBUG OUTPUT:\n        console.log(method + \" \" + path);\n        \n        $.ajax({\n            \"url\": path,\n            \"method\": method,\n            \"contentType\":'application/json; charset=UTF-8',\n           ");
if(f.has_serialized_functionparam()) { 
            
    str.push_str(" \"data\": JSON.stringify(");
    str.push_str(&f.get_serialized_functionparam_name());
    str.push_str(")");
str.push_str(", ");  
               } 
  str.push_str("\n            \"dataType\": \"json\",\n            \"success\": successHandler,\n            \"error\": errorHandler\n        });");

  return str;
}



fn get_impl_for_delete_function(f:&model::Function) -> String {
  let mut str:String = String::new();

  str.push_str("\n        // HTTP-DELETE call    \t\n        var method = \"DELETE\";\n        var queryParams = \"\";");
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



fn buildup_js_namespace_from_type(t:&model::Type) -> String {
    if t.is_param_present("js-namespace") {
        let ns = t.get_param_value("js-namespace");
        return buildup_js_namespace(&ns);
    } else {
        let mut str:String = String::new();
        str.push_str("var model = model || {};");
        return str;
    }
}

fn buildup_js_namespace_from_ifa(i:&model::Interface) -> String {
    if i.is_param_present("js-namespace") {
        let ns = i.get_param_value("js-namespace");
        return buildup_js_namespace(&ns);
    } else {
        let mut str:String = String::new();
        str.push_str("var proxy = proxy || {};");
        return str;
    }
}

fn buildup_js_namespace(s:&str) -> String {
    let mut str:String = String::new();
    let mut ns:String = String::new();
    let mut split = s.split(".");
    let mut i = 0;
    
    for token in split {        
        if i == 0 {
            ns.push_str(token);
            
            str.push_str("var ");
            str.push_str(&ns);
            str.push_str(" = ");
            str.push_str(&ns);
            str.push_str(" || {};\n");
        } else {
            ns.push_str(".");
            ns.push_str(token);
            
            str.push_str(&ns);
            str.push_str(" = ");
            str.push_str(&ns);
            str.push_str(" || {};\n");
        }
        i += 1;
    }    
    
    // str.push_str(s);
    return str;
}

fn get_js_namespace_from_type(i:&model::Type) -> String {
    let mut str:String = String::new();
    if i.is_param_present("js-namespace") {
        let ns = i.get_param_value("js-namespace");
        str.push_str(&ns);
    } else {        
        str.push_str("model");        
    }
    return str;
}

fn get_js_namespace_from_ifa(i:&model::Interface) -> String {
    let mut str:String = String::new();
    if i.is_param_present("js-namespace") {
        let ns = i.get_param_value("js-namespace");
        str.push_str(&ns);
    } else {        
        str.push_str("proxy");        
    }
    return str;
}


