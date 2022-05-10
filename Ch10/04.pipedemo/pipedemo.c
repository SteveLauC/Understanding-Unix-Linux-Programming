/*
 * pipedemo.c: demostrate how to create and use a pipe
 * 
 * action: 
 *          1. creates a pipe
 *          2.writes into the write end 
 *          3. reads from the read end to the buffer
 *          4.prints it out
*/
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main() {
    int a_pipe[2] = {0};
    char buf[101] = {'\0'};
    int len = 0;
    
    // get a pipe 
    if (pipe(a_pipe) == -1) {
        perror("pipe");
        exit(1);
    }
    printf("get a pipe, its file descriptors are: %d & %d\n", a_pipe[0], a_pipe[1]);
    
    // reads from stdin
    fgets(buf, 100, stdin);
    
    len = strlen(buf);
    
    // write to the write end
    if (len != write(a_pipe[1], buf, len)) {
        perror("write");
        exit(1);
    }
    close(a_pipe[1]);        // close the write end 
    // wipe the buffer
    for(int i = 0; i < len; i+=1) {
        buf[i] = '\0';
    }
    
    // read from the read end 
    len = read(a_pipe[0], buf, 100);
    if (len == -1) {
        perror("read");
        exit(1);
    }
    close(a_pipe[0]);

    printf("%s\n", buf);
}