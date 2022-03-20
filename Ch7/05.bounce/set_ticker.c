#include <sys/time.h>
#include <stdio.h>

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