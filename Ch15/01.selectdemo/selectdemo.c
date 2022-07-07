/*
 * selectdemo.c: watch for input on two devices
 *
 * usage: ./selectdemo dev1 dev2 timeout
 *
 * action: reports on input from each file, and reports timeouts
*/

#include <stdio.h>
#include <stdlib.h>
#include <sys/select.h>
#include <time.h>
#include <sys/types.h>
#include <unistd.h>
#include <fcntl.h>

void show_data(char *, int);

#define oops(m, x)         \
	{                  \
		perror(m); \
		exit(x);   \
	}

int main(int ac, char *av[])
{
	if (ac != 4) {
		fprintf(stderr, "usage: ./selectdemo file1 file2 timeout");
		exit(1);
	}

	int fd1 = open(av[1], O_RDONLY);
	if (fd1 == -1) {
		oops(av[1], 2);
	}
	int fd2 = open(av[2], O_RDONLY);
	if (fd2 == -1) {
		oops(av[2], 3);
	}

	int max_fd = 1 + (fd1 > fd2 ? fd1 : fd2);

	fd_set read_fds;
	struct timeval timeout;
	int ret_val = -1;
	while (1) {
		// make a list of file descriptors to watch
		FD_ZERO(&read_fds);
		FD_SET(fd1, &read_fds);
		FD_SET(fd2, &read_fds);

		// set timeout value
		// NOTE: the `select` syscall will modify `timeout`, this is why we reset it in every loop
		timeout.tv_sec = (long)atoi(av[3]);
		timeout.tv_usec = 0;

		// wait for input
		ret_val = select(max_fd, &read_fds, NULL, NULL, &timeout);
		if (ret_val == -1) {
			oops("select", 4);
		}

		// when we have message(s)
		if (ret_val > 0) {
			if (FD_ISSET(fd1, &read_fds)) {
				show_data(av[1], fd1);
			}
			if (FD_ISSET(fd2, &read_fds)) {
				show_data(av[2], fd2);
			}
		} else {
			printf("no input after %d seconds\n", atoi(av[3]));
		}
	}
}

void show_data(char *file_name, int fd)
{
	char buf[BUFSIZ];

	printf("%s: ", file_name);
	fflush(stdout);
	int n = read(fd, buf, BUFSIZ);
	if (n == -1) {
		oops(file_name, 5);
	}
	write(1, buf, n);
	write(1, "\n", 1);
}
