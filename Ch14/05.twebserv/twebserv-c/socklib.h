#include <stdint.h>

int make_server_socket(uint16_t port_num, int backlog);
int connect_to_server(char *hostname, uint16_t port_num);
