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

void revstr(char* str, int start, int end){
    end -= 1;
    for(; start < end; start++, end--){
        char* c1 = str + start;
        char* c2 = str + end;
        char tmp = *c1;
        *c1 = *c2;
        *c2 = tmp;
    }
}

// integer to string
char* itos(int num) {
    int sign = num >= 0;
    num = num >= 0 ? num : -num;
    int orig_num = num;

    int str_len = 0;
    for(; num > 0; num /= 10, str_len++) {
    }

    str_len = orig_num != 0 ? str_len : 1;
    char* str = malloc(str_len * sizeof(char) + 1 + !sign);
    char* cur_char = str;
    num = orig_num;
    for(int i = 0; i < str_len; i++, cur_char++, num /= 10) {
        *cur_char = '0' + (num % 10);
    }
    if (!sign) {
        *cur_char = '-';
        cur_char++;
    }
    *cur_char = 0; // termiante string
    revstr(str, 0, str_len + !sign);
    // TODO: reverse digits
    return str;
}