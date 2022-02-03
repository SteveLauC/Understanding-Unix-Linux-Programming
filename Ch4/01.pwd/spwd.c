/*
 * spwd.c: a simplified version of pwd
 * 
 * starts in current directory and recursively
 * climbs up to root of filesystem, prints top part
 * then prints currnet part
 * 
 * uses readdir() to get info about each thing
*/

#include <stdio.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <dirent.h>
#include <unistd.h>
#include <stdlib.h>
#include <string.h>

ino_t get_inode(char *);
void print_path_to(ino_t);
void inode_to_name(ino_t, char *, int);

int main(){
    print_path_to(get_inode("."));
    putchar('\n');
    return 0;
}

/*
 * returns inode number of the file
*/
ino_t get_inode(char * filename) {
    struct stat buf;
    if (stat(filename, &buf) == -1) {
        fprintf(stderr, "cannot stat ");
        perror(NULL);
        exit(-1);
    }
    return buf.st_ino;
}

/*
 * looks through current directory for a file with this inode
 * number and copied its name into namebuf
*/
void inode_to_name(ino_t this_inode, char * namebuf, int buflen) {
    DIR * dir_ptr = NULL;
    struct dirent * dirent_ptr = NULL;

    if ((dir_ptr = opendir(".")) == NULL) {
       perror(NULL);
       exit(-1);
    }

    while ((dirent_ptr=readdir(dir_ptr)) != NULL) {
        struct stat buf;
        stat(dirent_ptr->d_name, &buf);
        if (buf.st_ino == this_inode) {
            strncpy(namebuf, dirent_ptr->d_name, buflen);
            closedir(dir_ptr);
            namebuf[buflen-1] = '\0';
            return;
        }
    }
    fprintf(stderr, "error looking for inode: %ld\n", this_inode);
    exit(1);
}

/*
 * prints each leading down to an object with this inode
 * kindof recursive
*/
void print_path_to(ino_t inode) {
    ino_t my_inode = 0;
    char its_name[256];

    if (get_inode("..") != inode) {
        chdir("..");
        inode_to_name(inode, its_name, 256);
        my_inode = get_inode(".");
        print_path_to(my_inode);
        printf("/%s", its_name);
    }
}