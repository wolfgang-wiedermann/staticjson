use model;

pub struct Parser {
  buffer: String,
  types: Box<Vec<Box<model::Type>>>,
  interfaces: Box<Vec<Box<model::Interface>>>,
  current_type: Box<model::Type>,
  current_interface: Box<model::Interface>,
  current_function: Box<model::Function>,
  current_function_param: Box<model::FunctionParameter>,
  current_attribute: Box<model::Attribute>,
  current_param: Box<model::Parameter>,
  state: model::ParserState,
  substate: model::ParserSubState,
  cminus1: char,
  line:u64,
  column:u64,
}

impl Parser {

  pub fn new() -> Parser {
    Parser {
      buffer: String::new(),
      types: Box::new(Vec::new()),
      interfaces: Box::new(Vec::new()),
      current_type: Box::new(model::Type { typename: String::new(), 
                                           attributes: Vec::new(),
                                           params: Vec::new() }),
      current_interface: Box::new(model::Interface { name: String::new(), 
                                           functions: Vec::new(),
                                           params: Vec::new() }),
      current_function: Box::new(model::Function { name: String::new(),
                                           returntype: String::new(),
                                           returntype_is_array:false,
                                           params: Vec::new(),
                                           attributes: Vec::new()}),
      current_function_param: Box::new(model::FunctionParameter { name: String::new(), 
                                           typename: String::new(),
                                           is_array: false, 
                                           params: Vec::new() }),
      current_attribute: Box::new(model::Attribute { name: String::new(), 
                                                     attribute_type: String::new(),
                                                     is_array: false, 
                                                     params: Vec::new() }),
      current_param: Box::new(model::Parameter {name:String::new(), value:String::new()}),
      state:model::ParserState::INITIAL,
      substate:model::ParserSubState::LEADINGBLANKS,
      cminus1:' ',
      line:0,
      column:0,
    }
  }

  pub fn parse(&mut self, m: &model::GeneralModel) -> model::ParserResult {
    self.types  = Box::new(Vec::new());
    self.buffer = String::new();
    let iter = m.code.chars();

    for c in iter {
      self.update_position(&c);
      match self.state {
        model::ParserState::INITIAL => self.do_initial(c),
        model::ParserState::INOUTERCMT => self.do_inoutercomment(c),
        // Automaton parts for parsing types
        model::ParserState::INTYPENAME => self.do_intypename(c),
        model::ParserState::INATTRIBUTENAME => self.do_inattributename(c),
        model::ParserState::INATTRIBUTETYPE => self.do_inattributetype(c),
        model::ParserState::INATTRIBUTEARRAY => self.do_inattributearray(c),
        model::ParserState::INATTRIBUTEPARAMNAME => self.do_inattributeparamname(c),
        model::ParserState::INATTRIBUTEPARAMVALUE => self.do_inattributeparamvalue(c),
        model::ParserState::INATTRIBUTEPARAMSTRING => self.do_inattributeparamstring(c),
        model::ParserState::INATTRIBUTEPARAMLIST => self.do_inattributeparamlist(c),
        model::ParserState::INTYPE => self.do_intype(c),
        model::ParserState::ININNERCMT => self.do_ininnercomment(c),
        model::ParserState::INTYPEPARAMNAME => self.do_intypeparamname(c),
        model::ParserState::OUTOFFTYPEPARAMLIST => self.do_outofftypeparamlist(c),
        model::ParserState::INTYPEPARAMVALUE => self.do_intypeparamvalue(c),
        model::ParserState::INTYPEPARAMSTRING => self.do_intypeparamstring(c),
        // Automaton parts for parsing interfaces
        model::ParserState::ININTERFACENAME => self.do_ininterfacename(c),
        model::ParserState::ININTERFACEPARAMNAME => self.do_ininterfaceparamname(c),
        model::ParserState::ININTERFACEPARAMVALUE => self.do_ininterfaceparamvalue(c),
        model::ParserState::ININTERFACEPARAMSTRING => self.do_ininterfaceparamstring(c),
        model::ParserState::OUTOFINTERFACEPARAMLIST => self.do_outofinterfaceparamlist(c),
        model::ParserState::INFUNCTIONNAME => self.do_infunctionname(c),
        model::ParserState::ININTERFACECMT => self.do_ininterfacecomment(c),
        model::ParserState::INFUNCTIONPARAMNAME => self.do_infunctionparametername(c),
        model::ParserState::INFUNCTIONPARAMTYPE => self.do_infunctionparametertype(c),
        model::ParserState::INFUNCTIONPARAMTYPEARRAY => self.do_infunctionparametertypearray(c),
        model::ParserState::BEHINDFUNCTIONPARAMTYPEARRAY => self.do_behindfunctionparametertypearray(c),        
        model::ParserState::INFUNCTIONPARAMPARAMNAME => self.do_infunctionparameterparamname(c),
        model::ParserState::INFUNCTIONPARAMPARAMVALUE => self.do_infunctionparameterparamvalue(c),
        model::ParserState::INFUNCTIONPARAMPARAMSTRING => self.do_infunctionparameterparamstring(c),
        model::ParserState::INFUNCTIONPARAMPARAMLIST => self.do_infunctionparameterparamlist(c),
        model::ParserState::INFUNCTION => self.do_infunction(c),
        model::ParserState::INFUNCTIONRETURNTYPE => self.do_infunctionreturntype(c),
        model::ParserState::INFUNCTIONRETURNTYPEARRAY => self.do_infunctionreturntypearray(c),
        model::ParserState::BEHINDFUNCTIONRETURNTYPEARRAY => self.do_behindfunctionreturntypearray(c),
        model::ParserState::INFUNCTIONATTRIBUTENAME => self.do_infunctionattributename(c),
        model::ParserState::INFUNCTIONATTRIBUTEVALUE => self.do_infunctionattributevalue(c),
        model::ParserState::INFUNCTIONATTRIBUTESTRING => self.do_infunctionattributestring(c),        
        // This has to be here until all parts of the automaton for parsing interfaces are coded
        //_  => self.raise_syntax_error("\nERROR: Invalid State\n"),
      }
      self.cminus1 = c;
    }     
    
    return model::ParserResult {
      types: self.types.clone(),
      interfaces: self.interfaces.clone()
    }
  }

