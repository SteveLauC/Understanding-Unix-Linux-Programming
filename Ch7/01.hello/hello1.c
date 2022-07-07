/*
    hello1.c
    purpose: show the minmal calls needed to use curses
    outline: initialize, draw stuff, wait for input, quit
*/
#include <stdio.h>
#include <curses.h>

int main()
{
	initscr();

	clear();
	move(10, 20);
	addstr("Hello world");
	move(LINES - 1, 0);
	refresh();
	getch();
	endwin();
	return 0;
}
