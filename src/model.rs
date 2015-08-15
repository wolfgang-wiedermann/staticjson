/*
* This file contains the datastructures of the staticjson tool
*/
#[derive(Clone, Debug)]
pub enum TargetLanguage {
  SWIFT, C, RUST, HTMLDOC, JSVALIDATE, JAXRS
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
    
  ININTERFACENAME,
  ININTERFACEPARAMNAME,
  ININTERFACEPARAMVALUE,
  ININTERFACEPARAMSTRING,
  OUTOFINTERFACEPARAMLIST,
    
  INFUNCTIONNAME, INFUNCTIONRETURNTYPE, 
  INFUNCTIONRETURNTYPEARRAY, BEHINDFUNCTIONRETURNTYPEARRAY,
  
  INFUNCTIONPARAMNAME,
  INFUNCTIONPARAMTYPE, INFUNCTION,
  INFUNCTIONPARAMTYPEARRAY, BEHINDFUNCTIONPARAMTYPEARRAY,
  
  // Special: Function Params can have Parameters - TODO: has to be implemented in parser
  INFUNCTIONPARAMPARAMNAME, INFUNCTIONPARAMPARAMVALUE, 
  INFUNCTIONPARAMPARAMSTRING, INFUNCTIONPARAMPARAMLIST,
  // --  

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
pub struct ParserResult {
  pub types: Box<Vec<Box<Type>>>,
  pub interfaces: Box<Vec<Box<Interface>>>
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

  // Gets the value of the parameter with the given name
  pub fn get_param_value(&self, param_name:&str) -> String {          
    for p in self.params.iter() {
      if p.name == param_name {
        return (*p).value.clone();        
      }     
    }      
    return String::new();
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
    
  // Gets the value of the parameter with the given name
  pub fn get_param_value(&self, param_name:&str) -> String {          
    for p in self.params.iter() {
      if p.name == param_name {
        return (*p).value.clone();        
      }     
    }      
    return String::new();
  }

  pub fn is_attribute_param_present(&self, param_name:&str) -> bool {
    for a in self.attributes.iter() {
      if a.is_param_present(param_name) {
        return true;
      }
    }
    return false;
  }
}

#[derive(Clone, Debug)]
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
    
  // Gets the value of the parameter with the given name
  pub fn get_param_value(&self, param_name:&str) -> String {          
    for p in self.params.iter() {
      if p.name == param_name {
        return (*p).value.clone();        
      }     
    }      
    return String::new();
  }
}

#[derive(Clone, Debug)]
pub struct Function {
  pub name:String,
  pub returntype:String,
  pub returntype_is_array:bool,
  pub params:Vec<Box<FunctionParameter>>,
  pub attributes:Vec<Box<Parameter>>
}

impl Function {
  pub fn new() -> Function {
    Function {
      name:String::new(),
      returntype:String::new(),
      returntype_is_array:false,
      params:Vec::new(),
      attributes:Vec::new()
    }
  }

  /// Checks if a param with the given name is present in
  /// params attribute
  pub fn is_attribute_present(&self, param_name:&str) -> bool {
    for p in self.attributes.iter() {
      if p.name == param_name {
        return true;
      }
    }
    return false;
  }
  // Checks if a param with the given name has the given value
  pub fn is_attribute_value_present(&self, param_name:&str, param_value:&str) -> bool {
    for p in self.attributes.iter() {
      if p.name == param_name {
        return p.value == param_value;
      } 
    }
    return false;
  }
    
  pub fn get_attribute_value(&self, attr_name:&str) -> String {          
    for attr in self.attributes.iter() {
      if attr.name == attr_name {
        return (*attr).value.clone();        
      }     
    }      
    return String::new();
  }
}

#[derive(Clone, Debug)]
pub struct FunctionParameter {
  pub name:String,
  pub typename:String,
  pub is_array:bool,
  pub params:Vec<Box<Parameter>>
}

impl FunctionParameter {
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
