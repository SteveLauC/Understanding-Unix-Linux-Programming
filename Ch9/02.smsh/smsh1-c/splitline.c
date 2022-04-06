/*
    split_line.c
    command reading and parsing functions for smsh
    char * next_cmd(char * prompt, FILE * fp) get next command
    char ** splitline(char * str) parse a string
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "smsh.h"