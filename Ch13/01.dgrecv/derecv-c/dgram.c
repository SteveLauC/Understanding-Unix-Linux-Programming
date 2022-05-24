/*
 * dgram.c: support functions for datagram based programs
*/

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <netdb.h>
#include <stdint.h>
#include <string.h>
#include <stdint.h>

#define HOSTNAME_LEN 64

void make_internet_address(char * hostname, uint16_t port, struct sockaddr_in  * address  ) {
    address->sin_family = AF_INET;
    address->sin_port = htons(port); 
    struct hostent * hp = gethostbyname(hostname);
    if (hp==NULL||hp->h_addrtype!=AF_INET) {
        perror("gethostbyname");
        exit(1);
    }
    memcpy(&address->sin_addr, hp->h_addr_list[0], hp->h_length);
}

int make_dgram_server_socket(uint16_t port_num) {
    struct sockaddr_in server_address;
    memset(&server_address, 0, sizeof(server_address));

    char hostname[HOSTNAME_LEN];
    memset(hostname, 0, HOSTNAME_LEN);
    gethostname(hostname, HOSTNAME_LEN);

    int server_fd = socket(AF_INET, SOCK_DGRAM, 0);
    if (server_fd==-1) {
        return -1;
    }
    
    make_internet_address(hostname, port_num, &server_address);

    if(bind(server_fd, (struct sockaddr *)&server_address, sizeof(server_address)) == -1) {
        return -1;
    }

    return server_fd;
}

int make_dgram_client_socket() {
    return socket(AF_INET, SOCK_DGRAM, 0);
}

void get_internet_address(char *host, int len, uint16_t *port_p, struct sockaddr_in * addrp) {
    strncpy(host, inet_ntoa(addrp->sin_addr), len);
    *port_p = ntohs(addrp->sin_port);
}
