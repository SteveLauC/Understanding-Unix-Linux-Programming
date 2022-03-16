/*
 * sigdemo2.c
 * shows how to ignore a signal
 * press Ctrl-\ to kill this process
*/

#include <stdio.h>
#include <unistd.h>
#include <signal.h>



int main() {
    signal(SIGINT, SIG_IGN);

    printf("You can not stop me! \n");
    while (1) {
        sleep(1);
        printf("haha\n");
    }

    return 0;
}
