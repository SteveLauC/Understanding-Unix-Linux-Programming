/*
 * logfiled.c: a simple logfile server using UNIX domain datagram sockets
 *
 * usage: ./logfiled >> logfilename
 *
 * action: logfiled will constantly read log info from any program 
 * communicating with it and write these infomation to stdout, due
 * to the redirection(stdout -> logfilename), it will actually write
 * to the file specified by `logfilename`
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stddef.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <time.h>

#define MSG_LEN 512
#define oops(msg){perror(msg); exit(1);}
#define SOCKNAME "/tmp/logfilesock"

int main(void) {
    struct sockaddr_un server_addr;
    server_addr.sun_family = AF_UNIX;
    strncpy(server_addr.sun_path, SOCKNAME, strlen(SOCKNAME)+1);

    int server_sock = socket(AF_UNIX, SOCK_DGRAM, 0);
    if (server_sock == 0) {
        oops("socket");
    }

    // bind the address

    if (bind(server_sock, (struct sockaddr *)&server_addr, offsetof(struct sockaddr_un, sun_path)+strlen(server_addr.sun_path)+1) == -1) {
        oops("bind");
    }

    char msg[MSG_LEN];
    time_t now;
    char * time_str = NULL;
    int msg_num = 0;
    while (1) {
        int idx = read(server_sock, msg, MSG_LEN);
        msg[idx] = '\0';

        time(&now);
        time_str = ctime(&now);
        time_str[strlen(time_str)-1] = '\0';

        printf("[%5d] %s %s\n", msg_num++, time_str, msg);
        fflush(stdout);
    }
}
