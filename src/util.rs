/*
* Utility functions to convert camel case to snake case and inverse
*/

/**
* converts lower case camel case to lower case snake case
*/
pub fn lcamel_to_lsnake(txt:&str) -> String {
  let mut buf:String = String::new();

  for c in txt.chars() {
    if c.is_uppercase() {
      buf.push('_');
      for c2 in c.to_lowercase() {
        buf.push(c2);
      }
    } else {
      buf.push(c);
    }
  } 
 
  return buf.clone();
}


/**
* converts upper case camel case to lower case snake case
*/
pub fn ucamel_to_lsnake(txt:&str) -> String {
  let mut buf:String = String::new();

  for c in txt.chars() { 
    if buf.len() == 0 {
      for c2 in c.to_lowercase() {
        buf.push(c2);
      }
    } else if c.is_uppercase() {
      buf.push('_');
      for c2 in c.to_lowercase() {
        buf.push(c2);
      }
    } else {
      buf.push(c);
    }
  } 

  return buf.clone();
}

/**
* converts lower snake case to lower camel case
*/
pub fn lsnake_to_lcamel(txt:&str) -> String {
  let mut buf:String = String::new();
  let mut state = SnakeState::TXT;

  for c in txt.chars() {
    match state {
      SnakeState::TXT => {
        if c == '_' {
          state = SnakeState::UNDERSCORE;
        } else {
          buf.push(c);
        }
      }
      SnakeState::UNDERSCORE => {
        for c2 in c.to_uppercase() {
          buf.push(c2);
        }
        state = SnakeState::TXT;
      }
    }
  }  

  return buf.clone();
}

/**
* converts lower snake case to upper camel case
*/
pub fn lsnake_to_ucamel(txt:&str) -> String {
  let mut buf:String = String::new();
  let mut state = SnakeState::TXT;

  for c in txt.chars() {
    match state {
      SnakeState::TXT => {
        if buf.len() == 0 {
          for c2 in c.to_uppercase() {
            buf.push(c2);
          }
        } else if c == '_' {
          state = SnakeState::UNDERSCORE;
        } else {
          buf.push(c);
        }
      }
      SnakeState::UNDERSCORE => {
        for c2 in c.to_uppercase() {
          buf.push(c2);
        }
        state = SnakeState::TXT;
      }
    }
  }

  return buf.clone();
}

/**
* converts lower camel case to upper camel case
*/
pub fn lcamel_to_ucamel(txt:&str) -> String {
  let mut buf:String = String::new();
  for c in txt.chars() {
    if buf.len() == 0 {
      for c2 in c.to_uppercase() {
        buf.push(c2);
      }
    } else {
      buf.push(c);
    }
  }
  return buf.clone();
}

/**
* converts upper camel case to lower camel case
*/
pub fn ucamel_to_lcamel(txt:&str) -> String {
  let mut buf:String = String::new();
  for c in txt.chars() {
    if buf.len() == 0 {
      for c2 in c.to_lowercase() {
        buf.push(c2);
      }
    } else {
      buf.push(c);
    }
  }
  return buf.clone();
}

pub fn to_upper(txt:&str) -> String {
  let mut buf:String = String::new();
  for c in txt.chars() {
    for c2 in c.to_uppercase() {
      buf.push(c2);
    }
  }
  return buf.clone();
}

/**
* Removes the first char from a constant string
* (For example the first slash from paths)
*/
pub fn remove_first_char(txt:&str) -> String {
  let mut buf:String = String::new();
  let mut i = 0;
  for c in txt.chars() {
    if i > 0 {
      buf.push(c);
    }
    i += 1;
  }
  return buf.clone();
}

enum SnakeState {
  TXT, UNDERSCORE
}