  // #region parser functions
  fn do_initial(&mut self, c:char) {
    if c == ' ' && self.buffer == "type".to_string() {
      self.buffer.truncate(0);
      self.state = model::ParserState::INTYPENAME;
    } else if(c == ' ' && self.buffer == "interface".to_string()) {
      self.buffer.truncate(0);
      self.state = model::ParserState::ININTERFACENAME;
    } else if Parser::is_whitespace_or_newline(&c) && self.buffer.len() == 0 {
      // ignorieren und puffer leeren
      self.buffer.truncate(0);
    } else if Parser::is_whitespace_or_newline(&c) && self.buffer.len() > 0 {
      self.raise_syntax_error("type awaitet, blanks can not stand in middle of this string");
    } else if c == '/' && self.cminus1 == '/' {
      self.state = model::ParserState::INOUTERCMT;
    } else {
      self.buffer.push(c);
    }
    self.cminus1 = c;
  }  

  fn do_inoutercomment(&mut self, c:char) {
    if c == '\n' {
      self.buffer.truncate(0);
      self.state = model::ParserState::INITIAL;
    }
  }

  fn do_ininnercomment(&mut self, c:char) {
    if c == '\n' {
      self.buffer.truncate(0);
      self.state = model::ParserState::INATTRIBUTENAME;
    }
  }

  fn do_ininterfacecomment(&mut self, c:char) {
    if c == '\n' {
      self.buffer.truncate(0);
      self.state = model::ParserState::INFUNCTIONNAME;
    }
  }

  fn do_intypename(&mut self, c:char) {
    if c == '{' && self.buffer.len() > 0 {
      self.current_type.typename = self.buffer.clone();
      self.buffer.truncate(0);
      self.state = model::ParserState::INATTRIBUTENAME;
      self.substate = model::ParserSubState::LEADINGBLANKS;
    } else if c == '(' && self.buffer.len() > 0 {
      self.current_type.typename = self.buffer.clone();
      self.buffer.truncate(0);
      self.state = model::ParserState::INTYPEPARAMNAME;
      self.substate = model::ParserSubState::LEADINGBLANKS;
    } else if Parser::is_valid_name_character(&c) {
      match self.substate {
        model::ParserSubState::LEADINGBLANKS => {
          self.buffer.push(c);
          self.substate = model::ParserSubState::VALUE;
        }
        model::ParserSubState::VALUE => {
          self.buffer.push(c);
        }
        model::ParserSubState::TRAILINGBLANKS => {
          self.raise_syntax_error("blanks are not allowed within type names");
        }
      }
    } else if Parser::is_whitespace_or_newline(&c) {
      match self.substate {
        model::ParserSubState::VALUE => {
          self.substate = model::ParserSubState::TRAILINGBLANKS;
        }
        _ => {
          // Nix machen
        }
      }
    } else {
      self.raise_syntax_error("Invalid character in type name");
    }
  }
      
  fn do_ininterfacename(&mut self, c:char) {
    if c == '{' && self.buffer.len() > 0 {
      self.current_interface.name = self.buffer.clone();
      // DEBUG:
      println!("Interface: {}", self.buffer);
      // ------
      self.buffer.truncate(0);
      self.state = model::ParserState::INFUNCTIONNAME;
      self.substate = model::ParserSubState::LEADINGBLANKS;
    } else if c == '(' && self.buffer.len() > 0 {
      self.current_interface.name = self.buffer.clone();
      // DEBUG:
      println!("Interface: {}", self.buffer);
      // ------
      self.buffer.truncate(0);
      self.state = model::ParserState::ININTERFACEPARAMNAME;
      self.substate = model::ParserSubState::LEADINGBLANKS;
    } else if Parser::is_valid_name_character(&c) {
      match self.substate {
        model::ParserSubState::LEADINGBLANKS => {
          self.buffer.push(c);
          self.substate = model::ParserSubState::VALUE;
        }
        model::ParserSubState::VALUE => {
          self.buffer.push(c);
        }
        model::ParserSubState::TRAILINGBLANKS => {
          self.raise_syntax_error("blanks are not allowed within interface names");
        }
      }
    } else if Parser::is_whitespace_or_newline(&c) {
      match self.substate {
        model::ParserSubState::VALUE => {
          self.substate = model::ParserSubState::TRAILINGBLANKS;
        }
        _ => {
          // Nix machen
        }
      }
    } else {
      self.raise_syntax_error("Invalid character in interface name");
    }    
  }

