/*
 * write0.c
 * purpose: send messages to another terminal
 * method: open the other terminal for output then
 *         copy from stdin to that terminal
 * shows: a terminal is just a file supporting regular i/o
 * usage: write0 ttyname
*/ 

#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <fcntl.h>
#include <string.h>

int main(int ac, char *av[]) {
    int fd = 0;
    char buf[BUFSIZ];
    
    // check args
    if (ac != 2) {
        fprintf(stderr, "Usage: write0 ttyname\n");
        exit(-1);
    }

    // open devices
    fd = open(av[1], O_WRONLY);
    if (fd == -1) {
        perror(av[1]);
        exit(-1);
    }

    // loop until EOF on input
    while (fgets(buf, BUFSIZ, stdin) != NULL) {
		printf("one iteration executed\n");
        if (write(fd, buf, strlen(buf)) == -1) {
            break;
        }
		printf("write one piece of contents\n");
    }
    close(fd);
}
