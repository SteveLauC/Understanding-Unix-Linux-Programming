#ifndef UTMPLIB
#define UTMPLIB
// Sorry if your mind me putting implementation in a header file

/*
 * utmplib.h - functions to buffer reads from utmp file
 * functions are:
 *  utmp_open(filename)    - open file  
 *      returns -1 on error
 *  utmp_reload            - read next bunch of records into buffer
 *      return the number of records that have been read
 *  utmp_next()            - return pointer to next struct
 *      returns NULL on EOF
 *  utmp_close()           - close file
*/

#include <stdio.h>
#include <fcntl.h>
#include <sys/types.h>
#include <utmp.h>
#include <unistd.h>
#include <stdlib.h>

#define NRECS 16
#define NULLUT ((struct utmp *)NULL)
#define UTSIZE (sizeof(struct utmp))

static char utmpbuf[NRECS * UTSIZE];
static int num_recs;
static int cur_rec;
static int fd_utmp = -1;

int utmp_open(char *filename)
{
	fd_utmp = open(filename, O_RDONLY);
	cur_rec = 0;
	num_recs = 0;
	return fd_utmp;
}

int utmp_reload()
{
	int amt_read = read(fd_utmp, &utmpbuf, NRECS * UTSIZE);
	if (amt_read == -1) {
		fprintf(stderr, "Error: can not read from the utmp file");
		perror("");
		exit(-1);
	}
	num_recs = amt_read / UTSIZE;
	cur_rec = 0; // reset to 0
	return num_recs;
}

struct utmp *utmp_next()
{
	struct utmp *recp = NULLUT;
	if (fd_utmp == -1) {
		return recp;
	}
	// buffer is empty, time to reload records
	// check whether we have reached EOF at the same time
	// and if we got any records, cur_rec is reset to 0(line 44)
	if (cur_rec == num_recs && utmp_reload() == 0) {
		return recp;
	}
	recp = (struct utmp *)(&utmpbuf[cur_rec * UTSIZE]);
	cur_rec += 1; // increment the pointer by one
	return recp;
}

void utmp_close()
{
	if (fd_utmp != -1) {
		close(fd_utmp);
	}
}

#endif