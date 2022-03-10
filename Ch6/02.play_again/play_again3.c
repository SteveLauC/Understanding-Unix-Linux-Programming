
/*
  play_again3.c
  purpose: ask if user wants another transaction
  method: set tty into char-by-char mode, read char, return result
  returns: 0=>yes, 1=>no
  better: timeout if user walks away
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <ctype.h>
#include <assert.h>
#include <fcntl.h>
#include <termio.h>

#define TRIES 3
#define SLEEPTIME 2
#define QUESTION "Do you want another transaction"
#define BEEP putchar('\a');

/*
  skip over non-legal chars and return y, Y, n, N or EOF
*/
int get_ok_char() {
    int c;
    // in non-blocking mode, getchar() will return EOF if there is no input for it.
    while ( (c = getchar()) != EOF && strchr("ynYN", c) == NULL) {
        continue;
    }
    // looks like our assertion is useless here, but anyway:)
    assert(c >= -1 && c <= 127);    // if c is not in this range, `tolower(int c)` will trigger UB
                                    // see `man 3 tolower` for more details
    return c;
}

/*
  purpose: ask a question and wait for a y/n answer or maxtries
  method: use getchar and ingore all non y/n answers
  returns: 0=>yes, 1=>no, 2=>timeout
*/
int get_response(char * question, int max_tries) {
    int input;
    printf("%s (y/n)", question);
    fflush(stdout);
    while(1) {
        sleep(SLEEPTIME);     // user has SLEEPTIME seconds to provide the input
        input = tolower(get_ok_char());
        max_tries -= 1;       // already tried once
        if (input == (int)'y') {
            return 0;
        }
        if (input == (int)'n') {
            return 1;
        }
        if (0==max_tries) {
            return 2;
        }
        BEEP;                  // alert user
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
  purpose: put file descriptor 0 into non-blocking mode
  method: use fcntl to set bits
  notes: tcsetattr() will do somthing similar, but it is complicated
*/
void set_non_blocking_mode() {
    int terflags;
    terflags  = fcntl(0, F_GETFD);
    terflags |= O_NONBLOCK;
    fcntl(0, F_SETFD, terflags);
}

/* 
  store and restore the original tty and file setting
  how = 0: store the setting
  how = 1: restore the setting
*/
void tty_mode(int how) {
    static struct termios orig_mode;
    static int orig_flags;
    if (how == 0) {
        tcgetattr(0, &orig_mode);
        orig_flags = fcntl(0, F_GETFL);
    } else if (how == 1) {
        tcsetattr(0, TCSANOW, &orig_mode);
        fcntl(0, F_SETFL, orig_flags);
    }
}

int main() {
    int response = 0;
    tty_mode(0);
    set_cr_noecho_mode();
    set_non_blocking_mode();
    response = get_response(QUESTION, TRIES);
    tty_mode(1);
    return response;
}
