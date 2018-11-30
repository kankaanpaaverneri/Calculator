#include "head.h"

node *convert_str_to_data(char *str, node *root) {
	unsigned int i = 0;
	int pass = 0;
	while(str[i] != '\0') {
		pass = 0;
		node *new = NULL;
		if((new = (node*)malloc(sizeof(*new))) == NULL) {
			printf("OUT OF HEAP MEMORY\n\n");
			exit(1);
		} else {
			if((new->value = digits_detect(&str[i], &i)) != 0) {
				pass++;
			}
			if((new = operand_detect(&str[i], new)) != NULL) {
				pass++;
				new->operand = str[i];
			}
			if(pass >= 2) {
				root = add_to_list(new, root);
			} else {
				printf("INVALID STRING\n\n");
				free(new);
				free_list(root);
				return NULL;
			}
			if(new->count_order == 0)
				break;
			i++;
		}
	}
	return root;
}

float digits_detect(char *ch, int *jumps) {
	float *digit_arr = NULL;
	int i = 0;
	float r = 0;

	if((digit_arr = (float*)malloc(MAX_ARR*sizeof(*digit_arr))) == NULL) {
		printf("OUT OF HEAP MEMORY\n\n");
		exit(1);
	} else {
		while(ch != NULL) {
			if(*ch >= '0' && *ch <= '9') {
				digit_arr[i] = (float)*ch-48;
				i++;
				(*jumps)++;
				ch++;
			} else {
				break;
			}
		}
		r = transform_to_one(digit_arr, i);
		free(digit_arr);
	}
	return r;
}

float transform_to_one(float *arr, const int n) {
	char *str = NULL;
	int i = 0;
	float final_val = 0;
	
	if((str = (char*)malloc(MAX_ARR*sizeof(*str))) == NULL) {
		printf("OUT OF MEMORY\n\n");
		exit(1);
	} else {
		for(i = 0; i < n; i++) {
			str[i] = (char)arr[i]+48;
		}
		str[i] = '\0';
		final_val = (float)strtod(str, NULL);
		free(str);
	}
	return final_val;
}

node *operand_detect(char *ch, node *new) {
	if(*ch == '+' || *ch == '-') {
		new->count_order = 3;
	} else if(*ch == '*' || *ch == '/') {
		new->count_order = 2;
	} else if(*ch == '^') {
		new->count_order = 1;
	} else if(*ch == '=') {
		new->count_order = 0;
	} else {
		new->count_order = -1;
		return NULL;
	}
	return new;
}

