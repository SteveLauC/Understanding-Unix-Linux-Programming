
/*
  play_again2.c
  purpose: ask if user wants another transaction
  method: set tty into char-by-char mode, read char, return result
  returns: 0=>yes, 1=>no
  better: timeout if user walks away
*/

#include <stdio.h>
#include <stdlib.h>
#include <termio.h>

#define QUESTION "Do you want another transaction"

/*
  purpose: ask a question and wait for a y/n answer
  method: use getchar and ingore all non y/n answers
  returns: 0=>yes, 1=>no
*/
int get_response(char * question) {
    printf("%s (y/n)", QUESTION);
    while(1) {
        switch (getchar()) {
            case 'y':
            case 'Y':
                return 0;
            case 'n':
            case 'N':
            case EOF:
                return 1;
        }
    }
}

/*
  purpose: put file descriptor 0(i.e. stdin) into char-by-char and noecho mode
  method: disable ICANON and ECHO bits and set tty.c_cc[VMIN] to 1
*/
void set_cr_noecho_mode() {
    struct termios ttyinfo;
    if (-1 == tcgetattr(0, &ttyinfo)) {
        perror("tcgetattr");
        exit(EXIT_FAILURE);
    }

    ttyinfo.c_lflag &= ~ICANON; // disable ICANON bit
    ttyinfo.c_lflag &= ~ECHO;   // disable ECHO bit
    ttyinfo.c_cc[VMIN] = 1;     // set minimum number of bytes been read to 1

    if (-1 == tcsetattr(0, TCSANOW, &ttyinfo)) {
        perror("tcsetattr");
        exit(EXIT_FAILURE);
    }
}

/* 
  store and restore the original tty setting
  how = 0: store the setting
  how = 1: restore the setting
*/
void tty_mode(int how) {
    static struct termios orig_mode;
    if (how == 0) {
        tcgetattr(0, &orig_mode);
    } else if (how == 1) {
        tcsetattr(0, TCSANOW, &orig_mode);
    }
}

int main() {
    int response = 0;
    tty_mode(0);
    set_cr_noecho_mode();
    response = get_response(QUESTION);
    tty_mode(1);
    return response;
}