  fn do_ininterfaceparamname(&mut self, c:char) {
    if c == '=' && self.buffer.len() > 0 {
      self.current_param.name = self.buffer.clone();
      // DEBUG:
      println!("InterfaceParamName: {}", self.buffer);
      // END DEBUG
      self.buffer.truncate(0);
      self.state = model::ParserState::ININTERFACEPARAMVALUE; // TODO: Find out state
      self.substate = model::ParserSubState::LEADINGBLANKS;
    } else if c == ')' && self.buffer.len() == 0 {
      self.state = model::ParserState::OUTOFINTERFACEPARAMLIST;
      self.substate = model::ParserSubState::LEADINGBLANKS;
    } else if Parser::is_valid_name_character(&c) {
      match self.substate {
        model::ParserSubState::LEADINGBLANKS => {
          self.buffer.push(c);
          self.substate = model::ParserSubState::VALUE;
        }
        model::ParserSubState::VALUE => {
          self.buffer.push(c);
        }
        model::ParserSubState::TRAILINGBLANKS => {
          self.raise_syntax_error("blanks are not allwoed within interface param names");
        }
      }
    } else if Parser::is_whitespace_or_newline(&c) {
      match self.substate {
        model::ParserSubState::VALUE => {
          self.substate = model::ParserSubState::TRAILINGBLANKS;
        }
        _ => {
          // Nix machen
        }
      }
    } else {
      // DEBUG:
      println!("CHAR: {}", c);
      // END DEBUG
      self.raise_syntax_error("Invalid character in interface param name");
    }
  }

  fn do_ininterfaceparamvalue(&mut self, c:char) {
    if c == '"' {
      self.state = model::ParserState::ININTERFACEPARAMSTRING;
      self.substate = model::ParserSubState::LEADINGBLANKS;
    } else if c == ',' {
      self.state = model::ParserState::ININTERFACEPARAMNAME;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      self.store_current_interface_param();
    } else if c == ')' {
      self.state = model::ParserState::OUTOFINTERFACEPARAMLIST;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      self.store_current_interface_param();
    } else if !Parser::is_whitespace_or_newline(&c) {
      self.raise_syntax_error("invalid character between = and \" in INTERFACEPARAMVALUE");
    }
  }

  fn do_ininterfaceparamstring(&mut self, c:char) {
    if c == '"' && self.cminus1 != '\\' {
      self.state = model::ParserState::ININTERFACEPARAMVALUE;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      self.current_param.value = self.buffer.clone();
      println!("InterfaceParamString: {}", self.buffer);
      self.buffer.truncate(0);
    } else {
      self.buffer.push(c);
    }
  }

  fn do_outofinterfaceparamlist(&mut self, c:char) {
    if c == '{' {
      self.state = model::ParserState::INFUNCTIONNAME;
      self.substate = model::ParserSubState::LEADINGBLANKS;
    } else if !Parser::is_whitespace_or_newline(&c) {
      self.raise_syntax_error("invalid character between interface param list and { ");
    }
  }

  fn do_infunctionname(&mut self, c:char) {
    if c == '(' && self.buffer.len() > 0 {
      self.current_function.name = self.buffer.clone();
      // DEBUG:
      println!("{}", self.buffer);
      // ------
      self.buffer.truncate(0);
      self.state = model::ParserState::INFUNCTIONPARAMNAME;
      self.substate = model::ParserSubState::LEADINGBLANKS;
    } else if c == '}' && self.buffer.len() == 0 {
      self.state = model::ParserState::INITIAL;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      self.store_current_interface();
    } else if Parser::is_valid_name_character(&c) {
      match self.substate {
        model::ParserSubState::LEADINGBLANKS => {
          self.buffer.push(c);
          self.substate = model::ParserSubState::VALUE;
        }
        model::ParserSubState::VALUE => {
          self.buffer.push(c);
        }
        model::ParserSubState::TRAILINGBLANKS => {
          self.raise_syntax_error("blanks are not allowed within interface names");
        }
      }
    } else if c == '/' && self.cminus1 == '/' {
      self.state = model::ParserState::ININTERFACECMT;
    } else if Parser::is_whitespace_or_newline(&c) {
      match self.substate {
        model::ParserSubState::VALUE => {
          self.substate = model::ParserSubState::TRAILINGBLANKS;
        }
        _ => {
          // Nix machen
        }
      }
    } else if c != '/' {
      // DEBUG:
        println!("CHAR: {}", c);
      // ----
      self.raise_syntax_error("Invalid character in interface name");
    }  
  }

