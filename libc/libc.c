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

int next_heap_alloc_rel; // guaranteed to be initialized to 0

void* malloc(int size) {
    int HEAP_START = 4000;
    int HEAP_END = 6000;
    if (size < 0 || HEAP_START + next_heap_alloc_rel + size > HEAP_END) {
        return 0;
    }
    void* ret = next_heap_alloc_rel + HEAP_START;
    next_heap_alloc_rel += size;
    return ret;
}