#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define MAX_CHAR 126
#define MAX_ARR 26

typedef struct list {
	float value;
	char operand;
	int count_order;
	struct list *link;
}node;

//convert_str_to_data.c
node *convert_str_to_data(char*, node*);
float digits_detect(char*, int*);
float transform_to_one(float*, const int);
node *operand_detect(char*, node*);

//utility_functions.c
node *add_to_list(node*, node*);
void print_list(node*);
node *delete_node(node*, node*);
int list_len(node*);
void free_list(node*);
float potency(float, float);
int end_program(char*);

//calculate.c
node *calculate(node*);
float do_the_math(node*);