  fn do_infunctionparametername(&mut self, c:char) {
    if c == ':' && self.buffer.len() > 0 {
      self.current_function_param.name = self.buffer.clone();
      // DEBUG:
      println!("FunctionParamName: {}", self.buffer);
      // ------
      self.buffer.truncate(0);
      self.state = model::ParserState::INFUNCTIONPARAMTYPE;
      self.substate = model::ParserSubState::LEADINGBLANKS;
    } else if c == ')' && self.buffer.len() == 0 {
      self.buffer.truncate(0);
      self.state = model::ParserState::INFUNCTION;
      self.substate = model::ParserSubState::LEADINGBLANKS;
    } else if c == ',' && self.buffer.len() == 0 {
      // If it comes in again from do_infunctionparametertype
      self.state = model::ParserState::INFUNCTIONPARAMNAME;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      self.store_current_function_param();
    } else if Parser::is_valid_name_character(&c) {
      match self.substate {
        model::ParserSubState::LEADINGBLANKS => {
          self.buffer.push(c);
          self.substate = model::ParserSubState::VALUE;
        }
        model::ParserSubState::VALUE => {
          self.buffer.push(c);
        }
        model::ParserSubState::TRAILINGBLANKS => {
          self.raise_syntax_error("blanks are not allowed within function parameter names");
        }
      }
    } else if c == '/' && self.cminus1 == '/' {
      self.state = model::ParserState::ININTERFACECMT;
    } else if Parser::is_whitespace_or_newline(&c) {
      match self.substate {
        model::ParserSubState::VALUE => {
          self.substate = model::ParserSubState::TRAILINGBLANKS;
        }
        _ => {
          // Nix machen
        }
      }
    } else {
      // DEBUG:
      println!("CHAR: {}", c);
      println!("BUFF: {}", self.buffer);
      // ----
      self.raise_syntax_error("Invalid character in interface name");
    }  
  }

  fn do_infunctionparametertype(&mut self, c:char) {
    if c == ')' && self.buffer.len() > 0 {
      self.current_function_param.typename = self.buffer.clone();
      // DEBUG:
      println!("FunctionParamType: {}", self.buffer);
      // ------
      self.buffer.truncate(0);
      self.state = model::ParserState::INFUNCTION;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      self.store_current_function_param();    
    } else if c == '(' && self.buffer.len() > 0 {
      self.current_function_param.typename = self.buffer.clone();
      // DEBUG:
      println!("FunctionParamType: {}", self.buffer);
      // ------
      self.buffer.truncate(0);
      self.state = model::ParserState::INFUNCTIONPARAMPARAMNAME;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      // has to be stored after all function param params are read!      
    } else if c == '[' && self.buffer.len() > 0 {      
    self.current_function_param.typename = self.buffer.clone();
      // DEBUG:
      println!("FunctionParamType: {}", self.buffer);
      // ------
      self.buffer.truncate(0);
      self.state = model::ParserState::INFUNCTIONPARAMTYPEARRAY;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      // has to be stored after all function param params are read!      
    } else if c == ',' && self.buffer.len() > 0 {
      self.current_function_param.typename = self.buffer.clone();
      // DEBUG:
      println!("FunctionParamType: {}", self.buffer);
      // ------
      self.buffer.truncate(0);
      self.state = model::ParserState::INFUNCTIONPARAMNAME;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      self.store_current_function_param();
    } else if Parser::is_valid_name_character(&c) {
      match self.substate {
        model::ParserSubState::LEADINGBLANKS => {
          self.buffer.push(c);
          self.substate = model::ParserSubState::VALUE;
        }
        model::ParserSubState::VALUE => {
          self.buffer.push(c);
        }
        model::ParserSubState::TRAILINGBLANKS => {
          self.raise_syntax_error("blanks are not allowed within function parameter types");
        }
      }
    } else if c == '/' && self.cminus1 == '/' {
      self.state = model::ParserState::ININTERFACECMT;
    } else if Parser::is_whitespace_or_newline(&c) {
      match self.substate {
        model::ParserSubState::VALUE => {
          self.substate = model::ParserSubState::TRAILINGBLANKS;
        }
        _ => {
          // Nix machen
        }
      }
    } else if c != '/' {
      // DEBUG:
      println!("CHAR: {}", c);
      println!("BUFF: {}", self.buffer);
      println!("EXPR: {}", c == ')' && self.buffer.len() > 0);
      // ----
      self.raise_syntax_error("Invalid character in function parameter types");
    }  
  }
  
  fn do_infunctionparametertypearray(&mut self, c:char) {
    if c == ']' {
      self.current_function_param.is_array = true;
      self.state = model::ParserState::BEHINDFUNCTIONPARAMTYPEARRAY;
      self.substate = model::ParserSubState::LEADINGBLANKS;
    } else {
      self.raise_syntax_error("Invalid character [ must be followed by ]!");
    }
  }
  
  fn do_behindfunctionparametertypearray(&mut self, c:char) {
    if c == ')' {            
      self.state = model::ParserState::INFUNCTION;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      self.store_current_function_param();    
    } else if c == '(' {      
      self.state = model::ParserState::INFUNCTIONPARAMPARAMNAME;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      // has to be stored after all function param params are read!      
    } else if !Parser::is_valid_name_character(&c) {
      println!("Character: {}", c);
      self.raise_syntax_error("Invalid character behind function parameter type array definition");
    }
  }
  
