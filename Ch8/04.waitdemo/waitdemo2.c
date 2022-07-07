/*
    waitdemo2.c
    shows how parent get child status
*/
#include <stdio.h>
#include <unistd.h>
#include <assert.h>
#include <sys/wait.h>
#include <stdlib.h>

#define DELAY 10

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
	exit(17);
}

void parent_code(int child_pid)
{
	int child_status = 0;
	int high_8 = 0;
	int low_7 = 0;
	int bit_7 = 0;
	int wait_rv = wait(&child_status);

	printf("done waiting for %d. wait() returned: %d\n", child_pid,
	       wait_rv);
	high_8 = child_status >> 8;
	low_7 = child_status & 0x7f;
	bit_7 = (child_status & 0x80) != 0;

	printf("status: exit=%d, sig=%d, core=%d\n", high_8, low_7, bit_7);
}