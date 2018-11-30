#include "head.h"

node *add_to_list(node *new, node *root) {
	new->link = NULL;
	if(root == NULL) {
		root = new;
	} else {
		node *cur = root;
		while(cur->link != NULL) {
			cur = cur->link;
		}
		cur->link = new;
	}
	return root;
}

void print_list(node *root) {
	node *cur = root;
	while(cur != NULL) {
		printf("%.0f %c ", cur->value, cur->operand);
		cur = cur->link;
	}
	return;
}

node *delete_node(node *cur, node *root) {
	if(root == NULL) {
		return NULL;
	} else if(cur == root) {
		node *temp = root;
		root = temp->link;
		temp->link = NULL;
		free(temp);
	} else {
		node *p = root, *q = NULL;
		while(p->link != cur) {
			p = p->link;
		}
		q = p->link;
		p->link = q->link;
		q->link = NULL;
		free(q);
	}
	return root;
}

int list_len(node *root) {
	node *cur = root;
	int count = 0;
	while(cur != NULL) {
		count++;
		cur = cur->link;
	}
	return count;
}

void free_list(node *root) {
	node *cur = root, *temp = NULL;
	while(cur != NULL) {
		temp = cur;
		cur = cur->link;
		free(temp);
	}
	return;
}

float potency(float base, float expo) {
	int i = 0;
	float result = base;
	for(i = 0; i < expo-1; i++) {
		result = result*base;
	}
	return result;
}

int end_program(char *input) {
	if(*input == '0') {
		return 1;
	} else {
		return 0;
	}
}

