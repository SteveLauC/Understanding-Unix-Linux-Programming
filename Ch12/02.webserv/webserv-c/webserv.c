/*
 * webserv.c: a minimal web server
 *
 * build: gcc webserv.c socklib.c -o webserv
 *
 * usage: ./webserv PORT_NUM
 *
 * features:
 *      1. supports the GET command only
 *      2. runs in the current directory
 *      3. has major sercurity holes, for demo purpose only
 *      4. has many other weaknesses, but is a good start
 *
 * test:
 *       * cgi file: http://hostname:port/exe.cgi
 *       * dir: http://hostname:port/test_dir
 *       * cat: http://hostname:port/index.html
 *       * 404: http://hostname:port/??
*/

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <string.h>
#include <unistd.h>
#include "socklib.h"

void read_til_crlf(FILE * fp);
void process_rq(const char * request, int fd);
void cannot_do(int fd);
void do_404(const char * item, int fd);
int is_dir(const char *f);
int not_exist(const char * f);
void do_ls(const char * dir, int fd);
char * filetype(const char * f);
int ends_in_cgi(const char * f);
void do_exec(const char * prog, int fd);
void do_cat(const char *f, int fd);
void header(FILE * fp, const char * content_type);

int main(int ac, char *av[]) {
    if (ac != 2) {
        fprintf(stderr, "usage: ./webserv PORT_NUM");
        exit(1);
    }

    int server_fd = make_server_socket((uint16_t)atoi(av[1]), 4); 
    if (server_fd == -1) {exit(2);}


    while (1) {
        int client_fd = accept(server_fd, NULL, NULL);
        if (client_fd == -1) {
            exit(3);
        }

        FILE * client_reader = fdopen(client_fd, "r");
        if (client_reader == NULL) {
            perror("fdopen");
            exit(4);
        }
        // read request
        char request[BUFSIZ];
        memset(request, 0, BUFSIZ);
        fgets(request, BUFSIZ, client_reader);
        printf("got a call: request = %s\n", request);

        // do waht client asks
        process_rq(request, client_fd);


        close(client_fd);
    }

    waitpid(-1, NULL, WNOHANG);
}


/*
 * purpose: do what the request asks for and write reply to fd
 *
 * action: handles request in a new process
 *
 * arguments:
 *      * `request`: first line of a http head, like `GET /foo/bar.html HTTP/1.1`
 *      * `fd`: client file desctripor
*/
void process_rq(const char * request, int fd) {

    char cmd[BUFSIZ];
    char arg[BUFSIZ];
    memset(cmd, 0, BUFSIZ);
    memset(arg, 0, BUFSIZ);
    // create a new process and return if not the child
    if (fork() != 0) {
        return;
    }


    // request parsing
    if (sscanf(request, "%s %s", cmd, arg) != 2) {
        return;
    }


    // remove the first `/` in arg
    for(int i = 0; arg[i]!='\0'; i+=1) {
        arg[i] = arg[i+1];
    }

    if (strcmp(cmd, "GET") != 0) {
        cannot_do(fd);
    }else if (not_exist(arg)) {
        do_404(arg, fd);
    }else if (is_dir(arg)) {
        do_ls(arg, fd);
    }else if (ends_in_cgi(arg)) {
        do_exec(arg, fd);
    }else {
        do_cat(arg, fd);
    }
}

/*
 * purpose: write a HTTP response header to the client
 *
 * action: write response headers to the client
 *
 * arguments:
 *      * `fp`: client writer 
 *      * `content_type`: string to represent the content-type
*/
void header(FILE * fp, const char * content_type) {
    fprintf(fp, "HTTP/1.1 200 OK\r\n");

    if (content_type != NULL) {
        fprintf(fp, "Content-type: %s\r\n", content_type);
    }
}

/*
 * purpose: handle the cases where we have not implementated yet
 *
 * arguments:
 *      * `fd`: client file desctriptor
*/
void cannot_do(int fd) {
    FILE * client_writer = fdopen(fd, "w");
    if (client_writer == NULL) {
        perror("fdopen");
        return;
    }

    // header
    fprintf(client_writer, "HTTP/1.1 501 Not Implementated\r\n");
    fprintf(client_writer, "Content-tyep: text/plain\r\n");
    fprintf(client_writer, "\r\n");
                                    
    // body
    fprintf(client_writer, "The command is not yet implementated\r\n");
    
    fclose(client_writer);
}

