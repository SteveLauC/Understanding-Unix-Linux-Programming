#include <stdio.h>
#include <stdlib.h>
#include <termio.h>
#include <stdlib.h>

struct flaginfo {
    unsigned int fl_value;  // mask
    char * fl_name; // description
};

struct flaginfo input_flags[] = {
    {IGNBRK, "Ignore break condition"},
    {BRKINT, "Signal interrupt on break"},
    {IGNPAR, "Ignore chars with parity errors"},
    {PARMRK, "Mark parity errors"},
    {INPCK, "Enable input parity check"},
    {ISTRIP, "Strip character"},
    {INLCR, "Map NL to CR on input"},
    {IGNCR, "Ignore CR"},
    {ICRNL, "Map CR to NL on input"},
    {IXON, "Enable start/stop output control"},
    {IXOFF, "Enable start/stop inout control"},
    {0, NULL}, // end marker
};

struct flaginfo local_flags[] = {
    {ISIG, "Enable signals"},
    {ICANON, "Canonical input(erase and kill)"},
    {ECHO, "Enable echo"},
    {ECHOE, "ECHO ERASE as BS-SPACE-BS"},
    {0, NULL}, // end marker
};

void showbaud(int thespeed) {
    switch (thespeed) {
        case B300:
            printf("300\n");
            break;
        case B600:
            printf("600\n");
            break;
        case B1200:
            printf("1200\n");
            break;
        case B1800:
            printf("1800\n");
            break;
        case B2400:
            printf("2400\n");
            break;
        case B4800:
            printf("4800\n");
            break;
        case B9600:
            printf("9600\n");
            break;
        case B38400:
            printf("38400\n");
            break;
        default:
            printf("fast\n");
            break;
    }
}

/*
  check echo bit pattern and display descriptive title
*/
void show_flagset(int thevalue, struct flaginfo thebitnames[]) {
    // use `fl_value == NULL` as an end indicator
    for (int i = 0; thebitnames[i].fl_value; i++) {
        printf("%s is ", thebitnames[i].fl_name);
        if (thevalue & thebitnames[i].fl_value) {
            printf("ON\n");
        }else{
            printf("OFF\n");
        }
    }

}

/*
  show the values of two flags sets: c_iflag and c_lflag
*/
void show_some_flags(struct termios * ttyp) {
    show_flagset(ttyp->c_iflag, input_flags);
    show_flagset(ttyp->c_lflag, local_flags);
}

int main() {
    struct termios ttyinfo;
    if (-1 == tcgetattr(0, &ttyinfo)) {
        perror("cannot get params about Stdin\n");
        exit(1);
    }

    printf("OUTPUT BAUD RATE: \n");
    showbaud(cfgetospeed(&ttyinfo));
    printf("\n");

    printf("CONTROL CHARACTER: \n");
    printf("The erase character is ascii %d, Ctrl - %c\n", ttyinfo.c_cc[VERASE], ttyinfo.c_cc[VERASE]+'A'-1);
    printf("The line kill character is ascii %d, Ctrl - %c\n", ttyinfo.c_cc[VKILL], ttyinfo.c_cc[VKILL]+'A'-1);
    printf("\n");

    printf("INPUT FLAGSET AND LOCAL FLAGSET: \n");
    show_some_flags(&ttyinfo);
    printf("\n");
}
