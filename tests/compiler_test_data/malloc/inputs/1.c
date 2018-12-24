#include <libc.h>

int main(){
    int* p1 = malloc(1);
    int* p2 = malloc(1);
    return p2 > p1;
}