  fn do_infunctionparameterparamname(&mut self, c:char) {
    if c == ')' && self.buffer.len() == 0 {
      self.buffer.truncate(0);
      self.state = model::ParserState::INFUNCTION;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      self.store_current_function_param();   
    } else if c == '=' && self.buffer.len() > 0 {
      // DEBUG:
      println!("FunctionParamParamName: {}", self.buffer);
      // ------
      self.current_param.name = self.buffer.clone();
      self.buffer.truncate(0);
      self.state = model::ParserState::INFUNCTIONPARAMPARAMVALUE;
      self.substate = model::ParserSubState::LEADINGBLANKS;         
    } else if Parser::is_valid_name_character(&c) {
      match self.substate {
        model::ParserSubState::LEADINGBLANKS => {
          self.buffer.push(c);
          self.substate = model::ParserSubState::VALUE;
        }
        model::ParserSubState::VALUE => {
          self.buffer.push(c);
        }
        model::ParserSubState::TRAILINGBLANKS => {
          self.raise_syntax_error("blanks are not allowed within function parameter types");
        }
      }
    } else if Parser::is_whitespace_or_newline(&c) {
      match self.substate {
        model::ParserSubState::VALUE => {
          self.substate = model::ParserSubState::TRAILINGBLANKS;
        }
        _ => {
          // Nix machen
        }
      }
    }
  }
  
  fn do_infunctionparameterparamvalue(&mut self, c:char) {
    if c == '"' {
      self.state = model::ParserState::INFUNCTIONPARAMPARAMSTRING;
      self.substate = model::ParserSubState::LEADINGBLANKS; 
    } else if !Parser::is_whitespace_or_newline(&c) {
      self.raise_syntax_error("invalid character in definition function parameter parameters");
    }
  }
  
  fn do_infunctionparameterparamstring(&mut self, c:char) {
    if c == '"' && self.cminus1 != '\\' {
      // Debug output:
      println!("FunctionParamParamString: {}", self.buffer);
      // ---
      self.state = model::ParserState::INFUNCTIONPARAMPARAMLIST;
      self.current_param.value = self.buffer.clone();
      self.store_current_function_param_param();
      self.buffer.truncate(0);
    } else {
      self.buffer.push(c);
    }
  }
  
  fn do_infunctionparameterparamlist(&mut self, c:char) {
    if c == ',' {
      self.state = model::ParserState::INFUNCTIONPARAMPARAMNAME;
      self.substate = model::ParserSubState::LEADINGBLANKS; 
    } else if c == ')' {
      self.state = model::ParserState::INFUNCTIONPARAMNAME;
      self.substate = model::ParserSubState::LEADINGBLANKS; 
      self.store_current_function_param(); 
    } else if !Parser::is_whitespace_or_newline(&c) {
      self.raise_syntax_error("invalid character in definition function parameter parameters");
    }
  }

  fn do_infunction(&mut self, c:char) {
    if c == '{' {
      self.state = model::ParserState::INFUNCTIONATTRIBUTENAME;
      self.substate = model::ParserSubState::LEADINGBLANKS;
    } else if c == '-' && self.cminus1 != '-' {
      // Maybee if next is
    } else if c == '>' && self.cminus1 == '-' { 
      self.state = model::ParserState::INFUNCTIONRETURNTYPE;
      self.substate = model::ParserSubState::LEADINGBLANKS;    
    } else if !Parser::is_whitespace_or_newline(&c) {
      // DEBUG:
      println!("CHAR: {}", c);
      println!("BUFF: {}", self.buffer);
      // ----
      self.raise_syntax_error("Invalid character in behind function parameter list");
    }
  }
  
  fn do_infunctionreturntype(&mut self, c:char) {
    if c == '{' && self.buffer.len() > 0 {
      self.current_function.returntype = self.buffer.clone();
      // DEBUG:
      println!("FunctionReturnType: {}", self.buffer);
      // ------
      self.buffer.truncate(0);
      self.state = model::ParserState::INFUNCTIONATTRIBUTENAME;
      self.substate = model::ParserSubState::LEADINGBLANKS;
    } else if c == '[' && self.buffer.len() > 0 {
      self.current_function.returntype = self.buffer.clone();
      // DEBUG:
      println!("FunctionReturnType: {}", self.buffer);
      println!("FunctionReturnType is array");
      // ------
      self.buffer.truncate(0);
      self.state = model::ParserState::INFUNCTIONRETURNTYPEARRAY;
      self.substate = model::ParserSubState::LEADINGBLANKS;
    } else if Parser::is_valid_name_character(&c) {
      match self.substate {
        model::ParserSubState::LEADINGBLANKS => {
          self.buffer.push(c);
          self.substate = model::ParserSubState::VALUE;
        }
        model::ParserSubState::VALUE => {
          self.buffer.push(c);
        }
        model::ParserSubState::TRAILINGBLANKS => {
          self.raise_syntax_error("blanks are not allowed within function parameter types");
        }
      }
    } else if Parser::is_whitespace_or_newline(&c) {
      match self.substate {
        model::ParserSubState::VALUE => {
          self.substate = model::ParserSubState::TRAILINGBLANKS;
        }
        _ => {
          // Nix machen
        }
      }
    } else {
      // DEBUG:
      println!("CHAR: {}", c);
      println!("BUFF: {}", self.buffer);      
      // ----
      self.raise_syntax_error("Invalid character in function return type");
    }
  }

