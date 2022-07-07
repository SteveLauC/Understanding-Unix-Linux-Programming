/*
 * tbounce1d.c: controlled animation using two threads
 *
 * note: one thread handles animation, other thread handles keyboard input
 *
*/

#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <curses.h>
#include <pthread.h>
#include <stdlib.h>
#include <unistd.h>

// shared variables both threads use. These need a mutex
#define MESSAGE "hello"
#define BLANK "     "
#define LEN strlen(MESSAGE)

pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;

int row;
int col;
int dir;
int delay;

void *moving_msg(void *);

int main(void)
{
	int ndelay; // new delay
	int c; // user input
	pthread_t msg_thread;

	initscr();
	crmode();
	noecho();
	clear();

	pthread_mutex_lock(&lock);
	row = 10;
	col = 0;
	dir = 1;
	delay = 200;
	move(row, col);
	pthread_mutex_unlock(&lock);
	addstr(MESSAGE);
	refresh();

	if (pthread_create(&msg_thread, NULL, moving_msg, (void *)MESSAGE) !=
	    0) {
		fprintf(stderr, "error creating thread");
		endwin();
		exit(0);
	}

	while (1) {
		ndelay = 0;
		c = getch();

		if (c == 'Q')
			break;
		if (c == ' ') {
			pthread_mutex_lock(&lock);
			dir = -dir;
			pthread_mutex_unlock(&lock);
		}
		if (c == 'f' && delay > 2) {
			pthread_mutex_lock(&lock);
			ndelay = delay / 2;
			pthread_mutex_unlock(&lock);
		}
		if (c == 's') {
			pthread_mutex_lock(&lock);
			ndelay = delay * 2;
			pthread_mutex_unlock(&lock);
		}
		if (ndelay > 0) {
			pthread_mutex_lock(&lock);
			delay = ndelay;
			pthread_mutex_unlock(&lock);
		}
	}

	pthread_cancel(msg_thread);
	endwin();
}

void *moving_msg(void *m)
{
	char *msg = (char *)m;
	while (1) {
		usleep(delay * 1000);
		pthread_mutex_lock(&lock);
		move(row, col);
		addstr(BLANK);
		col += dir;
		move(row, col);
		pthread_mutex_unlock(&lock);
		addstr(msg);
		refresh();

		if (col <= 0 && dir == -1) {
			pthread_mutex_lock(&lock);
			dir = 1;
			pthread_mutex_unlock(&lock);
		} else if (col + (int)LEN >= COLS && dir == 1) {
			pthread_mutex_lock(&lock);
			dir = -1;
			pthread_mutex_unlock(&lock);
		}
	}
}
