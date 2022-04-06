#include <stddef.h>

#define YES 1
#define NO 0

char * next_cmd();
char *newstr(char *, int);
char ** splitline(char *);
void freelist(char **);
void *emalloc(size_t);
void *erealloc(void *, size_t);
int execute(char **);
void fatal(char*, char *, int);