/*
    sleep1.c
    purpose: show how sleep() works, implement sleep using alarm.
    usage: sleep1
    outline: sets handler, sets alarm, pause, then returns
*/

#include <stdio.h>
#include <signal.h>
#include <unistd.h>

void wakeup(int signum)
{
	printf("Alarm received from kernel\n");
}

int main()
{
	signal(SIGALRM, wakeup); // set handler
	printf("About to sleep for 4 seconds\n");
	alarm(4); // set alarm
	pause(); // go to sleep
	printf("Morning so soon?\n"); // morning
	return 0;
}
