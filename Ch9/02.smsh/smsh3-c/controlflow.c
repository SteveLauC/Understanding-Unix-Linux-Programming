/*
    controlflow.c
    "if" processing is done with two state variables `if_state` and `if_result`
*/

#include <stdio.h>
#include <string.h>
#include "smsh.h"

enum states {
	NEUTRAL,
	WANT_THEN,
	THEN_BLOCK,
};

enum results {
	SUCCESS,
	FAIL,
};

// global static status variables
static int if_state = NEUTRAL;
static int if_result = SUCCESS;
static int last_stat = 0;

/*
    purpose: handles syntan error in control structures
    return: always return -1
    action: reset if_state to NENTRAL, print `msg` to stderr and return
*/
int syn_err(char *msg)
{
	if_state = NEUTRAL;
	fprintf(stderr, "syntax error: %s\n", msg);
	return -1;
}

/*
    purpose: determine the whether the shell should execute the current normal command
    return: true for yes, false for no
*/
bool ok_to_execute()
{
	bool rv;
	switch (if_state) {
	case NEUTRAL:
		rv = true;
		break;
	case WANT_THEN:
		rv = false;
		break;
	case THEN_BLOCK:
		// printf("last_status: %d\n", last_stat);
		if (if_result == SUCCESS) {
			rv = true;
		} else {
			rv = false;
		}
		break;
	default:
		rv = false;
	}

	return rv;
}

/*
    purpose: determine whether the command is a control command
    return: true on control command
*/
bool is_control_command(char *cmd)
{
	return (strncmp(cmd, "if", 2) == 0 || strncmp(cmd, "then", 4) == 0 ||
		strncmp(cmd, "fi", 2) == 0);
}

/*
    purpose: process `if`, `then` and `fi` commands
    return: 0 for ok, -1 for syntax error
*/
int do_control_command(char **args)
{
	// extract the cmd
	char *cmd = args[0];
	int rv = -1;

	// handle the `if` command
	if (strncmp(cmd, "if", 2) == 0) {
		if (if_state != NEUTRAL) {
			rv = syn_err("if unexpected");
		} else {
			last_stat = process(
				args + 1); // process the subsequent command
			if_result = last_stat == 0 ? SUCCESS : FAIL;
			if_state = WANT_THEN;
			rv = 0;
		}
	} else if (strncmp(cmd, "then", 4) == 0) {
		if (if_state != WANT_THEN) {
			rv = syn_err("then unexpected");
		} else {
			if_state = THEN_BLOCK;
			rv = 0;
		}
	} else if (strncmp(cmd, "fi", 2) == 0) {
		if (if_state != THEN_BLOCK) {
			rv = syn_err("fi unexpected");
		} else {
			if_state = NEUTRAL;
			rv = 0;
		}
	} else {
		fatal("internal error message", cmd, 2);
	}

	return rv;
}
