/*
 * setecho.c
 * usage: setecho [y|n]
 * shows: how to read, change, reste tty attributes
*/

#include <stdio.h>
#include <stdlib.h>
#include <termios.h>

#define oops(s, x)         \
	{                  \
		perror(s); \
		exit(x);   \
	}

int main(int ac, char *av[])
{
	struct termios buf;
	if (ac == 1)
		exit(EXIT_SUCCESS);

	if (-1 == tcgetattr(0, &buf)) {
		oops("tcgetattr", EXIT_FAILURE);
	}

	if ('y' == av[1][0]) {
		buf.c_lflag |= ECHO;
	} else {
		buf.c_lflag &= ~ECHO;
	}

	if (-1 == tcsetattr(0, TCSANOW, &buf)) {
		oops("tcsetattr", EXIT_FAILURE);
	}
	return 0;
}