/*
    hello3.c
    purpose: using refresh and sleep for animated effects
    outline: initialize, draw stuff, wrap up
*/
#include <stdio.h>
#include <unistd.h>
#include <curses.h>

int main() {
    initscr();
    clear();

    for (int i = 0; i < LINES; i++) {
        move(i, i+1);

        if (i%2 == 1) {
            standout();
        }
        addstr("Hello world");
        if (i%2 == 1) {
            standend();
        }
        sleep(1);
        refresh();
    }
    getch();
    endwin();
    return 0;
}
