/*
 * dgrecv.c: datagram receiver
 *
 * usage: dgrecv port_num
 *
 * action: listen at the specified port and reports messages
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <unistd.h>
#include "dgram.h"

#define oops(msg){perror(msg); exit(1);}

void say_who_called(struct sockaddr_in *);


int main(int ac, char * av[]) {
    if (ac != 2) {
        fprintf(stderr, "usage: ./derecv port_num");
        exit(1);
    }

    uint16_t port = (uint16_t)atoi(av[1]);
    if (port < 0) {
        fprintf(stderr, "Invalid port number");
        exit(1);
    }

    int server_fd = make_dgram_server_socket(port);
    if (server_fd == -1) {
        oops("Can not make socket");
    }

    char buf[BUFSIZ];
    memset(buf, 0, BUFSIZ);
    int msglen = 0;
    struct sockaddr_in client_address;
    memset(&client_address, 0, sizeof(client_address));
    socklen_t address_len = 0;
    while ((msglen = recvfrom(server_fd, buf, BUFSIZ, 0, (struct sockaddr *)&client_address, &address_len)) > 0) {
        printf("dgrecv: got a message: %s\n", buf);
        say_who_called(&client_address);
    }
}

void say_who_called(struct sockaddr_in * addr_p) {
    char host[64];
    memset(host, 0, 64);
    uint16_t port = -1;

    get_internet_address(host, 64, &port, addr_p);
    printf("from: %s:%d\n", host, port);
}