/*
 * purpose: handle the cases where the requested file does not exist
 *
 * arguments: 
 *      * `item`: file name/path
 *      * `fd`: client file desctriptor
*/
void do_404(const char * item, int fd) {
    FILE * client_writer = fdopen(fd, "w");
    if (client_writer == NULL) {
        perror("fdopen");
        return;
    }

    // header
    fprintf(client_writer, "HTTP/1.1 404 Not Found\r\n");
    fprintf(client_writer, "Content-type: text/plain]\r\n");
    fprintf(client_writer, "\r\n");

    // body
    fprintf(client_writer, "The item you requested: %s is not found\r\n", item);

    fclose(client_writer);
}

/*
 * purpose: to determine whether `f` is a directory
 *
 * arguments:
 *      * `f`: file path 
 *
 * return: a boolean value(int)
*/
int is_dir(const char * f) {
    struct stat stat_buf;
    stat(f, &stat_buf);

    return S_ISDIR(stat_buf.st_mode);
}


/*
 * purpose: to check if `f` exists
 *
 * arguments:
 *      * `f`: file path
 * return: a boolean value(int)
 *
 * note: this is not that right, there are more than one cases where `stat` will 
 * return -1
*/
int not_exist(const char * f) {
    struct stat stat_buf;
    return (stat(f, &stat_buf) == -1);
}

/*
 * purpose: execute `ls -l dir`
 *
 * arguments: 
 *      * `dir`: target directory
 *      * `fd`: client file descriptor
*/
void do_ls(const char *dir, int fd) {
    FILE * client_writer = fdopen(fd, "w");
    if (client_writer == NULL) {
        perror("fdopen");
        return;
    }
    header(client_writer, "text/plain");
    fprintf(client_writer, "\r\n");
    fflush(client_writer);

    dup2(fd, 1); // redirect stdout to the client file descriptor
    dup2(fd, 2); // redirect stderr to the client file descriptor
    close(fd);
    execlp("ls", "ls", "-l", dir, NULL);

    // error handling
    perror(dir);
    exit(1);
}

/*
 * purpose: get file type of f
 *
 * arguments:
 *      * `f`: file name 
 *
 * return: substring representing the file type
*/
char * file_type(const char * f) {
    char * idx = strchr(f, '.');
    if (idx == NULL) {
        return ""; 
    }else {
        return idx + 1;
    }
}

/*
 * purpose: to check if f is of type `cgi`
 *
 * arguments:
 *      * `f`: file name 
 *
 * return: a boolean value(int)
*/
int ends_in_cgi(const char * f) {
    return strcmp(file_type(f), "cgi") == 0;
}

/*
 * purpose: to execute .cgi file
 *
 * arguments:
 *      * `prog`: something like `exe.cgi`
 *      * `fd`: client file descriptor
*/
void do_exec(const char * prog, int fd) {
    FILE * client_writer = fdopen(fd, "w");
    if (client_writer == NULL) {
        perror("fdopen");
        return;
    }

    // header
    header(client_writer, NULL);
    fprintf(client_writer, "\r\n");
    fflush(client_writer);

    int file_name_len = strlen(prog);
    char command[10+file_name_len];
    memset(command, 0, file_name_len+10);
    sprintf(command, "bash %s", prog);

    FILE * exe_reader = popen(command, "r");

    char buf[BUFSIZ];
    memset(buf, 0, BUFSIZ);

    // body
    while (fgets(buf, BUFSIZ, exe_reader) != NULL) {
        fprintf(client_writer, "%s", buf);
        memset(buf, 0, BUFSIZ);
    } 

    fclose(client_writer);
    fclose(exe_reader);

}

void do_cat(const char * f, int fd) {
    char * extension = file_type(f);
    char * content_type = "text/plain";
    FILE * client_writer = fdopen(fd, "w");
    FILE * file_reader = fopen(f, "r");

    if (strcmp(extension, "html") == 0) {
        content_type = "text/html";
    } else if (strcmp(extension, "gif") == 0) {
        content_type = "image/gif";
    } else if (strcmp(extension, "jpg") == 0) {
        content_type = "image/jpg";
    } else if (strcmp(extension, "jpeg") == 0) {
        content_type = "image/jpeg";
    }

    if (client_writer!=NULL && file_reader!=NULL) {
        header(client_writer, content_type);
        fprintf(client_writer, "\r\n");
        
        int c = EOF;
        while((c=fgetc(file_reader)) != EOF) {
            putc(c, client_writer);
        }
        fclose(client_writer);
        fclose(file_reader);
    }
    exit(0);
}
