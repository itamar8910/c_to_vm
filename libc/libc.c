void putc(char c){
    int* COS = 200;
    int* COD = 201;
    *COD = c;
    *COS = 1;
}

void puts(char* str){
    while(*str != 0){
        putc(*(str++));
    }
}