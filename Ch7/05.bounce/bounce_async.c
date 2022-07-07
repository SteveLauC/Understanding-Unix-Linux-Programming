/*
    bounce_async.c
    purpose: animation with usercontrl, using O_ASYNC on fd
    note: set_ticker() sends SIGALRM, handler does animation
          keyboard sends SIGIO, main only calls pause()
    compile: gcc bounce_async.c set_ticker.c -l curses -o bounce_async
*/

#include <stdio.h>
#include <curses.h>
#include <signal.h>
#include <fcntl.h>
#include <unistd.h>
#include <string.h>

#define MSG "hello"
#define BLK "     "

int row = 10;
int col = 0;
int dir = 1;
int delay = 200;
int done = 0;

void on_alarm(int signum);
void on_input(int signum);
void enable_kbd_signals();
int set_ticker(int n_msecs);

int main()
{
	initscr();
	crmode();
	noecho();
	clear();

	signal(SIGIO, on_input);
	enable_kbd_signals();
	signal(SIGALRM, on_alarm);
	set_ticker(delay);

	move(row, col);
	addstr(MSG);

	while (!done) {
		pause();
	}

	endwin();
	return 0;
}

void on_input(int signum)
{
	int c = getch();
	if (c == 'Q' || c == EOF) {
		done = 1;
	} else if (c == ' ') {
		dir *= -1;
	}
}

void on_alarm(int signum)
{
	signal(SIGALRM, on_alarm);
	mvaddstr(row, col, BLK);
	col += dir;
	mvaddstr(row, col, MSG);
	refresh();

	// handle bounders
	if (dir == -1 && col <= 0) {
		dir = 1;
	}
	if (dir == 1 && col + strlen(MSG) >= COLS) {
		dir = -1;
	}
}

void enable_kbd_signals()
{
	fcntl(0, F_SETOWN, getpid());
	fcntl(0, F_SETFL, fcntl(0, F_GETFL) | O_ASYNC);
}
