#ifndef smsh
#define smsh

#include <stddef.h>
#include <stdio.h>

// bool type support
#define bool int
#define true 1
#define false 0

// function signatures
char *next_cmd(char *, FILE *);
char *newstr(char *, int);
char **splitline(char *);
void freelist(char **);
void *emalloc(size_t);
void *erealloc(void *, size_t);
int execute(char **);
bool ok_to_execute();
bool is_control_command(char *);
int do_control_command(char **);
int syn_err(char *);
int process(char **);
void fatal(char *, char *, int);
bool is_built_in_command(char *);
int builtin_command(char **);

#endif