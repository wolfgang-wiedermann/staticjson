// Template for generation of c based json parsers
use model;
use filehandler;
use util;



//
// Generates Files for Utilities
//
fn generate_buffer_code(folder:&str) {
  generate_sj_buffer_c(folder);
  generate_sj_buffer_h(folder);
}

//
// Buffer-Code-File
//
fn generate_sj_buffer_c(folder:&str) {
  let mut str:String = String::new();
  
  str.push_str("#include \"sj_buffer.h\"\n#include <stdlib.h>\n#include <string.h>\n\n// Defines the steps for increasing the buffer size\n#define BUFSIZE_STEP 1024\n\n/*\n * Create a new buffer instance\n */\nSjBuffer* sj_buffer_new() {\n    SjBuffer *buf = (SjBuffer*)malloc(sizeof(SjBuffer*));\n    buf->buf = (char*)malloc(sizeof(char)*BUFSIZE_STEP);\n    buf->length = 0;\n    buf->buffer_size = BUFSIZE_STEP;\n    return buf;\n}\n\n/*\n * Add a char to a buffer\n */\nvoid sj_buffer_push(SjBuffer *buf, char c) {\n    if(buf->length+2 >= buf->buffer_size) {\n        // Increase buffer size\n        char *b2 = (char*)malloc(sizeof(char)*(buf->buffer_size+BUFSIZE_STEP));\n        strncpy(b2, buf->buf, buf->buffer_size);\n        buf->buffer_size += BUFSIZE_STEP;\n        free(buf->buf);\n        buf->buf = b2;\n    } \n    // Add a char to the buffer\n    buf->length += 1;\n    buf->buf[buf->length-1] = c;\n    buf->buf[buf->length] = '\\0';\n}\n\n/*\n * Get the used size of the buffer\n */\nint sj_buffer_get_size(SjBuffer *buf) {\n    return buf->length;\n}\n\n/*\n * Gets the content of the given buffer \n */\nchar* sj_buffer_get_content(SjBuffer *buf) { \n    char *tmp_buf = (char*)malloc(sizeof(char)*(buf->length+1));\n    strncpy(tmp_buf, buf->buf, (buf->length+1));\n    return tmp_buf;\n}\n\n/*\n * Clean content of buffer\n */\nvoid sj_buffer_clean(SjBuffer *buf) {\n    buf->length = 0;\n    buf->buf[0] = '\\0';\n    buf->buf[1] = '\\0';\n}\n\n/*\n * Free Buffer from Memory\n */\nvoid sj_buffer_free(SjBuffer *buf) {\n    free(buf->buf);\n    free(buf);\n}");

  let filename = format!("{}/{}", folder, "sj_buffer.c");
  let result = format!("{}", str);
  filehandler::write_file(filename, result);
}

//
// Buffer-Header-File
//
fn generate_sj_buffer_h(folder:&str) {
  let mut str:String = String::new();
  
  str.push_str("#ifndef SJ_BUFFER_H\n#define SJ_BUFFER_H\n\ntypedef struct sj_buffer {\n    char *buf;\n    int length;\n    int buffer_size;\n} SjBuffer;\n\n/*\n * Create a new buffer instance\n */\nSjBuffer* sj_buffer_new();\n\n/*\n * Add a char to a buffer\n */\nvoid sj_buffer_push(SjBuffer *buf, char c);\n\n/*\n * Free Buffer from Memory\n */\nvoid sj_buffer_free(SjBuffer *buf);\n\n/*\n * Clean Content of Buffer\n */\nvoid sj_buffer_clean(SjBuffer *buf);\n\n/*\n * Get the used size of the buffer\n */\nint sj_buffer_get_size(SjBuffer *buf);\n\n/*\n * Gets a copy of the content of the given buffer \n */\nchar* sj_buffer_get_content(SjBuffer *buf);\n\n#endif");

  let filename = format!("{}/{}", folder, "sj_buffer.h");
  let result = format!("{}", str);
  filehandler::write_file(filename, result);
}


