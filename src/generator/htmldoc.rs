use model;
use filehandler;
use util;

//
// This generate Method is the entry point to generation
// of html documentation about the given types
//
pub fn generate(types:&Box<Vec<Box<model::Type>>>, folder:&str) {
  for typ in (*types).iter() {
    let result = gen_type(typ);
    let filename = format!("{}/{}.html", folder, typ.typename);
    filehandler::write_file(filename, result);
  }
}

// 
// Generate code for type
//
fn gen_type(typ:&Box<model::Type>) -> String {
  let mut str:String = String::new(); 

  str.push_str("<html>\n<head><title>Documentation about Type: ");
  str.push_str(&typ.typename);
  str.push_str(" </title></head>\n<body>\n<h1>Dokumentation zu den SimpleJSON-Typen</h1>\n<h2>Typ: ");
  str.push_str(&typ.typename);
  str.push_str("</h2>\n<ul>\n<li>PHP:");
  str.push_str(&util::ucamel_to_lsnake(&typ.typename));
  str.push_str("</li>\n</ul>");
    for attribut in typ.attributes.iter() { 
    str.push_str("\n<h3>Attribut: ");
    str.push_str(&attribut.name);
    str.push_str("</h3>\n<ul>\n  <li>Java: ");
    str.push_str(&util::lsnake_to_lcamel(&attribut.name));
    str.push_str("</li>\n  <li>C# Property: ");
    str.push_str(&util::lsnake_to_ucamel(&attribut.name));
    str.push_str("</li>\n</ul>\n<ul>");
      if model::Type::is_basic_type(&attribut.attribute_type) { 
      str.push_str("\n  <li>Typ: ");
      str.push_str(&attribut.attribute_type);
      str.push_str("</li>");
} else { 
      str.push_str(" \n  <li><a href=\"./");
      str.push_str(&attribut.attribute_type);
      str.push_str(".html\">Typ:");
      str.push_str(&attribut.attribute_type);
      str.push_str("</a></li>");
} 
    str.push_str(" \n  <li>Attribut is Array:");
if attribut.is_array { str.push_str("true"); } else { str.push_str("false"); } 
    str.push_str(" </li>");
      for param in attribut.params.iter() { 
      str.push_str("\n  <li>");
      str.push_str(&param.name);
      str.push_str(":");
      str.push_str(&param.value);
      str.push_str("</li>");
      } 
    str.push_str("\n</ul>");
    } 
  str.push_str("\n</body>\n</html>");
  return str;
} 
