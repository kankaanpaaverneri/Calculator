#include "head.h"

int main(void) {
	while(1) {
		char *string = (char*)malloc(MAX_CHAR * sizeof(*string));
		node *root = NULL;
		printf("CALCULATOR: ");
		fgets(string, MAX_CHAR-1, stdin);
		if(end_program(&string[0]) == 1) {
			free(string);
			break;
		}
		root = convert_str_to_data(string, root);
		if(root != NULL) {
			print_list(root);
			root = calculate(root);
			printf("%.0f\n", root->value);
			free_list(root);
		}
		free(string);
	}
	return 0;
}
