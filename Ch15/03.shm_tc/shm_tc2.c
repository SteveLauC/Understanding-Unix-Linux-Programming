/*
 * shm_tc2.c: time client shared memory version2
 *
 * use semaphore for locking
 *
 * program uses shared memory with key 99
 * program uses semaphore set with key 9900
*/

#include <stdio.h>
#include <stdlib.h>
#include <sys/shm.h>
#include <sys/sem.h>
#include <time.h>

#define TIME_MEM_KEY 99
#define SEG_SIZE ((size_t)100)
#define oops(m, x){perror(m); exit(x);}
