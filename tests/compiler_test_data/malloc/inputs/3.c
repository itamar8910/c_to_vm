#include <libc.h>

int main(){
    int* p;
    p = malloc(sizeof(int) * 1500);
    puts(itos(p));
    putc('\n');
    p = malloc(sizeof(int) * 1000);
    puts(itos(p));
    putc('\n');
    return p == 0;
}