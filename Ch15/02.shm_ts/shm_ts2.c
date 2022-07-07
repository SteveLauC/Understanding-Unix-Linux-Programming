/*
 * shm_ts2.c: time server shared memory version 2(use semaphores for locking)
 *
 * program uses shared memory with key 99
 * program uses semaphore set with key 9900
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/shm.h>
#include <time.h>
#include <sys/sem.h>
#include <signal.h>
#include <sys/types.h>

#define TIME_MEM_KEY 99
#define TIME_SEM_KEY 9900
#define SEG_SIZE ((size_t)100)
#define oops(m, x)         \
	{                  \
		perror(m); \
		exit(x);   \
	}

int seg_id;
int sem_set_id;
void clean_up(int);
void set_sem_value(int, int, int);
void wait_and_lock(int);
void release_lock(int);

int main(void)
{
	char *mem_ptr = NULL;
	time_t now;
	signal(SIGINT, clean_up);

	seg_id = shmget(TIME_MEM_KEY, SEG_SIZE, IPC_CREAT | 0777);
	if (seg_id == -1) {
		oops("shmget", 1);
	}

	mem_ptr = shmat(seg_id, NULL, 0);
	if (mem_ptr == (void *)-1) {
		oops("shmat", 2);
	}

	sem_set_id = semget(TIME_SEM_KEY, 2, IPC_CREAT | IPC_EXCL | 0666);
	if (sem_set_id == -1) {
		oops("semget", 3);
	}
	// set both to zero
	semctl(sem_set_id, 0, 0);
	semctl(sem_set_id, 1, 0);

	// server: run for a minute
	for (int i = 0; i < 60; i++) {
		time(&now);
		printf("\tshm_ts2 waiting for lock\n");
		wait_and_lock(sem_set_id);
		printf("\tshm_ts2 updaing memory\n");
		strcpy(mem_ptr, ctime(&now));
		// sleep(5);
		release_lock(sem_set_id);
		printf("\tshm_ts2 release lock\n");
		sleep(1);
	}

	clean_up(0);
}

void clean_up(int _n)
{
	shmctl(seg_id, IPC_RMID, NULL);
	semctl(sem_set_id, 0, IPC_RMID, NULL);
}

void set_sem_value(int sem_set_id, int sem_num, int val)
{
	if (semctl(sem_set_id, sem_num, SETVAL, val) == -1) {
		oops("semctl", 4);
	}
}

void wait_and_lock(int sem_set_id)
{
	struct sembuf action[2];
	action[0].sem_num = 0;
	action[0].sem_flg = SEM_UNDO;
	action[0].sem_op = 0;

	action[1].sem_num = 1;
	action[1].sem_flg = SEM_UNDO;
	action[1].sem_op = +1;

	if (semop(sem_set_id, action, 2) == -1) {
		oops("semop: locking", 10);
	}
}

void release_lock(int sem_set_id)
{
	struct sembuf action[1];
	action[0].sem_num = 1;
	action[0].sem_flg = SEM_UNDO;
	action[0].sem_op = -1;

	if (semop(sem_set_id, action, 1) == -1) {
		oops("semop: unlocking", 10);
	}
}
