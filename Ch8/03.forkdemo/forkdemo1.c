/*
    forkdemo1.c
    shows how fork creates two processes, distinguishable by the difference return values from fork()
*/

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main()
{
	pid_t mypid = getpid();
	printf("Before: my pid is %d\n", mypid);

	pid_t ret_from_fork = fork();
	if (-1 == ret_from_fork) {
		perror("fork()");
		exit(EXIT_FAILURE);
	}

	sleep(1);

	printf("After: my pid is %d, fork() said %d\n", getpid(),
	       ret_from_fork);

	return 0;
}