/*
 * lclnt1.c: licence client version 1
*/

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include "lclnt_funcs1.h"

void do_regular_work() {
    printf("SuperSleep version 1 running licenced software");
    sleep(10); 
}

int main() {
    setup();

    if (get_ticket()!=0) {
        exit(0);
    }

    do_regular_work();
    release_ticket();
    shut_down();
}
