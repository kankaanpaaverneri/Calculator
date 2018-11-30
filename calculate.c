#include "head.h"

node *calculate(node *root) {
	float result = 0.0;
	node *cur = root;
	unsigned int i = 1;
	while(i <= 3) {
		if(list_len(cur) == 0)
			break;
		while(cur != NULL) {
			if(cur->count_order == i) {
				node *temp = cur;
				cur->link->value = do_the_math(cur);
				cur = cur->link;
				root = delete_node(temp, root);	
			} else {
				cur = cur->link;
			}
		}
		i++;
		cur = root;
	}
	return root;
}

float do_the_math(node *cur) {
	float result = cur->value;
	switch(cur->operand) {
		case '+':
			result += cur->link->value;
			break;
		case '-':
			result -= cur->link->value;
			break;
		case '*':
			result *= cur->link->value;
			break;
		case '/':
			result /= cur->link->value;
			break;
		case '^':
			result = potency(result, cur->link->value);
			break;
		default:
			result = cur->value;
			break;
	}
	return result;
}
