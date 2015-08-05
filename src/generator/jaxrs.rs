use model;
use filehandler;
use util;

//
// This generate Method is the entry point to generation
// of html documentation about the given types
//
pub fn generate(tree:model::ParserResult, folder:&str) {
  for typ in tree.types.iter() {
    let result = gen_type(typ);
    let filename = format!("{}/entites/{}.java", folder, typ.typename);
    filehandler::write_file(filename, result);
  }
  
  for ifa in tree.interfaces.iter() {
    let result = gen_interface(ifa);
    let filename = format!("{}/interfaces/I{}.java", folder, ifa.name);
    filehandler::write_file(filename, result);
  }
} 
// 
// Generate code for type
//
fn gen_type(typ:&Box<model::Type>) -> String {
  let mut str:String = String::new(); 

  str.push_str("package entities\n\nimport java.util.ArrayList;\n\n/**\n* generated type for Entity ");
  str.push_str(&typ.typename);
  str.push_str("\n*/\n\n@Entity\npublic class");
  str.push_str(&typ.typename);
  str.push_str(" {");
    for attribut in typ.attributes.iter() { 
    str.push_str("\n    private TYPEFOR(");
    str.push_str(&attribut.attribute_type);
    str.push_str(") ");
    str.push_str(&util::lsnake_to_lcamel(&attribut.name));
    str.push_str(";   ");
    } 
  str.push_str("\n}");
  return str;
} 


// 
// Generate code for interface
//
fn gen_interface(ifa:&Box<model::Interface>) -> String {
  let mut str:String = String::new();

  str.push_str("package interfaces;\n\nimport java.util.ArrayList;\n// ...\n\n/**\n* Generated Interface for ");
  str.push_str(&ifa.name);
  str.push_str(" with JAX-RS Annotations\n*/");
  return str;
} 
