#include <stdint.h>
void get_internet_address(char *, int, uint16_t *, struct sockaddr_in *);
int make_dgram_server_socket(uint16_t);
int make_dgram_client_socket();
void make_internet_address(char *, uint16_t, struct sockaddr_in *);
