/*
    waitdemo1.c
    shows how parent pauses until child finishes
*/
#include <stdio.h>
#include <unistd.h>
#include <assert.h>
#include <sys/wait.h>
#include <stdlib.h>

#define DELAY 2

void child_code(int delay);
void parent_code(pid_t child_pid);

int main()
{
	pid_t newpid = 0;

	printf("before: mypid is %d\n", getpid());

	if (-1 == (newpid = fork())) {
		perror("fork");
	} else if (0 == newpid) {
		child_code(DELAY);
	} else {
		parent_code(newpid);
	}

	return 0;
}

void child_code(int delay)
{
	printf("child %d here. will sleep for %d seconds\n", getpid(), delay);

	unsigned int u_delay = (unsigned int)delay;
	assert((int)u_delay == delay);

	sleep(u_delay);
	printf("child done. about to exit\n");
	exit(10);
}

void parent_code(int child_pid)
{
	// will suspend the parent until child's termination
	int wait_rv = wait(NULL);
	printf("done waiting for %d. wait() returned: %d\n", child_pid,
	       wait_rv);
}