#include <stdio.h>

//void sum(int begin, int end);
void swap();


int main(){

	swap(5,6);

	return 0;
}

void swap(double a, double b){
	printf("in swap,a = %f , b = %f\n", a, b);
}
