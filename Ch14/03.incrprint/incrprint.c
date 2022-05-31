/*
 * incrprint.c: one thread increments, the other prints
*/

#include <stdio.h>
#include <unistd.h>
#include <pthread.h>

#define NUM 5

int counter = 0;

void * print_count(void *);

int main(void) {
    pthread_t t1;
    pthread_create(&t1, NULL, print_count, NULL); 
    for(int i = 0; i < NUM; i+=1) {
        counter += 1;
        sleep(1);
    }

    pthread_join(t1, NULL);

    return 0;
}

void * print_count(void *m) {
    for(int i = 0; i < NUM; i+=1) {
        printf("count = %d\n", counter);
        sleep(1); 
    }

    return NULL;
}
