/*
 * who2.c - read /var/run/utmp and lsit info therein
 * suppresses empty records
 * formats time neatly
 *
*/
#include <stdio.h>
#include <utmp.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdlib.h>
#include <time.h>
#include <sys/types.h>
#include <ctype.h>
#include <string.h>


/*
 * trim the string s
*/ 
char *trim(char *s) {
    char *ptr;
    if (!s)
        return NULL;   // handle NULL string
    if (!*s)
        return s;      // handle empty string
    for (ptr = s + strlen(s) - 1; (ptr >= s) && isspace(*ptr); --ptr);
    ptr[1] = '\0';
    return s;
}

/*
 * display time in a format fit for human consumption
*/ 
void show_time(int seconds){
    time_t sec = (time_t)seconds;
    time_t * p = &sec;
    char * s = trim(ctime(p));
    printf("%s", s+4);
}

/*
 * display contents of the utmp struct in human readable format
 * display nothing if the record is empty
*/
void show_info(struct utmp * ut_buf_p){
    if (ut_buf_p->ut_type != USER_PROCESS){
        return;
    }

    printf("%-8.8s", ut_buf_p->ut_user);
    printf(" ");
    printf("%-8.8s", ut_buf_p->ut_line);
    printf(" ");
    show_time(ut_buf_p->ut_tv.tv_sec);
    if (ut_buf_p->ut_host[0] != '\0'){
        printf("(%s)", ut_buf_p->ut_host);
    }
    printf("\n");
}

int main(){
    struct utmp current_record; 
    int utmpfd;
    size_t rec_len = sizeof(struct utmp);

    if (( utmpfd = open("/var/run/utmp", O_RDONLY)) == -1){
        perror(UTMP_FILE);
        exit(-1);
    }
    while ((read(utmpfd, &current_record, rec_len) == rec_len)){
        show_info(&current_record);
    }
    close(utmpfd);
    return 0;
}
