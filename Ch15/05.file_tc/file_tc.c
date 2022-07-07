/*
 * file_tc.c: read the current data/time from a file
 *
 * usage: ./file_tc filename
 *
 * uses: `fcntl`-based locking
*/

#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <fcntl.h>

#define oops(m, x)         \
	{                  \
		perror(m); \
		exit(x);   \
	}
#define BUF_LEN 30

void lock_operation(int, int);

int main(int ac, char *av[])
{
	int fd = -1;
	char buf[BUF_LEN];
	int n_read = 0;

	if (ac != 2) {
		fprintf(stderr, "usage: ./file_tc filename");
		exit(1);
	}

	if ((fd = open(av[1], O_RDONLY)) == -1) {
		oops(av[1], 2);
	}

	lock_operation(fd, F_RDLCK);

	while ((n_read = read(fd, buf, BUF_LEN)) > 0) {
		write(1, buf, n_read);
	}

	lock_operation(fd, F_UNLCK);
	close(fd);
	return 0;
}

void lock_operation(int fd, int op)
{
	struct flock lock_config;
	lock_config.l_type = op;
	lock_config.l_len = 0;
	lock_config.l_start = 0;
	lock_config.l_whence = SEEK_SET;
	lock_config.l_pid = getpid();

	if (-1 == fcntl(fd, F_SETLKW, &lock_config)) {
		oops("lock operation", 6);
	}
}