  fn do_infunctionreturntypearray(&mut self, c:char) {
    if c == ']' {
      self.current_function.returntype_is_array = true;
      self.state = model::ParserState::BEHINDFUNCTIONRETURNTYPEARRAY;
      self.substate = model::ParserSubState::LEADINGBLANKS;
    } else {
      self.raise_syntax_error("[ must be followed by ] directly");
    }
  }

  fn do_behindfunctionreturntypearray(&mut self, c:char) {
    if c == '{' {
      self.state = model::ParserState::INFUNCTIONATTRIBUTENAME;
      self.substate = model::ParserSubState::LEADINGBLANKS;
    } else if !Parser::is_whitespace_or_newline(&c) {
      self.raise_syntax_error("invalid character behind functionreturntype array");
    }
  }
  
  fn do_infunctionattributename(&mut self, c:char) {
    if c == '=' && self.buffer.len() > 0 {
      self.current_param.name = self.buffer.clone();
      // DEBUG:
      println!("FunctionAttributeName: {}", self.buffer);
      // END DEBUG
      self.buffer.truncate(0);
      self.state = model::ParserState::INFUNCTIONATTRIBUTEVALUE; // TODO: Find out state
      self.substate = model::ParserSubState::LEADINGBLANKS;
    } else if c == '}' && self.buffer.len() == 0 {
      self.state = model::ParserState::INFUNCTIONNAME;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      self.store_current_function();
    } else if c == '/' && self.cminus1 == '/' {
      self.state = model::ParserState::ININTERFACECMT;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      self.buffer.truncate(0);
    } else if Parser::is_valid_name_character(&c) {
      match self.substate {
        model::ParserSubState::LEADINGBLANKS => {
          self.buffer.push(c);
          self.substate = model::ParserSubState::VALUE;
        }
        model::ParserSubState::VALUE => {
          self.buffer.push(c);
        }
        model::ParserSubState::TRAILINGBLANKS => {
          self.raise_syntax_error("blanks are not allwoed within attribute names");
        }
      }
    } else if Parser::is_whitespace_or_newline(&c) {
      match self.substate {
        model::ParserSubState::VALUE => {
          self.substate = model::ParserSubState::TRAILINGBLANKS;
        }
        _ => {
          // Nix machen
        }
      }
    } else if c != '/' {
      // DEBUG:
      println!("CHAR: {}", c);
      // END DEBUG
      self.raise_syntax_error("Invalid character in attribute name");
    }
  }

  fn do_infunctionattributevalue(&mut self, c:char) {
    if c == '"' {
      self.state = model::ParserState::INFUNCTIONATTRIBUTESTRING;
      self.substate = model::ParserSubState::LEADINGBLANKS;
    } else if c == ',' {
      self.state = model::ParserState::INFUNCTIONATTRIBUTENAME;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      self.store_current_function_attribute();
    } else if c == '}' {
      self.state = model::ParserState::INFUNCTIONNAME;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      self.store_current_function_attribute();
      self.store_current_function();
    } else if !Parser::is_whitespace_or_newline(&c) {
      self.raise_syntax_error("invalid character between = and \" in FUNCTIONATTRIBUTEVALUE");
    }
  }

  fn do_infunctionattributestring(&mut self, c:char) {
      if c == '"' && self.cminus1 != '\\' {
        self.state = model::ParserState::INFUNCTIONATTRIBUTEVALUE;
        self.substate = model::ParserSubState::LEADINGBLANKS;
        self.current_param.value = self.buffer.clone();
        println!("FunctionAttributeValue: {}", self.buffer);
        self.buffer.truncate(0);        
      } else {
        self.buffer.push(c);
      }
  }

  fn do_inattributename(&mut self, c:char) {
    if c == ':' && self.buffer.len() > 0 {
      self.current_attribute.name = self.buffer.clone();
      self.buffer.truncate(0);
      self.state = model::ParserState::INATTRIBUTETYPE;
      self.substate = model::ParserSubState::LEADINGBLANKS;
    } else if c == '}' && self.buffer.len() == 0 {
      self.state = model::ParserState::INITIAL;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      self.store_current_type();
    } else if c == '/' && self.cminus1 == '/' {
      self.state = model::ParserState::ININNERCMT;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      self.buffer.truncate(0);
    } else if Parser::is_valid_name_character(&c) {
      match self.substate {
        model::ParserSubState::LEADINGBLANKS => {
          self.buffer.push(c);
          self.substate = model::ParserSubState::VALUE;
        }
        model::ParserSubState::VALUE => {
          self.buffer.push(c);
        }
        model::ParserSubState::TRAILINGBLANKS => {
          self.raise_syntax_error("blanks are not allwoed within attribute names");
        }
      }
    } else if Parser::is_whitespace_or_newline(&c) {
      match self.substate {
        model::ParserSubState::VALUE => {
          self.substate = model::ParserSubState::TRAILINGBLANKS;
        }
        _ => {
          // Nix machen
        }
      }
    } else if c != '/' {
      self.raise_syntax_error("Invalid character in attribute name");
    }
  }
  
