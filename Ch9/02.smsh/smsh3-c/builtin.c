/*
 * builtin.c: contains the switch and the functions for the built-in commands
 * 
 * Currently, the supported built-in commands are as follows:
 *      1. set: print all the shell variables
 *      2. var=value: initialize a shell variable
 *      3. export: make an existed global
*/

#include <stdio.h>
#include <string.h>
#include <ctype.h>
#include <assert.h>
#include "smsh.h"
#include "varlib.h"



/*
 * purpose: determine if a string is a legal varable name
 *
 * action: a valid variable should be:
 *         1. can not start with number
 *         2. every char of it should be a letter or a number(0-9) or a underscore
 *         3. length of it should be bigger or equal to 1
 *
 * arguments:
 *  * `str`: variable name
 *  
 * return: true on valid name, false on invalid one
*/
bool okname(char * str) {
    assert(str != NULL);
    
    char * c;
    for (c = str; *c != 0; c+=1) {
        if ((isdigit((unsigned char)*c)&&c==str) || !(isalnum((unsigned char)*c)||*c=='_')){
            return false;
        }
    }
    
    return (str!=c);
}


/*
 * purpose: ensure variable is valid and execute the assignment
 * 
 * arguments: 
 *  * `str`: assignment statement
 * 
 * return: -1 on illegal variable name, or the result of VLstore
*/
int assign(char * str) {
    assert(str != NULL);

    char * equal_sign = "=";
    char * equal_sign_idx = strchr(str, '=');
    assert(equal_sign_idx != NULL);

    char * var = strtok(str, equal_sign);
    assert(var != NULL);
    if (okname(var)) {
        return VLstore(var, equal_sign_idx+1);
    } else {
        return -1;
    }
}

/*
 * purpose: check if the command is one of the supported built-in commands
 *
 * action: check if the command is `set/export` or contains `=`
 * 
 * arguments: 
 *  * `cmd`: command
 *
 * return: true on built-in commands; otherwise, return false
*/
bool is_built_in_command(char * cmd) {
    return ( 0==strcmp(cmd, "set") || 0==strcmp(cmd, "export")|| strchr(cmd, '=')!= NULL);
}

/*
 * purpose: run the built-in commands
 * 
 * action:
 *
 *
 * arguments: 
 *  * `args`: command words list
 *  
 * return: 
*/
int builtin_command(char ** args) {
    if (strcmp(args[0], "set") == 0) {
        VLlist(); 
        return 0;
    }
    
    if (strcmp(args[0], "export") == 0) {
        if (args[1]!=NULL && okname(args[1]))  {
            return VLexport(args[1]); 
        } else {
            return 1;
        }
    }
    
    if (strchr(args[0], '=') != NULL) {
        return assign(args[0]); 
    }
    

    // unreachable
    return 0;
}