fn generate_header(types:&Box<Vec<Box<model::Type>>>, folder:&str) {
  let mut str:String = String::new(); 


  str.push_str("/*\n* C code generated by staticjson\n*/\n#ifndef JSONINC_PARSER_H\n#define JSONINC_PARSER_H\n");
/*
 * Generation C structs for staticjson types
 */
for typ in (*types).iter() { 
    str.push_str("\ntypedef struct ");
    str.push_str(&util::ucamel_to_lsnake(&typ.typename));
    str.push_str(" {");
 for attribut in typ.attributes.iter() {
    if attribut.is_array { 
      if attribut.attribute_type == "string" { 
          str.push_str("\n        char **");
          str.push_str(&attribut.name);
          str.push_str(";\n        int ");
          str.push_str(&attribut.name);
          str.push_str("_length;");
} else if attribut.attribute_type == "int" { 
          str.push_str("\n        int *");
          str.push_str(&attribut.name);
          str.push_str(";");
} else if attribut.attribute_type == "uint" { 
          str.push_str("\n        unsigned int *");
          str.push_str(&attribut.name);
          str.push_str(";");
} else if attribut.attribute_type == "long" { 
          str.push_str("\n        long *");
          str.push_str(&attribut.name);
          str.push_str(";");
} else if attribut.attribute_type == "ulong" { 
          str.push_str("\n        unsigned long *");
          str.push_str(&attribut.name);
          str.push_str(";");
} else if attribut.attribute_type == "decimal" { 
          str.push_str("\n        double *");
          str.push_str(&attribut.name);
          str.push_str(";");
} else if attribut.attribute_type == "date" { 
          str.push_str("\n        // ERROR: at ");
          str.push_str(&attribut.name);
          str.push_str("\n        // Date is currently unsupported in C Parsers");
} else { 
        // TODO: date, datetime and time missing !!! 
          str.push_str("\n        ");
          str.push_str(&attribut.attribute_type);
          str.push_str(" **");
          str.push_str(&attribut.name);
          str.push_str(";");
}
    } else {
    if attribut.attribute_type == "string" { 
          str.push_str("\n        char *");
          str.push_str(&attribut.name);
          str.push_str(";\n        int ");
          str.push_str(&attribut.name);
          str.push_str("_length;");
} else if attribut.attribute_type == "int" { 
          str.push_str("\n        int ");
          str.push_str(&attribut.name);
          str.push_str(";");
} else if attribut.attribute_type == "uint" { 
          str.push_str("\n        unsigned int ");
          str.push_str(&attribut.name);
          str.push_str(";");
} else if attribut.attribute_type == "long" { 
          str.push_str("\n        long ");
          str.push_str(&attribut.name);
          str.push_str(";");
} else if attribut.attribute_type == "ulong" { 
          str.push_str("\n        unsigned long ");
          str.push_str(&attribut.name);
          str.push_str(";");
} else if attribut.attribute_type == "decimal" { 
          str.push_str("\n        double ");
          str.push_str(&attribut.name);
          str.push_str(";");
} else if attribut.attribute_type == "date" { 
          str.push_str("\n        // ERROR: at ");
          str.push_str(&attribut.name);
          str.push_str("\n        // Date is currently unsupported in C Parsers");
} else { 
          str.push_str("\n        ");
          str.push_str(&attribut.attribute_type);
          str.push_str(" *");
          str.push_str(&attribut.name);
          str.push_str(";");
} 
        str.push_str("");
 } }

    str.push_str("\n} ");
    str.push_str(&typ.typename);
    str.push_str(";\n");
}

  str.push_str("\n\n#endif");
  let filename = format!("{}/jsoninc_parser.h", folder);
  filehandler::write_file(filename, str);
}


// 
// This generat Method is the entry point of code generation
//
pub fn generate(types:&Box<Vec<Box<model::Type>>>, folder:&str) {
  generate_buffer_code(folder.clone());
  generate_header(types, folder);
  let mut str:String = String::new(); 
  str.push_str("/*\n  * C code generated by staticjson\n  */\n\n#include \"sj_buffer.h\"\n#include <stdio.h>\n#include <stdlib.h>\n#include <string.h>\n\n// data structures");
for typ in (*types).iter() { 
    str.push_str("\nstruct ");
    str.push_str(&util::ucamel_to_lsnake(&typ.typename));
    str.push_str(" {\n  // TODO: Attributes\n};\n");
} 
  str.push_str("\n// end of data structures\n");
  // write it to one single file 
  let filename = format!("{}/main.c", folder);
  filehandler::write_file(filename, str);
} 
