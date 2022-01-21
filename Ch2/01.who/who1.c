/*
 * who1.c - a first version of the who program
 * open,read UTMP file, and show results.
 *
*/
#include <stdio.h>
#include <utmp.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdlib.h>
#include <time.h>
#include <sys/types.h>


/*
 * display contents of the utmp struct in human readable format
*/
void show_info(struct utmp * ut_buf_p){
    printf("%-8.8s", ut_buf_p->ut_user);
    printf(" ");
    printf("%-8.8s", ut_buf_p->ut_line);
    printf(" ");
    long sec = (long)ut_buf_p->ut_tv.tv_sec;
    printf("%s", ctime(&sec));
    printf("%s", ut_buf_p->ut_host);
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