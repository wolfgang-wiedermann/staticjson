use model;
use filehandler;
use util;
use std::collections::HashSet;

//
// This generate Method is the entry point to generation
// of java client code for the given types
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
  
  // Proxies
  for ifa in tree.interfaces.iter() {
    let result = gen_proxy(ifa, tree.types.clone());
    if ifa.is_param_present("java-package") {
      let package = ifa.get_param_value("java-package").replace(".", "/");
      let filename = format!("{}/{}/{}Proxy.java", folder, package, ifa.name);
      filehandler::write_file(filename, result);
    } else {
      let filename = format!("{}/{}Proxy.java", folder, ifa.name);
      filehandler::write_file(filename, result);
    }
  }
  
  // Testcases
  for ifa in tree.interfaces.iter() {
    let result = gen_test(ifa);
    if ifa.is_param_present("java-package") {
      let package = format!("test.{}", ifa.get_param_value("java-package")).replace(".", "/");
      let filename = format!("{}/{}/{}Test.java", folder, package, ifa.name);
      filehandler::write_file(filename, result);
    } else {
      let filename = format!("{}/{}Test.java", folder, ifa.name);
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
  str.push_str("\n\nimport java.util.ArrayList; \nimport org.apache.http.client.utils.URIBuilder;\nimport org.apache.http.HttpEntity;\nimport org.apache.http.HttpResponse;\nimport org.apache.http.client.HttpClient;\nimport org.apache.http.client.methods.HttpGet;\nimport org.apache.http.client.methods.HttpPost;\nimport org.apache.http.client.methods.HttpPut;\nimport org.apache.http.entity.StringEntity;\nimport org.apache.http.entity.ContentType;\nimport org.apache.http.impl.client.HttpClients;\nimport com.fasterxml.jackson.databind.ObjectMapper;");
  str.push_str(&get_proxies_referenced_java_packages(&ifa, types.clone()));
  str.push_str("\n\n/**\n* Generated Proxy for ");
  str.push_str(&ifa.name);
  str.push_str("\n*/\npublic class ");
  str.push_str(&ifa.name);
  str.push_str("Proxy {\n\n    // TODO: Attributes and Methods for Authentication and Connection Handling, Basepath and so on...\n    private String basePath = \"http://localhost:8081/TestApplication\";");
if ifa.is_param_present("path") { 
    str.push_str("\n    private String ifaPathFragment = \"");
    str.push_str(&ifa.get_param_value("path"));
    str.push_str("\";  ");
} else { 
    str.push_str("\n    private String ifaPathFragment = null;");
} 
  str.push_str("    \n    private ObjectMapper mapper = new ObjectMapper();\n\tprivate HttpClient clnt = HttpClients.createDefault();\n\t\n\tpublic void setBasePath(String basePath) {\n\t\tthis.basePath = basePath;\n\t}\n");
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
    str.push_str(") { ");
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

  str.push_str("\n        // HTTP-GET call\n    \ttry {\n\t    \tString path = this.basePath;\n            if(this.ifaPathFragment != null) {\n                path += this.ifaPathFragment;\n            }");
if f.is_attribute_present("path") { 
    str.push_str(" \n            path += \"");
    str.push_str(&f.get_attribute_value("path"));
    str.push_str("\";");
} 
  str.push_str("");
for param in f.params.iter() { 
    if param.is_param_present("path-param") { 
      str.push_str("\n\t    \tpath = path.replaceAll(\"\\\\{");
      str.push_str(&param.get_param_value("path-param"));
      str.push_str("\\\\}\",\"\"+");
      str.push_str(&param.name);
      str.push_str(");");
    }
} 
  str.push_str("\n            URIBuilder uriBuilder = new URIBuilder(path);");
for param in f.params.iter() {
    if param.is_param_present("query-param") { 
      str.push_str("\n            uriBuilder.addParameter(\"");
      str.push_str(&param.get_param_value("query-param"));
      str.push_str("\", ");
      str.push_str(&param.name);
      str.push_str(");");
    }
} 
  str.push_str("\n\t    \tHttpGet get = new HttpGet(uriBuilder.build());\n\t\t\tHttpResponse resp;\n\t\t\tresp = clnt.execute(get);\n\t\t\tHttpEntity httpEntity = resp.getEntity();");
if f.returntype_is_array { 
    str.push_str("\n            ");
    str.push_str(&get_java_type(&f.returntype, f.returntype_is_array));
    str.push_str(" lst = new ");
    str.push_str(&get_java_type(&f.returntype, f.returntype_is_array));
    str.push_str("();\n\t\t\t");
    str.push_str(&get_java_type(&f.returntype, false));
    str.push_str("[] array = mapper.readValue(httpEntity.getContent(), ");
    str.push_str(&get_java_type(&f.returntype, false));
    str.push_str("[].class);\n\t\t\tlst.addAll(Arrays.asList(array));\n\t\t\treturn lst;");
} else { 
    str.push_str("\n\t\t\treturn mapper.readValue(httpEntity.getContent(), ");
    str.push_str(&get_java_type(&f.returntype, f.returntype_is_array));
    str.push_str(".class);");
} 
  str.push_str("\n    \t} catch(Exception ex) {\n    \t\tthrow new RuntimeException(ex);\n    \t}");

  return str;
}

fn get_impl_for_post_function(f:&model::Function) -> String {
  let mut str:String = String::new();

  str.push_str("\n        // HTTP-POST call\n        try {\n\t    \tString path = this.basePath;\n            if(this.ifaPathFragment != null) {\n                path += this.ifaPathFragment;\n            }");
if f.is_attribute_present("path") { 
    str.push_str(" \n            path += \"");
    str.push_str(&f.get_attribute_value("path"));
    str.push_str("\";");
} 
  str.push_str("");
for param in f.params.iter() { 
    if param.is_param_present("path-param") { 
      str.push_str("\n\t    \tpath = path.replaceAll(\"\\\\{");
      str.push_str(&param.get_param_value("path-param"));
      str.push_str("\\\\}\",\"\"+");
      str.push_str(&param.name);
      str.push_str(");");
    }
} 
  str.push_str("            \n            String jsonString = mapper.writeValueAsString(");
  str.push_str(&f.get_serialized_functionparam_name());
  str.push_str(");\n            HttpPost post = new HttpPost(path);\n            StringEntity request = new StringEntity(jsonString, ContentType.APPLICATION_JSON);\n            post.setEntity(request);\n            HttpResponse resp;\n\t\t\tresp = clnt.execute(post);\n            post.releaseConnection();\n\t\t\tHttpEntity httpEntity = resp.getEntity();");
if f.returntype_is_array { 
    str.push_str("\n            ");
    str.push_str(&get_java_type(&f.returntype, f.returntype_is_array));
    str.push_str(" lst = new ");
    str.push_str(&get_java_type(&f.returntype, f.returntype_is_array));
    str.push_str("();\n\t\t\t");
    str.push_str(&get_java_type(&f.returntype, false));
    str.push_str("[] array = mapper.readValue(httpEntity.getContent(), ");
    str.push_str(&get_java_type(&f.returntype, false));
    str.push_str("[].class);\n\t\t\tlst.addAll(Arrays.asList(array));\n\t\t\treturn lst;");
} else { 
    str.push_str("\n\t\t\treturn mapper.readValue(httpEntity.getContent(), ");
    str.push_str(&get_java_type(&f.returntype, f.returntype_is_array));
    str.push_str(".class);");
} 
  str.push_str("\n    \t} catch(Exception ex) {\n    \t\tthrow new RuntimeException(ex);\n    \t}");

  return str;
}

fn get_impl_for_put_function(f:&model::Function) -> String {
  let mut str:String = String::new();

  str.push_str("\n        // HTTP-PUT call\n        try {\n\t    \tString path = this.basePath;\n            if(this.ifaPathFragment != null) {\n                path += this.ifaPathFragment;\n            }");
if f.is_attribute_present("path") { 
    str.push_str(" \n            path += \"");
    str.push_str(&f.get_attribute_value("path"));
    str.push_str("\";");
} 
  str.push_str("");
for param in f.params.iter() { 
    if param.is_param_present("path-param") { 
      str.push_str("\n\t    \tpath = path.replaceAll(\"\\\\{");
      str.push_str(&param.get_param_value("path-param"));
      str.push_str("\\\\}\",\"\"+");
      str.push_str(&param.name);
      str.push_str(");");
    }
} 
  str.push_str("\n            String jsonString = mapper.writeValueAsString(");
  str.push_str(&f.get_serialized_functionparam_name());
  str.push_str(");\n            HttpPut put = new HttpPut(path);\n            StringEntity request = new StringEntity(jsonString, ContentType.APPLICATION_JSON);\n            put.setEntity(request);\n            HttpResponse resp;\n\t\t\tresp = clnt.execute(put);\n            put.releaseConnection();\t\t\t");
if f.returntype_is_array { 
    str.push_str("\n            HttpEntity httpEntity = resp.getEntity();\n            ");
    str.push_str(&get_java_type(&f.returntype, f.returntype_is_array));
    str.push_str(" lst = new ");
    str.push_str(&get_java_type(&f.returntype, f.returntype_is_array));
    str.push_str("();\n\t\t\t");
    str.push_str(&get_java_type(&f.returntype, false));
    str.push_str("[] array = mapper.readValue(httpEntity.getContent(), ");
    str.push_str(&get_java_type(&f.returntype, false));
    str.push_str("[].class);\n\t\t\tlst.addAll(Arrays.asList(array));\n\t\t\treturn lst;");
} else if f.returntype != "void" { 
    str.push_str("\n            HttpEntity httpEntity = resp.getEntity();\n\t\t\treturn mapper.readValue(httpEntity.getContent(), ");
    str.push_str(&get_java_type(&f.returntype, f.returntype_is_array));
    str.push_str(".class);");
} 
  str.push_str("\n\n    \t} catch(Exception ex) {\n    \t\tthrow new RuntimeException(ex);\n    \t}");

  return str;
}

fn get_impl_for_delete_function(f:&model::Function) -> String {
  let mut str:String = String::new();

  str.push_str("\n        // HTTP-DELETE call\n        try {\n\t    \tString path = this.basePath;\n            if(this.ifaPathFragment != null) {\n                path += this.ifaPathFragment;\n            }");
if f.is_attribute_present("path") { 
    str.push_str(" \n            path += \"");
    str.push_str(&f.get_attribute_value("path"));
    str.push_str("\";");
} 
  str.push_str("");
for param in f.params.iter() { 
    if param.is_param_present("path-param") { 
      str.push_str("\n\t    \tpath = path.replaceAll(\"\\\\{");
      str.push_str(&param.get_param_value("path-param"));
      str.push_str("\\\\}\",\"\"+");
      str.push_str(&param.name);
      str.push_str(");");
    }
} 
  str.push_str("\n            //return null;\n    \t} catch(Exception ex) {\n    \t\t//throw new RuntimeException(ex);\n    \t}\n        throw new RuntimeException(\"Method not implemented\");");

  return str;
}


// 
// Generate code for interface
//
fn gen_test(ifa:&Box<model::Interface>) -> String {
  let mut str:String = String::new();
  if ifa.is_param_present("java-package") {

    str.push_str("package  test.");
    str.push_str(&ifa.get_param_value("java-package"));
    str.push_str(";");
} 
  str.push_str("\n\nimport org.junit.Test;\n\n/**\n* Generated Proxy for ");
  str.push_str(&ifa.name);
  str.push_str("\n*/\npublic class ");
  str.push_str(&ifa.name);
  str.push_str("Test {");
for function in ifa.functions.iter() { 
    str.push_str("\n\n    /** \n     * Test Case for function ");
    str.push_str(&function.name);
    str.push_str("\n     *");
for param in function.params.iter() { 
      str.push_str("\n     * @param ");
      str.push_str(&param.name);
      str.push_str("");
} 
    str.push_str(" \n     * @return ");
    str.push_str(&get_java_type(&function.returntype, function.returntype_is_array));
    str.push_str("\n     */ \n    @Test\n    public void ");
    str.push_str(&function.name);
    str.push_str("Test() { \n        // TODO: write test code here\n        throw new RuntimeException(\"Method not implemented\");\n    }");
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn get_proxies_referenced_java_packages(ifa:&Box<model::Interface>, types:Box<Vec<Box<model::Type>>>) -> String {    
  let mut package_set:HashSet<String> = HashSet::new();
  for func in ifa.functions.iter() {
    if func.returntype_is_array {
      package_set.insert(format!("java.util.Arrays"));
    }
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


