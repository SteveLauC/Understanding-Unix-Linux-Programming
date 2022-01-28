/* 
 * ls2.c
 * purpose: list contents of directory or directories
 * action: if no arg, use `.`. else, list files in args
 * note: uses stat and pwd.h and grp.h
 * BUG: try ls2 /tmp [no such file or directory]
 * reason: the arg of do_stat() is only the name of file
 *         but not the path, so stat() will seek that file
 *         in the current working directory. if the arg given to
 *         this program is not ".", it will absolutely print
 *         no such file or directory.
*/

#include <stdio.h>
#include <sys/stat.h>
#include <dirent.h>
#include <string.h>
#include <pwd.h>
#include <grp.h>
#include <time.h>
#include <ctype.h>

void do_ls(char []);

void do_stat(char *);
void mode_to_letters(mode_t, char []);
char * uid_to_name(uid_t);
char * gid_to_name(gid_t);
char * trim(char *);
char * show_time_nifty(time_t);
void show_file_info(char *, struct stat *);

int main(int ac, char *av[]){
    if (ac == 1) {
        do_ls(".");
    }else{
        while (--ac) {
            printf("%s: \n", *++av);
            do_ls(*av);
        }
    }
    return 0;
}


// list files in directory called dirname
void do_ls(char dirname[]) {
    DIR * dir_ptr = NULL;
    struct dirent * dirent_ptr = NULL;

    if ((dir_ptr = opendir(dirname)) == NULL) {
        fprintf(stderr, "ls1: cannot open %s\n", dirname);
    }else{
        while((dirent_ptr = readdir(dir_ptr)) != NULL) {
            do_stat(dirent_ptr->d_name);
        }
        closedir(dir_ptr);
    }
}

void do_stat(char * filename) {
    struct stat buf;
    if (-1 == stat(filename, &buf) ) {
        perror(filename);
    }else{
        show_file_info(filename, &buf);
    }
}


/*
 * This function takes a mode value and a char array
 * and puts into the char array the file type and the 
 * nine lettters that correspond to the bits in mode
 * NOTE: it doesn't code steuid/setgid/sticky-bit
*/
void mode_to_letters(mode_t mode, char mode_str[] ) {
    strcpy(mode_str, "----------");   // default = no perms
    // file type
    if (S_ISDIR(mode)) mode_str[0] = 'd';
    if (S_ISCHR(mode)) mode_str[0] = 'c';
    if (S_ISBLK(mode)) mode_str[0] = 'b';
    if (S_ISLNK(mode)) mode_str[0] = 'l';
    if (S_ISSOCK(mode)) mode_str[0] = 's';
    if (S_ISFIFO(mode)) mode_str[0] = 'p';

    // permission
    if (mode & S_IRUSR) mode_str[1] = 'r';
    if (mode & S_IWUSR) mode_str[2] =  'w';
    if (mode & S_IXUSR) mode_str[3] = 'x';
    if (mode & S_IRGRP) mode_str[4] = 'r';
    if (mode & S_IWGRP) mode_str[5] = 'w';
    if (mode & S_IXGRP) mode_str[6] = 'x';
    if (mode & S_IROTH) mode_str[7] = 'r';
    if (mode & S_IWGRP) mode_str[8] = 'w';
    if (mode & S_IXOTH) mode_str[9] = 'x';
}


char * uid_to_name(uid_t uid) {
    struct passwd * pw_ptr = NULL;
    // make it static to prevent it from becoming 
    // a dangling ptr after stack frame is reclaimed 
    static char uid_str[10];      
    if ((pw_ptr = getpwuid(uid)) == NULL) {
        sprintf(uid_str, "%d", uid);
        return uid_str;
    }else{
        return pw_ptr->pw_name;
    }   
}

char * gid_to_name(gid_t gid) {
    struct group * gp_ptr = NULL;
    static char gid_str[10];
    if ((gp_ptr = getgrgid(gid)) == NULL) {
        sprintf(gid_str, "%d", gid);
        return gid_str;
    }else{
        return gp_ptr->gr_name;
    }
}

char *trim(char *s) {
    char *ptr;
    if (!s)
        return NULL;   // handle NULL string
    if (!*s|| strlen(s)==0)
        return s;      // handle empty string
    for (ptr = s + strlen(s) - 1; (ptr >= s) && isspace(*ptr); --ptr);
    ptr[1] = '\0';
    return s;
}

/*
 * display time in a format fit for human consumption
*/ 
char * show_time_nifty(time_t seconds){
    time_t * p = &seconds;
    char * s = trim(ctime(p));
    return s+4;
}

void show_file_info(char * filename, struct stat * st_buf) {
    // mode
    char mode[10];
    mode_to_letters(st_buf->st_mode, mode);

    // link numeral
    nlink_t link_num = st_buf->st_nlink;

    // owner
    char * owner = uid_to_name(st_buf->st_uid);

    // group
    char * group = gid_to_name(st_buf->st_gid);

    // size
    off_t size = st_buf->st_size;

    // mtime
    char * mtime = show_time_nifty(st_buf->st_mtime);

    // list a record
    printf("%10s %4ld %8s %8s %8ld %12s %s\n", mode, link_num, owner, group, size, mtime, filename);
}