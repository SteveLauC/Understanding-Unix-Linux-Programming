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
#define TIME_SEM_KEY 9900
#define SEG_SIZE ((size_t)100)
#define oops(m, x) {perror(m); exit(x);}

void wait_and_lock(int);
void release_lock(int);

int main(void) {
    int seg_id;
    int sem_set_id;
    char * mem_ptr = NULL;

    seg_id = shmget(TIME_MEM_KEY, SEG_SIZE, 0777);
    if (seg_id == -1) {
        oops("shmget", 1);
    }

    mem_ptr = shmat(seg_id, NULL, 0);
    if (mem_ptr == (void *)-1) {
        oops("shmat", 2);
    }

    sem_set_id = semget(TIME_SEM_KEY, 2, 0);
    if (sem_set_id == -1) {
        oops("shmat", 3);
    }
    wait_and_lock(sem_set_id);
    printf("The time, direct from memory: %s", mem_ptr);
    release_lock(sem_set_id);
    shmdt(mem_ptr);
}

void wait_and_lock(int sem_set_id) {
    struct sembuf action[2]; 
    action[0].sem_num = 1;
    action[0].sem_flg = SEM_UNDO;
    action[0].sem_op = 0;

    action[1].sem_num = 0;
    action[1].sem_flg = SEM_UNDO;
    action[1].sem_op = +1;

    if (semop(sem_set_id, action, 2) == -1) {
        oops("semop: locking", 10);
    }
}

void release_lock(int sem_set_id) {
    struct sembuf action[1];
    action[0].sem_num = 0;
    action[0].sem_flg = SEM_UNDO;
    action[0].sem_op = -1;

    if (semop(sem_set_id, action, 1) == -1) {
        oops("semop: unlocking", 10);
    }
}
