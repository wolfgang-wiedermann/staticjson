/*
* This file contains the datastructures of the staticjson tool
*/
#[derive(Clone, Debug)]
pub enum TargetLanguage {
  SWIFT, C, RUST, HTMLDOC, JSVALIDATE
}

#[derive(Clone, Debug)]
pub enum ParserState {
  INITIAL, INTYPENAME, INTYPE,

  INTYPEPARAMNAME,    // Neu, noch unbenutzt
  INTYPEPARAMVALUE,   // Neu, noch unbenutzt 
  INTYPEPARAMSTRING,  // Neu, noch unbenutzt
  OUTOFFTYPEPARAMLIST,// Neu, noch unbenutzt

  INATTRIBUTENAME, INATTRIBUTETYPE, 
  INATTRIBUTEARRAY,

  INATTRIBUTEPARAMLIST,
  INATTRIBUTEPARAMNAME,
  INATTRIBUTEPARAMVALUE,
  INATTRIBUTEPARAMSTRING,
  
  INOUTERCMT, // in comment outside of typedefinition
  ININNERCMT, // in comment inside of typedefinition
    
  // Special States for Interface-Definitions
  ININTERFACECMT,
    
  ININTERFACENAME, ININTERFACE,
  ININTERFACEPARAMNAME,
  ININTERFACEPARAMVALUE,
  ININTERFACEPARAMSTRING,
  OUTOFINTERFACEPARAMLIST,
    
  INFUNCTIONNAME, INFUNCTIONRETURNTYPE, 
  INFUNCTIONRETURNTYPEARRAY, BEHINDFUNCTIONRETURNTYPEARRAY,
  INFUNCTIONPARAMLIST, INFUNCTIONPARAMNAME,
  INFUNCTIONPARAMTYPE, INFUNCTION,

  INFUNCTIONATTRIBUTENAME,
  INFUNCTIONATTRIBUTEVALUE,
  INFUNCTIONATTRIBUTESTRING,
  // End of special States for Interface Definitions
}

#[derive(Clone, Debug)]
pub enum ParserSubState {
  LEADINGBLANKS, // Fuehrende Leerzeichen
  VALUE, // Wert
  TRAILINGBLANKS, // Auf den Wert folgende Leerzeichen
}

#[derive(Clone, Debug)]
pub struct CommandlineOptions {
  pub filename:String,
  pub target_language:TargetLanguage,
  pub target_folder:String,
  pub debug:bool
}

#[derive(Clone, Debug)]
pub struct Parameter {
  pub name:String,
  pub value:String
}

#[derive(Clone, Debug)]
pub struct Attribute {
  pub name:String,
  pub attribute_type:String,
  pub is_array:bool,
  pub params:Vec<Box<Parameter>>
}

impl Attribute {
  /// Checks if a param with the given name is present in
  /// params attribute
  pub fn is_param_present(&self, param_name:&str) -> bool {
    for p in self.params.iter() {
      if p.name == param_name {
        return true;
      }
    }
    return false;
  }
  // Checks if a param with the given name has the given value
  pub fn is_param_value_present(&self, param_name:&str, param_value:&str) -> bool {
    for p in self.params.iter() {
      if p.name == param_name {
        return p.value == param_value;
      } 
    }
    return false;
  }
}

#[derive(Clone, Debug)]
pub struct Type {
  pub typename:String,
  pub attributes:Vec<Box<Attribute>>,
  pub params:Vec<Box<Parameter>>
}

impl Type {
  pub fn new() -> Type {
    Type {
      typename:String::new(),
      attributes:Vec::new(),
      params:Vec::new()
    }
  }

  pub fn is_basic_type(name:&str) -> bool {
    return name == "string"
        || name == "int"
        || name == "decimal"
        || name == "byte"
        || name == "char"
        || name == "uint"
        || name == "long"
        || name == "ulong"
        || name == "date"
        || name == "time"
        || name == "datetime";
  }

  /// Checks if a param with the given name is present in
  /// params attribute
  pub fn is_param_present(&self, param_name:&str) -> bool {
    for p in self.params.iter() {
      if p.name == param_name {
        return true;
      }
    }
    return false;
  }
  // Checks if a param with the given name has the given value
  pub fn is_param_value_present(&self, param_name:&str, param_value:&str) -> bool {
    for p in self.params.iter() {
      if p.name == param_name {
        return p.value == param_value;
      } 
    }
    return false;
  }
}

pub struct Interface {
  pub name:String,
  pub functions:Vec<Box<Function>>,
  pub params:Vec<Box<Parameter>>
}

impl Interface {
  pub fn new() -> Interface {
    Interface {
      name:String::new(),
      functions:Vec::new(),
      params:Vec::new()
    }
  }

  /// Checks if a param with the given name is present in
  /// params attribute
  pub fn is_param_present(&self, param_name:&str) -> bool {
    for p in self.params.iter() {
      if p.name == param_name {
        return true;
      }
    }
    return false;
  }
  // Checks if a param with the given name has the given value
  pub fn is_param_value_present(&self, param_name:&str, param_value:&str) -> bool {
    for p in self.params.iter() {
      if p.name == param_name {
        return p.value == param_value;
      } 
    }
    return false;
  }
}

pub struct Function {
  pub name:String,
  pub params:Vec<Box<Parameter>>,
  pub attributes:Vec<Box<Parameter>>
}

impl Function {
  pub fn new() -> Function {
    Function {
      name:String::new(),
      params:Vec::new(),
      attributes:Vec::new()
    }
  }

  /// Checks if a param with the given name is present in
  /// params attribute
  pub fn is_param_present(&self, param_name:&str) -> bool {
    for p in self.params.iter() {
      if p.name == param_name {
        return true;
      }
    }
    return false;
  }
  // Checks if a param with the given name has the given value
  pub fn is_param_value_present(&self, param_name:&str, param_value:&str) -> bool {
    for p in self.params.iter() {
      if p.name == param_name {
        return p.value == param_value;
      } 
    }
    return false;
  }
}

pub struct GeneralModel<'a> {
  pub options:&'a CommandlineOptions,
  pub code:String,
}
