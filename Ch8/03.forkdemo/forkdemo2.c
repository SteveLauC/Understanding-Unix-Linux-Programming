/*
    forkdemo2.c
    shows how child process pick up at the return.
    from fork() and can execute any code they like, 
    even fork(). predict number of lines of output.
*/
#include <stdio.h>
#include <unistd.h>

int main()
{
	printf("my pid is %d\n", getpid());
	fork();
	fork();
	fork();
	printf("my pid is %d\n", getpid());

	return 0;
}