  fn do_inattributetype(&mut self, c:char) {
    if c == ';' && self.buffer.len() > 0 {
      self.state = model::ParserState::INATTRIBUTENAME;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      self.current_attribute.attribute_type = self.buffer.clone();
      self.buffer.truncate(0);
      self.store_current_attribute();
    } else if c == '[' {
      self.state = model::ParserState::INATTRIBUTEARRAY;
      self.current_attribute.attribute_type = self.buffer.clone();
    } else if c == '(' && self.buffer.len() > 0 {
      self.state = model::ParserState::INATTRIBUTEPARAMNAME;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      self.current_attribute.attribute_type = self.buffer.clone();
      self.buffer.truncate(0);
    } else if c == ';' || c == '(' {
      self.raise_syntax_error("Attribute type required");
    } else if Parser::is_valid_name_character(&c) {
      match self.substate {
        model::ParserSubState::LEADINGBLANKS => {
          self.buffer.push(c);
          self.substate = model::ParserSubState::VALUE;
        }
        model::ParserSubState::VALUE => {
          self.buffer.push(c);
        }
        model::ParserSubState::TRAILINGBLANKS => {
          self.raise_syntax_error("blanks are not allwoed within attribute types");
        }
      }
    } else if Parser::is_whitespace_or_newline(&c) {
      match self.substate { 
        model::ParserSubState::VALUE => {
          self.substate = model::ParserSubState::TRAILINGBLANKS;
        }
        _ => {
          // Nix machen
        }
      }
    } else {
      self.raise_syntax_error("Invalid character in attribute type");
    }
  }

  fn do_inattributearray(&mut self, c:char) {
    if c == ']' {
      self.state = model::ParserState::INATTRIBUTETYPE;
      self.current_attribute.is_array = true;
    } else {
      self.raise_syntax_error("[ has to be followed by ]");
    }
  }

  fn do_inattributeparamname(&mut self, c:char) {
    if c == '=' && self.buffer.len() > 0 {
      self.state = model::ParserState::INATTRIBUTEPARAMVALUE;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      self.current_param.name = self.buffer.clone();
      self.buffer.truncate(0);
    } else if c == '=' {
      self.raise_syntax_error("Invalid character = , parameter name required");
    } else if Parser::is_valid_name_character(&c) {
      match self.substate {
        model::ParserSubState::LEADINGBLANKS => {
          self.buffer.push(c);
          self.substate = model::ParserSubState::VALUE;
        }
        model::ParserSubState::VALUE => {
          self.buffer.push(c);
        }
        model::ParserSubState::TRAILINGBLANKS => {
          self.raise_syntax_error("blanks are not allwoed within parameter names");
        }
      } 
    } else if Parser::is_whitespace_or_newline(&c) {
      match self.substate {
        model::ParserSubState::VALUE => {
          self.substate = model::ParserSubState::TRAILINGBLANKS;
        }
        _ => {
          // Nix machen
        }
      }
    } else {
      self.raise_syntax_error("Invalid character in parameter name");
    }
  }

  fn do_inattributeparamvalue(&mut self, c:char) {
    if c == '"' {
      self.state = model::ParserState::INATTRIBUTEPARAMSTRING;
    } else if !Parser::is_whitespace_or_newline(&c) {
      self.raise_syntax_error("just whitespace-characters allowed between = and \" ");
    }
  }

  fn do_inattributeparamstring(&mut self, c:char) {
    if c == '"' && self.cminus1 != '\\' {
      self.state = model::ParserState::INATTRIBUTEPARAMLIST;
      self.current_param.value = self.buffer.clone();
      self.store_current_param();
      self.buffer.truncate(0);
    } else {
      self.buffer.push(c);
    }
  }

  fn do_inattributeparamlist(&mut self, c:char) {
    if c == ',' {
      self.state = model::ParserState::INATTRIBUTEPARAMNAME;
    } else if c == ')' {
      self.state = model::ParserState::INTYPE;
      self.store_current_attribute();
    } else if !Parser::is_whitespace_or_newline(&c) {
      self.raise_syntax_error("just whitespace-characters allowed between \" and terminator");
    }
  }

  fn do_intype(&mut self, c:char) {
    if c == ';' {
      self.state = model::ParserState::INATTRIBUTENAME;
    } else if c == '}' {
      self.state = model::ParserState::INITIAL;
      self.store_current_type();
    } else if !Parser::is_whitespace_or_newline(&c) {
      self.raise_syntax_error("just whitespace-characters allowed");
    }
  }

  fn do_intypeparamname(&mut self, c:char) {
    if c == ')' {
      self.state = model::ParserState::OUTOFFTYPEPARAMLIST;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      self.current_param.name = self.buffer.clone();
      //println!("TYPENAME: {}", self.buffer);
      self.buffer.truncate(0);
    } else if c == '=' && self.buffer.len() > 0 {
      self.state = model::ParserState::INTYPEPARAMVALUE;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      self.current_param.name = self.buffer.clone();
      //println!("TYPENAME: {}", self.buffer);
      self.buffer.truncate(0);
    } else if Parser::is_valid_name_character(&c) {
      match self.substate {
        model::ParserSubState::LEADINGBLANKS => {
          self.buffer.push(c);
          self.substate = model::ParserSubState::VALUE;
        }
        model::ParserSubState::VALUE => {
          self.buffer.push(c);
        }
        model::ParserSubState::TRAILINGBLANKS => {
          self.raise_syntax_error("blanks are not allwoed within parameter names");
        }
      }
    } else if Parser::is_whitespace_or_newline(&c) {
      match self.substate {
        model::ParserSubState::VALUE => {
          self.substate = model::ParserSubState::TRAILINGBLANKS;
        }
        _ => {
          // Ignorieren
        }
      }
    } else {
      self.raise_syntax_error("illegal character found"); // Sprechender machen
    }
  }

