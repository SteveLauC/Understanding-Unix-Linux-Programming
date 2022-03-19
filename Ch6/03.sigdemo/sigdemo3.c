/*
    sigdemo3.c
    purpose: show answers to signal questions

    question1: does the handler stay in effect after a signal arrives 
    answer1: yes

    question2: what if a signalX arrives while handling signalX? 
    answer2: second signal is blocked, and they will be handled in order

    question3: what if a signalX arrives while handling signalY?
    anser3: system will handle signalY while it is hanling signalX,
    as if it were a recurisive function call(i.e. call signalY handler in signalX handler) 

    question4: wht happens to read() when a signal arrives?
    answer4: read function will take effect when signal handling is done
*/

#include <stdio.h>
#include <signal.h>
#include <string.h>
#include <unistd.h>

#define INPUTLEN 100


void int_handler(int signum) {
    printf("Recived signal %d .. waiting\n", signum);
    sleep(2);
    printf("Leaving int_handler\n");
}

void quit_handler(int signum) {
    printf("Received signal %d .. waiting\n", signum);
    sleep(3);
    printf("Leaving quit_handler\n");
}

int main() {
    char input[INPUTLEN];
    *input = '\0';
    int nchars = 0;

    signal(SIGINT, int_handler);
    signal(SIGQUIT, quit_handler);

    do{
        printf("\nType a message\n");
        nchars=read(0, &input, INPUTLEN);

        if (-1 == nchars) {
            perror("read returned an error");
        }else{
            input[nchars] = '\0';
            printf("You typed: %s\n", input);
        }
    }while(strncmp(input, "quit", 4)!=0);

    return 0;
}