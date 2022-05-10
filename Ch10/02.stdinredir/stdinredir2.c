/*
 * stdinredir2.c: use open-close-dup-close to redirect stdin to a file
*/

#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>
#include <unistd.h>
#include <assert.h>

int main() {
    int fd = open("/etc/passwd", O_RDONLY);
    assert(fd != -1);
    assert(close(0) == 0);
    assert(dup(fd) == 0); // we need the new file descriptor to be 0
    assert(close(fd) == 0);

    char buf[101] = {'\0'};
    
    fgets(buf, 100, stdin);
    printf("%s\n", buf);
}