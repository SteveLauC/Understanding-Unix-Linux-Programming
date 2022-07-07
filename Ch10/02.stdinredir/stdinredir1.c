/*
 * stdinredir.c: show how to redirect standard input by replacing file descriptor 
 *               0 with connection to a file
 *               
 * action: closes fd 0, opens a disk file,
 *         then reads three more lines from standard input
*/

#include <stdlib.h>
#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>

int main()
{
	char buf[101] = { '\0' };
	close(0);

	int fd = open("/etc/passwd", O_RDONLY);
	if (fd != 0) {
		fprintf(stderr, "Can not open data as fd 0");
		exit(1);
	}

	fgets(buf, 100, stdin);
	printf("%s", buf);
	fgets(buf, 100, stdin);
	printf("%s", buf);
	fgets(buf, 100, stdin);
	printf("%s\n", buf);
	close(fd);
	return 0;
}