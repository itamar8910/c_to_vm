#include <libc.h>

int main(){
    puts(itos(5));
    putc('\n');
    puts(itos(0));
    putc('\n');
    puts(itos(1123));
    putc('\n');
    puts(itos(-1124));
    return 0;
}