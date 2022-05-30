/*
 * logfielc.c: logfile client 
 *
 * send messages to the logfile server
 *
 * usage: ./logfilec "a message here"
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stddef.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <sys/un.h>

#define SOCK "/tmp/logfilesock"
#define oops(msg){perror(msg); exit(1);}


int main(int ac, char * av[]) {
    // arguments check
    if (ac != 2) {
        fprintf(stderr, "usage: ./logfilec \"a message here\"\n");
        exit(1);
    } 


    int sock = socket(AF_UNIX, SOCK_DGRAM, 0); 
    if (sock == -1) {
        oops("socket");
    }

    struct sockaddr_un server_addr;
    strncpy(server_addr.sun_path, SOCK, strlen(SOCK)+1);
    server_addr.sun_family = AF_UNIX;

    socklen_t server_addr_len = offsetof(struct sockaddr_un, sun_path)+strlen(server_addr.sun_path)+1;
    
    if (sendto(sock, av[1], strlen(av[1]), 0, (struct sockaddr *)&server_addr, server_addr_len)==-1) {
        oops("sendto");
    }
}
