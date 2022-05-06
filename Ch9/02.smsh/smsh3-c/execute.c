/*
    execute.c
    code used by small shell to eecute commands
*/

#include <stdio.h>
#include <stdlib.h>
#include <signal.h>
#include <sys/wait.h>
#include <assert.h>
#include <unistd.h>

/*
    purpose: run a program passing its arguments
    returns status retutned via wait, or -1 on error
    errors: -1 on fork() or wait() errors
*/
int execute(char *argv[]) {
    pid_t pid;
    int child_info = -1;


    if (NULL == argv[0]) {
        return 0;
    }

    if (-1 == (pid = fork())) {
        perror("fork()");
        return -1;
    } else if (0 == pid) {
        // enable default signal handling for SIGINT and SIGQUIT        
        signal(SIGINT, SIG_DFL);
        signal(SIGQUIT, SIG_DFL);

        // execute the command
        if (-1 == execvp(argv[0], argv)) {
            perror("execvp()");
            exit(-1);
        }
    } else {
        if (-1 == wait(&child_info)) {
            perror("wait()");
            return -1;
        }
    }

    return child_info;
}