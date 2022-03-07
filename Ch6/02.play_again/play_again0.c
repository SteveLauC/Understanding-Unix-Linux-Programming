/*
  play_again0.c
  purpose: ask if user want another transaction
  method: ask a question, wait for yes/no answer
  returns: 0=>yes, 1=>no
  better: eliminate need to press return
*/

#include <stdio.h>

#define QUESTION "Do you want another transaction"

/*
  purpose: ask a question and wait for a y/n answer
  method: use getchar and ignore non y/n answers
  returns: 0=>yes, 1=>no
*/
int get_response(char * question) {
    printf("%s (y/n)", QUESTION);
    while(1) {
        switch (getchar() ) {
            case 'y':
            case 'Y':
                return 0;
            case 'n':
            case 'N':
            case EOF:
                return 1;
        }
    }
}

int main() {
    int response = get_response(QUESTION);
    return response;
}
