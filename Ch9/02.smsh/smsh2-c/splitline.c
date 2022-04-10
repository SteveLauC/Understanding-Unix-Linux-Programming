/*
    split_line.c
    command reading and parsing functions for smsh
    char * next_cmd(char * prompt, FILE * fp) get next command
    char ** splitline(char * str) parse a string
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "smsh.h"




/*
    purpose: read next line from fp
    return: valid ptr pointing to the command or NULL when encounters EOF
    action:    
*/
char * next_cmd(char * prompt, FILE * fp) {
    char * buf = NULL; // ptr to the heap memory
    int capacity = 0;  // how many bytes do we have on the heap
    int len = 0;       // how many bytes have we used
    int c = EOF;       // intput char

    // prompt user
    printf("%s", prompt);
    fflush(stdout);

    while ((c = getc(fp)) != EOF) {
        // are we running out of allocated heap memory
        if (len + 1 >= capacity) {
            if (0==capacity) {
                buf = emalloc(BUFSIZ);
            }else {
                buf = erealloc(buf, capacity+BUFSIZ);
            }
            capacity += BUFSIZ;
        }

        // already got a line
        if ('\n' == c) {
            break;
        }

        // put the char to the buf
        buf[len] = c;
        len += 1;
    }

    // we have got nothing
    if (EOF == c && len == 0) {
        return NULL;
    }

    // return the valid command
    buf[len] = '\0';
    return buf;
}


/*
    purpose: split a line into a array of white space separated tokens
    return: a NULL-terminated array of pointers to copies of the token or NULL 
        if no tokens on the line
    action: travese the array, locate strings, make copies
*/
char ** splitline(char * line) {
    if (NULL == line) {
        return NULL;
    }

    char ** arg = emalloc(BUFSIZ);
    int slot_cap = BUFSIZ/sizeof(char *);
    int slot_used = 0; 
    int buf_size = BUFSIZ;

    char * cp = line;
    char *del = " ";
    char * token = strtok(cp, del);
    while (NULL != token) {
        // check if our slot is sufficient
        if (slot_used + 1 >= slot_cap) {
            arg = erealloc(arg, buf_size+BUFSIZ);
            buf_size+=BUFSIZ;
            slot_cap += BUFSIZ/sizeof(char *);
        }

        // allocate the memory for arg
        arg[slot_used] = newstr(token, strlen(token));
        slot_used+=1;

        token = strtok(NULL, del);
    }

    // append the NULL the arg
    arg[slot_used] = NULL;
    return arg;
}

/*
    purpose: constructor for strings
    returns: a string, never NULL
*/
char * newstr(char * s, int l) {
    assert(NULL != s);

    char * rv = emalloc(l+1);
    rv[l] = '\0';
    strncpy(rv, s, l);
    return rv;
}

/*
    purpose: free the list returned by slitline
    returns: nothing
    action: free all the strings in  list and then free the list
*/
void freelist(char **list) {
    if (NULL == list) {
        return;
    }

    char **cp = list;
    while (*cp) {
        free(*cp);
        *cp = NULL;
        cp += 1;
    }

    free(list);
}

/*
    purpose: allocate n bytes on the heap and return the first address of it
        if it fails, do the error handling automatically
    return: first address of the allocated heap memory
*/
void * emalloc(size_t n) {
    void * rv = NULL;
    if ((rv = malloc(n)) == NULL) {
        fatal("out of memory", "", 1);
    }

    return rv;
}

/*
    purpose: change the size of memory block pointed by p to size n
        if it fails, do the error handling automatically
    return: first address of reallocated heap memory
*/
void * erealloc(void * p, size_t n) {
    void * rv = NULL;
    if ((rv = realloc(p, n)) == NULL) {
        fatal("realloc() failed", "", 1);
    }

    return rv;
}