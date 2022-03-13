/*
    ticker_demo.c
    demostrates use of interval timer to generate regular signals, which are in 
    turn caught and used to count down
*/
#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>
#include <signal.h>
#include <unistd.h>

/*
    handler for signal SIGALRM
*/
void count_down(int signum) {
    static int num = 10;
    printf("%d..", num--);
    fflush(stdout);

    if (num < 0) {
        printf("DONE!\n");
        exit(EXIT_SUCCESS);
    }
}

/*
    [from set_ticker.c]
    set ticker(number of milliseconds)
    arranges for interval timer to issue SIGALRMs at regular intervals
    retrun -1 on error, 0 for ok
    arg in milliseconds, converted into whole seconds and microseconds
    note: set_ticker(0) turns off timer
*/
int set_ticker(int n_msecs) {
    struct itimerval new_timeset;
    long n_secs, n_usecs;

    n_secs = (long)(n_msecs/1000);
    n_usecs = (long)(n_msecs%1000)*1000L;

    new_timeset.it_interval.tv_sec = n_secs;
    new_timeset.it_interval.tv_usec = n_usecs;

    new_timeset.it_value.tv_sec = n_secs;
    new_timeset.it_value.tv_usec = n_usecs;

    return setitimer(ITIMER_REAL, &new_timeset, NULL);
}

int main() {
    signal(SIGALRM, count_down);

    if (-1 == set_ticker(500)) {
        perror(NULL);
        exit(EXIT_FAILURE);
    } else {
        while(1) {
            pause();
        }
    }

    return 0;
}