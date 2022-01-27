/* 
 * ls1.c
 * purpose: list contents of directory or directories
 * action:  if no args, use `.`, else list files in args
*/ 

#include <stdio.h>
#include <stdlib.h>
#include <dirent.h>

void do_ls(char []);

int main(int ac, char *av[]){
    if (ac == 1) {
        do_ls(".");
    }else{
        while (--ac) {
            printf("%s: \n", * ++av); 
            do_ls(*av);
        }
    }
}

// list files in directory called filename
void do_ls(char dirname[]) {
    DIR * dir_ptr = NULL;
    struct dirent * dirent_ptr = NULL;

    if ((dir_ptr = opendir(dirname)) != NULL) {
        while ((dirent_ptr=readdir(dir_ptr)) != NULL) {
            printf("%s\n", dirent_ptr->d_name);
        }
        closedir(dir_ptr);
    }
}