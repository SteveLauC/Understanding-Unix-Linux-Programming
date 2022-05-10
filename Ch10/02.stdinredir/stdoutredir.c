#include <assert.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
    int old_fd = open("out", O_WRONLY|O_CREAT, 0644);
    if (old_fd == -1) {
        perror(NULL);
        exit(1);
    }

    assert(dup2(old_fd, 1) == 1);
    close(old_fd);   
    
    printf("hello world"); // will write to the file `out`
}
