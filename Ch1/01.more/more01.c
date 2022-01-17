/*
 * more01.c - version 0.1 of more
 * read and print 24 lines then pause for a few special commands
 */
#include <stdio.h>
#include <stdlib.h>
#define PAGELEN 24
#define LINELEN 512

void do_more(FILE * );
int see_more();

int main(int ac, char * av[]){
    FILE * fp;
    if (ac == 1) {
        // no argument is supplied, read from stdin.
        do_more(stdin);
    }else {
        // support multiple files
        while (--ac) {
            if ((fp= fopen(* ++av, "r")) != NULL) {
                do_more(fp);
                fclose(fp);
            }else{
                // exit if we encounter an error when attempting to open the file
                exit(1);
            }
        }
    }
}


/*
 * read PAGELEN lines, then call see_more() for further instructions
 */
void do_more(FILE * fp){
    char line[LINELEN];   // buffer for the line contents
    int num_of_line = 0;  // record how many lines we have read relatively, which ranges from  0 to 24
    // receive the instruction of see_more(), which is encoded into 0(exit)/1(more line to read)/24(more PAGE to read)
    int reply = 0;

    while (NULL != fgets(line, LINELEN, fp)) {
        if (num_of_line == PAGELEN) {
            reply = see_more();
//            printf("debug: reply = %d", reply);
            if (reply == 0) {
                break;
            }
            num_of_line -= reply;
        }
        if (fputs(line, stdout) == EOF) {
            exit(1);
        }
        num_of_line++;
    }
}

/*
 * print a message, wait for response, return # of lines to advance
 * q means quit, space means yes, CR means one line
 */
int see_more(){
//    printf("debug: see_more() is executed");
    int c = 0;
    printf("\033[7m more? \033[m");
    while ((c = getchar()) != EOF) {
        if (c == 'q') {
//            printf("debug: quit");
            return 0;
        }
        if (c == ' ') {
//            printf("debug: one more page");
            return PAGELEN;
        }
        if (c=='\n') {
//            printf("debug: one more line");
            return 1;
        }
    }
}