// Template for generation of c based json parsers
use model;
use filehandler;
use util;

// 
// This generat Method is the entry point of code generation
//
pub fn generate(types:&Box<Vec<Box<model::Type>>>, folder:&str) {
  let mut str:String = String::new(); 
  str.push_str("// generated c code\n\n#include <stdio.h>\n#include <stdlib.h>\n#include <string.h>\n\n// TODO: generate some more code\n\n// data structures");
for typ in (*types).iter() { 
    str.push_str("\nstruct ");
    str.push_str(&util::ucamel_to_lsnake(&typ.typename));
    str.push_str(" {\n  // TODO: Attributes\n};\n");
} 
  str.push_str("\n// end of data structures\n");
  // write it to one single file 
  let filename = format!("{}/jsonparser.c", folder);
  filehandler::write_file(filename, str);
} 
