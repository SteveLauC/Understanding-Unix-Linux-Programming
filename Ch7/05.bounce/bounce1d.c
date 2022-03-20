/*
    bounce1d.c
    purpose: animation with user controled speed and direction
    note: the handler does the animation, the main program reads the input
    compile: gcc bounce1d.c set_ticker.c -l curses -o bounce1d
    weird bug: when dir is -1 and MESSAGE is going back, there will be some redundant o in its tail
*/

#include <stdio.h>
#include <curses.h>
#include <signal.h>
#include <string.h>

// seme global settings main and the handler use
#define MESSAGE "hello"
#define BLANK   "     "

int set_ticker(int);

int row = 10;        // current row
int col = 0;         // current column
int dir = 1;         // where we are going
int MESSAGE_LEN = strlen(MESSAGE);

void move_msg(int signum) {
    // reset, just in case.
    signal(SIGALRM, move_msg);  

    move(row, col);
    addstr(BLANK);
    // fprintf(stderr, "Write %ld spaces to %d\n", strlen(BLANK), col);
    col += 2*dir;
    addstr(MESSAGE);
    // fprintf(stderr, "Write %d chars to %d\n", MESSAGE_LEN, col);
    refresh();

    // handle borders
    if (dir == -1 && col <= 0) {
        dir = 1;
    }

    if (dir == 1&&(col+MESSAGE_LEN) >= COLS) {
        dir = -1;
    }
}

int main() {
    int delay = 1000; // 200 milliseconds = 0.2 seconds
    int ndelay;   // new delay
    int c;        // user input


    initscr();
    crmode();     // make tty enter char-by-char mode
    noecho();     // disable echo bit
    clear();

    move(row, col);
    addstr(MESSAGE);
    signal(SIGALRM, move_msg);
    set_ticker(delay);

    while(1) {
        ndelay = 0;
        c = getch();
        if (c == 'Q'){
            break;
        }

        if (c == ' ') {
            dir = -dir;
        }
        if (c == 'f' && delay > 2) {
            ndelay = delay/2;
        }

        if (c == 's') {
            ndelay = delay*2;
        }

        if (ndelay > 0) {
            delay = ndelay;
            set_ticker(delay);
        }
    }

    endwin();
    return 0;
}