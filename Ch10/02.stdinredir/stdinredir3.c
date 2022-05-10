/*
 * stdinredir3.c: use open-dup2-close to redirect stdin to a file
*/

#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>
#include <assert.h>

int main() {
    int old_fd = open("/etc/passwd", O_RDONLY);
    assert(dup2(old_fd, 0)==0);
    assert(close(old_fd) == 0);
    
    char buf[101] = {'\0'};
    fgets(buf, 100, stdin);
    printf("%s\n", buf);

    return 0;
}