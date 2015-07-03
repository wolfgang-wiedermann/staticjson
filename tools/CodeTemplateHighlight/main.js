/*jslint vars: true, plusplus: true, devel: true, nomen: true, regexp: true, indent: 4, maxerr: 50 */
/*global define, $, brackets, window */

/** Simple extension that adds a "File > Hello World" menu item */
define(function (require, exports, module) {
    "use strict";

    var CommandManager  = brackets.getModule("command/CommandManager"),
        LanguageManager = brackets.getModule("language/LanguageManager"),
        CodeMirror      = brackets.getModule("thirdparty/CodeMirror2/lib/codemirror"),
        Menus           = brackets.getModule("command/Menus");


    // Function to run when the menu item is clicked
    function handleHelloWorld() {
        window.alert("Hello, world!");
    }
    
    // Code-Mirror-Mode for CodeTemplates
    // Example from http://marijnhaverbeke.nl/blog/codemirror-mode-system.html
    CodeMirror.defineMode("strings", function() {
      return {
        startState: function() {return {inString: false};},
        token: function(stream, state) {
          // If a string starts here
          if (!state.inString && stream.peek() == '{') {
            stream.next();            // Skip quote
            if(stream.peek() == '{') {
                stream.next();            
                state.inString = true;    // Update state
            }
          }

          if (state.inString) {
            if (stream.skipTo('}')) { // Quote found on this line
              stream.next();          // Skip quote
              if(stream.peek() === '}') {
                stream.next();
                state.inString = false; // Clear flag
              }
            } else {
               stream.skipToEnd();    // Rest of line is string
            }
            return "string";          // Token style
          } else {
            stream.skipTo('{') || stream.skipToEnd();
            return null;              // Unstyled token
          }
        }
      };
    });
    
    // Register CodeMirror-Mode for ct-Files
    LanguageManager.defineLanguage("codetemplate", {
      name: "Code Template",
      mode: "strings",
      fileExtensions: ["ct"]
    });
    
    LanguageManager.defineLanguage('rust', {
      name: 'Rust',
      mode: 'rust',
      fileExtensions: ['rs'],
      blockComment: ['/*', '*/'],
      lineComment: ['//']
    });

    // First, register a command - a UI-less object associating an id to a handler
    var MY_COMMAND_ID = "helloworld.sayhello";   // package-style naming to avoid collisions
    CommandManager.register("Hello World", MY_COMMAND_ID, handleHelloWorld);

    // Then create a menu item bound to the command
    // The label of the menu item is the name we gave the command (see above)
    var menu = Menus.getMenu(Menus.AppMenuBar.FILE_MENU);
    menu.addMenuItem(MY_COMMAND_ID);

    // We could also add a key binding at the same time:
    //menu.addMenuItem(MY_COMMAND_ID, "Ctrl-Alt-H");
    // (Note: "Ctrl" is automatically mapped to "Cmd" on Mac)
});