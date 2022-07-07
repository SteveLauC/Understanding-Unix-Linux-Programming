/*
 * varlib.h: header for varlib.c package
 */

int VLenviron2table(char **);
int VLexport(char *);
char *VLlookup(char *);
void VLlist();
int VLstore(char *, char *);
char **VLtable2environ();
int VLenviron2table(char **);