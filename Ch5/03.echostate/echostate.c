/*
 * echostate.c
 * reports currnet state of echo bit in tty driver for fd 0
 * shows how to read attributes from driver and test a bit
*/

#include <stdio.h>
#include <stdlib.h>
#include <termio.h>

int main()
{
	struct termios buf;

	if (-1 == tcgetattr(0, &buf)) {
		perror("tcgetattr");
		exit(EXIT_FAILURE);
	}

	if (buf.c_lflag & ECHO) {
		printf("echo is on, since its bit is 1\n");
	} else {
		printf("echo is OFF, since its bit is 0\n");
	}
	return 0;
}