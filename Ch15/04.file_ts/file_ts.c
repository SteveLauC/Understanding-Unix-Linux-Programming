/*
 * file_ts.c: read the current date/time from a file
 *
 * usage: ./file_ts filename
 *
 * action: writes the current time/date to filename
 *
 * note: uses `fcntl` -based lockging
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <time.h>

#define oops(m, x) {perror(m); exit(x);}

void lock_operation(int , int );

int main(int ac, char * av[]) {
    int fd;
    time_t now;
    char * message = NULL;

    if (ac != 2) {
        fprintf(stderr, "usage: ./file_ts filename");
        exit(1);
    }

    if ((fd = open(av[1], O_WRONLY|O_CREAT|O_TRUNC, 0644)) == -1) {
        oops(av[1], 2);
    }

    while (1) {
        time(&now);
        message = ctime(&now);

        // lock the file for write purpose
        lock_operation(fd, F_WRLCK);

        // move the cursor back to start
        if (lseek(fd, 0, SEEK_SET) == -1) {
            oops("lseek", 3);
        }
        // printf("debug: cursor: %ld\n", lseek(fd, 0, SEEK_CUR));
        if(write(fd, message, strlen(message)) == -1) {
            oops("write", 4);
        }

        // unlock the file
        lock_operation(fd, F_UNLCK);
        sleep(1);
    }
}

void lock_operation(int fd, int op) {
    struct flock lock_config;
    lock_config.l_type = op;
    lock_config.l_len = 0;
    lock_config.l_start = 0;
    lock_config.l_whence = SEEK_SET;
    lock_config.l_pid = getpid();

    if (-1 == fcntl(fd, F_SETLKW, &lock_config)) {
        oops("lock operation", 6);
    }
}
