#!/usr/bin/python
#
# -*- coding: utf-8 -*-
# Simple commandlinetool to generate code for code generators
#

import sys

#
# Constants
#

# Parser-States
PS_INTEXT = 0
PS_INBRACKETS = 1
PS_INVALUEASSIGNMENT = 2
PS_INCODE = 3

# Parser-SubStates
# (substates of PS_INCODE for indentation issues)
PSS_INRUSTCODE = 0
PSS_INSTRING = 1
PSS_INLINECOMMENT = 2
PSS_INBLOCKCOMMENT = 3

ENC_HTML = "html"
ENC_C = "c"


#
# Function for reading the template file
#
def read_file(filename):
    f = open(filename, 'r')
    string = f.read()
    return string


#
# Converts the given String into a push statement
#
def to_push(string, encodetype, indent):
    if encodetype == "c":
        val = string.replace("\\", "\\\\").replace("\n", "\\n").replace("\t", "\\t").replace("\"", "\\\"")
        return "{1}str.push_str(\"{0}\");".format(val, indent)
    elif encodetype == "html":
        val = string.replace("<", "&lt;").replace(">", "&gt;").replace("\n", "<br>")
        return "{1}str.push_str(\"{0}\");".format(val, indent)


#
# Converts the given String into a formated push statement
#
def to_assignment(string, indent):
    val = string.strip(' ')
    return "{1}str.push_str(&{0});".format(val, indent)
    #return "write!(&mut str, \"{1}\", {0});".format(val, "{}")


#
# Parser-Function
#
def parse(filename, encodetype):
    code = read_file(filename)
    state = PS_INTEXT
    substate = PSS_INRUSTCODE
    buffer = ""
    charbefore = ' '
    indent = ""
    indent_depth = 0

    for c in code:
        if state == PS_INTEXT:
            if c == '{' and charbefore == '{':
                state = PS_INBRACKETS
                if buffer.__len__() > 1:
                    print to_push(buffer[0:buffer.__len__()-2], encodetype, indent)
                buffer = ""
            else:
                buffer += c
        elif state == PS_INBRACKETS:
            if c == '=':
                state = PS_INVALUEASSIGNMENT
            elif c == ' ':
                state = PS_INCODE
            else:
                print "Error: invalid char {0}".format(c)
        elif state == PS_INCODE:
            if c == '}' and charbefore == '}':
                state = PS_INTEXT
                substate = PSS_INRUSTCODE
                print buffer[0:buffer.__len__()-1]
                buffer = ""
            else:
                buffer += c
                #
                # Calculate code indentation for rust code
                #
                if substate == PSS_INRUSTCODE:
                    if c == '{':
                        indent += "  "
                        indent_depth += 1
                        #print("// INDENT+: {0}".format(indent_depth))
                    elif c != '}' and charbefore == '}' and indent_depth > 0:
                        indent_depth -= 1
                        indent = ""
                        for i in range(0, indent_depth):
                            indent += "  "
                        #print("// INDENT-: {0}".format(indent_depth))
                    elif c == '"' and charbefore != "\\":
                        substate = PSS_INSTRING
                    elif c == '*' and charbefore == '/':
                        substate = PSS_INBLOCKCOMMENT
                    elif c == '/' and charbefore == '/':
                        substate = PSS_INLINECOMMENT
                elif substate == PSS_INSTRING:
                    if c == '"' and charbefore != "\\":
                        substate = PSS_INRUSTCODE
                elif substate == PSS_INBLOCKCOMMENT:
                    if c == '/' and charbefore == '*':
                        substate = PSS_INRUSTCODE
                elif substate == PSS_INLINECOMMENT:
                    if c == '\n':
                        substate = PSS_INRUSTCODE
        elif state == PS_INVALUEASSIGNMENT:
            if c == '}' and charbefore == '}':
                state = PS_INTEXT
                print to_assignment(buffer[0:buffer.__len__()-1], indent)
                buffer = ""
            else:
                buffer += c

        charbefore = c

    if state == PS_INTEXT and buffer.__len__() > 1:
        print to_push(buffer, encodetype, indent)


#
# Main
#
#sys.argv.append("../templates/swift_template.ct")
#sys.argv.append("c")

if sys.argv.__len__() == 3:
    parse(sys.argv[1], sys.argv[2])
else:
    print "Usage: generate_code filename [c|html]"
