/*
 * rls.c: a client for a remote directory listing srevice
 *
 * usage: rls hostname directory
*/

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <assert.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <netdb.h>

#define PORT_NUM 15000
#define oops(msg) {perror(msg); exit(1);}

int main(int ac, char ** av) {
    assert(ac == 3);
    // get a socket
    int client_fd = socket(AF_INET, SOCK_STREAM, 0);
    if (client_fd == 0) {
        oops("socket");
    }


    // connect to the server
    struct sockaddr_in server_address;
    memset(&server_address, 0, sizeof(server_address));
    struct hostent * hp = gethostbyname(av[1]);
    if (hp == NULL || hp->h_addrtype != AF_INET) {
        oops("gethostbyname failed or can not find a v4 address for this hostname");
    }
    memcpy(&server_address.sin_addr.s_addr, hp->h_addr_list[0], hp->h_length);
    server_address.sin_port = htons(PORT_NUM);
    server_address.sin_family = AF_INET;
    if (connect(client_fd, (struct sockaddr *)&server_address, sizeof(server_address))) {
        oops("connect");
    }

    // send dirctory name, then read back results
    int n_read = 0;
    char buffer[BUFSIZ];
    memset(buffer, 0, BUFSIZ);
    if(write(client_fd, av[2],  strlen(av[2])) == -1) {
        oops("write");
    }
    if(write(client_fd, "\n", 1) == -1) {
        oops("write");
    }

    while((n_read = read(client_fd, buffer, BUFSIZ)>0)) {
        if(write(1, buffer, n_read) == -1) {
            oops("write");
        }
    }
    close(client_fd);
}
