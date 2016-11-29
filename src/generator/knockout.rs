use model;
use filehandler;
use util;
use std::collections::HashSet;

//
// This generate Method is the entry point to generation
// of knockout-JS Model-Prototypes for the given types
//
pub fn generate(tree:model::ParserResult, folder:&str) {
  let mut code:String = String::new();

  for typ in tree.types.iter() {
    if typ.is_param_value_present("knockout-js", "true") {
      let result = gen_type(typ, tree.types.clone());  
      code.push_str(&result);
      code.push_str("\n");    
    }
  }

  code.push_str(&generate_main_model(tree.types.clone()));

  let filename = format!("{}/knockout_model.js", folder);
  filehandler::write_file(filename, code);    
} 
/** 
 * staticjson Code-Generation for Types
 */
fn gen_type(typ:&Box<model::Type>, types:Box<Vec<Box<model::Type>>>) -> String {
  let mut str:String = String::new(); 
  

  str.push_str("// Knockout-Model für ");
  str.push_str(&typ.typename);
  str.push_str("\n// Dieser Code wurde mit staticjson generiert \n// Namespace:\n");
  str.push_str(&buildup_js_namespace_from_type(typ));
  str.push_str("\n\n");
  str.push_str(&get_js_namespace_from_type(typ));
  str.push_str(".");
  str.push_str(&typ.typename);
  str.push_str(" = function(data) {\n    var self = this;\n    if(!!data) {");
       for attribut in typ.attributes.iter() { 
            let mut temp:String = String::new();
            temp.push_str("data."); 
            temp.push_str(&util::lsnake_to_ucamel(&attribut.name)); 
    str.push_str("                \n        self.");
    str.push_str(&util::lsnake_to_ucamel(&attribut.name));
    str.push_str(" = ");
    str.push_str(&get_initializer_for_attribute_and_value(&attribut, &temp));
    str.push_str(";");
       } 
  str.push_str("\n    } else {");
       for attribut in typ.attributes.iter() { 
    str.push_str("\n        self.");
    str.push_str(&util::lsnake_to_ucamel(&attribut.name));
    str.push_str(" = ");
    str.push_str(&get_initializer_for_attribute(&attribut));
    str.push_str(";");
       } 
  str.push_str("\n    }\n};\n");
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


// Function for generating the main model
fn generate_main_model(types:Box<Vec<Box<model::Type>>>) -> String {
    let mut str:String = String::new(); 
  str.push_str("\nvar MainModel = function() {\n    var self = this;");
   for typ in types.iter() { 
        if typ.is_param_value_present("knockout-in-main", "true") { 
      str.push_str("\n    self.");
      str.push_str(&util::ucamel_to_lsnake(&typ.typename));
      str.push_str("_selected = ko.observable(new ");
      str.push_str(&get_js_namespace_from_type(typ));
      str.push_str(".");
      str.push_str(&typ.typename);
      str.push_str("());\n    self.");
      str.push_str(&util::ucamel_to_lsnake(&typ.typename));
      str.push_str("_list = ko.observableArray([]);");
       }
    } 
  str.push_str("    \n};    ");
    return str;
} 

// Helper functions for knockout template generation
fn get_initializer_for_attribute(attr:&model::Attribute) -> String {
    let mut str:String = String::new();
    if attr.is_array {
        str.push_str("ko.observableArray([])");
    } else if attr.attribute_type == "string" {
        str.push_str("ko.observable(\"\")");
    } else if attr.attribute_type == "bool" {
        str.push_str("ko.observable(false)");
    } else if attr.attribute_type == "date" {
        str.push_str("ko.observable()");
    } else if attr.attribute_type == "time" {
        str.push_str("ko.observable()");
    } else if attr.attribute_type == "datetime" {
        str.push_str("ko.observable()");                
    } else if model::Type::is_basic_type(&attr.attribute_type) {
        str.push_str("ko.observable(0)");
    } else {
        str.push_str("ko.observable()");
    }
    return str;
} 

fn get_initializer_for_attribute_and_value(attr:&model::Attribute, string:&str) -> String {
    let mut str:String = String::new();
    if attr.is_array {
        str.push_str("ko.observableArray([])");
    } else if model::Type::is_basic_type(&attr.attribute_type) {
        str.push_str("ko.observable(");
        str.push_str(string);
        str.push_str(")"); 
    } else {
        // TODO: hier muss rekursiv abgestiegen werden: new mit Attribut-Typ!
        //       dafür brauch ich hier aber die Liste der Typen, denn sonst bekomm
        //       ich den js-namespace nicht für den Typ und kann so kein new machen!
        str.push_str("ko.observable()");
    }
    return str;
} 