  fn do_intypeparamvalue(&mut self, c:char) {
    if c == '"' {
      self.state = model::ParserState::INTYPEPARAMSTRING;
    } else if c == ',' {
      self.state = model::ParserState::INTYPEPARAMNAME;
    } else if c == ')' {
      self.state = model::ParserState::OUTOFFTYPEPARAMLIST;
    } else if Parser::is_whitespace_or_newline(&c) {
      // Ignore whitespace
    } else {
      self.raise_syntax_error("invalid character between = and \"");
    }
  }

  fn do_intypeparamstring(&mut self, c:char) {
    if c == '"' && self.cminus1 != '\\' {
      self.state = model::ParserState::INTYPEPARAMVALUE;
      self.current_param.value = self.buffer.clone();
      self.store_current_type_param();
      //println!("TYPEPARAMSTRING: {}", self.buffer);
      self.buffer.truncate(0);
    } else {
      self.buffer.push(c);
    }
  }

  fn do_outofftypeparamlist(&mut self, c:char) {
    if c == '{' {
      self.state = model::ParserState::INATTRIBUTENAME;
      self.substate = model::ParserSubState::LEADINGBLANKS;
      self.buffer.truncate(0);
    } else if Parser::is_whitespace_or_newline(&c) {
      // Ignore whitespaces
    } else {
      self.raise_syntax_error("invalid character");
    }
  }

  // #end_region parsr_functions
  // #region helper functions

  fn store_current_param(&mut self) {
    self.current_attribute.params.push(self.current_param.clone());
    self.current_param.name.truncate(0);
    self.current_param.value.truncate(0);
  }

  fn store_current_type_param(&mut self) {
    self.current_type.params.push(self.current_param.clone());
    self.current_param.name.truncate(0);
    self.current_param.value.truncate(0);
  }

  fn store_current_interface_param(&mut self) {
    self.current_interface.params.push(self.current_param.clone());
    self.current_param.name.truncate(0);
    self.current_param.value.truncate(0);
  }

  fn store_current_attribute(&mut self) {
    self.current_type.attributes.push(self.current_attribute.clone());
    self.current_attribute.name.truncate(0);
    self.current_attribute.attribute_type.truncate(0);
    self.current_attribute.params.truncate(0);
    self.current_attribute.is_array = false;
  }

  fn store_current_type(&mut self) {
    self.types.push(self.current_type.clone());
    self.current_type.typename.truncate(0);
    self.current_type.attributes.truncate(0);
    self.current_type.params.truncate(0);
  }

  fn store_current_interface(&mut self) {
    self.interfaces.push(self.current_interface.clone());
    self.current_interface.name.truncate(0);
    self.current_interface.functions.truncate(0);
    self.current_interface.params.truncate(0);
  }

  fn store_current_function(&mut self) {
    self.current_interface.functions.push(self.current_function.clone());
    self.current_function.name.truncate(0);
    self.current_function.params.truncate(0);
    self.current_function.attributes.truncate(0);
    self.current_function.returntype_is_array = false;
  }

  fn store_current_function_attribute(&mut self) {
    self.current_function.attributes.push(self.current_param.clone());
    self.current_param.name.truncate(0);
    self.current_param.value.truncate(0);
  }
  
  fn store_current_function_param(&mut self) {
    self.current_function.params.push(self.current_function_param.clone());
    self.current_function_param.name.truncate(0);
    self.current_function_param.typename.truncate(0);
    self.current_function_param.is_array = false;
    self.current_function_param.params.truncate(0);
  }
   
  fn store_current_function_param_param(&mut self) {
    self.current_function_param.params.push(self.current_param.clone());
    self.current_param.name.truncate(0);
    self.current_param.value.truncate(0);
  }

  pub fn is_whitespace_or_newline(c:&char) -> bool {
    return *c == ' ' || *c == '\t' || *c == '\n' || *c == '\r';
  }

  pub fn is_valid_name_character(c:&char) -> bool {
    return ( *c >= 'a' && *c <= 'z' )
        || ( *c >= 'A' && *c <= 'Z' )
        || ( *c >= '0' && *c <= '9' )
        || *c == '_' || *c == '-';
  }

  fn update_position(&mut self, c:&char) {
    match *c {
      '\n' => {
        self.column = 0;
        self.line += 1;
      }
      '\r' => { 
        // einfach ignorieren 
      }
      _ => {
        self.column += 1;
      }
    }
  }

  fn raise_syntax_error(&mut self, txt:&str) {
    //panic!("SYNTAX-ERROR: {} in line {}:{}", txt, self.line+1, self.column-1);
      panic!("SYNTAX-ERROR: {} : {:?}", txt, self.state);
  }
  // #end_region helper functions
}
