/*
    process.c
    command processing layer, frontend for `do_control_command` and `execute`
*/

#include "smsh.h"

/*
    purpose: process user command

    action: call `execute` on shell command and call `do_control_command` on control command

    return: the result of processing command
*/
int process(char ** args) {
    int rv = -1;

    if (args == NULL) {
        rv = 0;
    } else if (is_control_command(args[0])) {
        rv = do_control_command(args);
    } else if (is_built_in_command(args[0])) {
        rv = builtin_command(args);
    } else if (ok_to_execute()) {
        rv = execute(args);
    }

    return rv;
}