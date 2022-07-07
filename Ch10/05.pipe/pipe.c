/*
 * pipe.c: demostrates how to create a pipe from one process to another
 * 
 * action: takes two arguments, each one is a command, and connects av[1]'s
 *         output to av[2]'s input
 *         
 * uage: pipe command1 command2
 * effect: command1|command2
 * 
 * liminations: commands do not take arguments
 * 
 * uses execlp() since known number of args
*/

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#define oops(m, x)         \
	{                  \
		perror(m); \
		exit(x);   \
	}

int main(int ac, char **av)
{
	int pipe_fd[2] = { -1, -1 };

	if (ac != 3) {
		fprintf(stderr, "usage: pipe cmd1 cmd2\n");
		exit(1);
	}

	if (pipe(pipe_fd) == -1) {
		oops("Can not get a pipe", 1);
	}

	switch (fork()) {
	case -1:
		oops("Can not fork", 2);
		break;
	case 0: // In the child-process
		close(pipe_fd[0]);
		dup2(pipe_fd[1], 1);
		close(pipe_fd[1]);
		execlp(av[1], av[1], NULL);
		oops(av[1], 5); // error handling
		break;
	default: // In the parent-process
		close(pipe_fd[1]);
		dup2(pipe_fd[0], 0);
		close(pipe_fd[0]);
		execlp(av[2], av[2], NULL);
		oops(av[2], 4); // error handling
	}

	return 0;
}