/*
    forkdemo3.c
    shows how the return value of fork() allows to determine whether it is a child process
*/

#include <stdio.h>
#include <unistd.h>

int main()
{
	printf("Before: my pid is %d\n", getpid());

	pid_t fork_rv = fork();

	if (-1 == fork_rv) {
		perror("fork()");
	} else if (0 == fork_rv) {
		printf("I am the child process. my pid = %d\n", getpid());
	} else {
		printf("I am the parent process. my pid = %d\n", getpid());
	}
	return 0;
}