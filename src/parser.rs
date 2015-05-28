use model;

pub struct Parser {
  buffer: String,
  types: Box<Vec<Box<model::Type>>>,
  current_type: Box<model::Type>,
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
      buffer:String::new(),
      types:Box::new(Vec::new()),
      current_type: Box::new(model::Type { typename: String::new(), 
                                           attributes: Vec::new(),
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

  pub fn parse(&mut self, m: &model::GeneralModel) -> &Box<Vec<Box<model::Type>>> {
    self.types  = Box::new(Vec::new());
    self.buffer = String::new();
    let iter = m.code.chars();

    for c in iter {
      self.update_position(&c);
      match self.state {
        model::ParserState::INITIAL => self.do_initial(c),
        model::ParserState::INOUTERCMT => self.do_inoutercomment(c),
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
        //_  => self.raise_syntax_error("\nERROR: Invalid State\n"),
      }
      self.cminus1 = c;
    } 

    return &self.types;
  }

  // #region parser functions
  fn do_initial(&mut self, c:char) {
    if c == ' ' && self.buffer == "type".to_string() {
      self.buffer.truncate(0);
      self.state = model::ParserState::INTYPENAME;
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
    panic!("SYNTAX-ERROR: {} in line {}:{}", txt, self.line+1, self.column-1);
  }
  // #end_region helper functions
}
