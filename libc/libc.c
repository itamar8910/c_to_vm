void* malloc(int size);
void free(void* addr);

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
    return str;
}

struct FreeBlock {
    struct FreeBlock* next_free;
    struct FreeBlock* prev_free;
    char* start; 
    int size;
};
struct FreeBlock* free_root;


void malloc_init(){
    // yo we need #define
    int HEAP_START = 4000;
    int HEAP_END = 6000;
    free_root = (struct FreeBlock*) (HEAP_START);
    free_root->next_free = 0;
    free_root->prev_free = 0;
    free_root->start = (void*) (HEAP_START);
    free_root->size = HEAP_END - HEAP_START;
    // puts(itos(free_root->size));
}

void* malloc(int alloc_size){
    if (!free_root) {
        malloc_init();
    }
    // find free block with sufficient size & remove a chunk from it for allocation
    struct FreeBlock* cur = free_root;
    while (cur){
        if (cur->size >= alloc_size + sizeof(struct FreeBlock) + sizeof(int)){
            void* alloc_addr = cur->start + sizeof(int);
            struct FreeBlock* rest_of_cur = (struct FreeBlock*) (alloc_addr + alloc_size);
            if (cur->prev_free) {
                cur->prev_free->next_free = rest_of_cur;
            }
            if (cur->next_free) {
                cur->next_free->prev_free = rest_of_cur;
            }
            int* alloc_size_save = (int*) cur->start;
            struct FreeBlock* cur_next = cur->next_free;
            struct FreeBlock* cur_prev = cur->prev_free;
            int cur_size = cur->size;
            rest_of_cur->next_free = cur_next;
            rest_of_cur->prev_free = cur_prev;
            rest_of_cur->start = (char*) rest_of_cur;
            rest_of_cur->size = cur_size - alloc_size - sizeof(int);
            if (free_root == cur) {
                free_root = rest_of_cur;
            }
            *alloc_size_save = alloc_size;
            return alloc_addr;
        }
        cur = cur->next_free;
    }
    return 0;
}

void free(void* addr){
    // insert a new free block to the free list
    struct FreeBlock* prev_root = free_root;
    int alloc_size = *((int*)(addr-sizeof(int)));
    struct FreeBlock* addr_block = (struct FreeBlock*) addr - sizeof(int);
    if (alloc_size < sizeof(struct FreeBlock)) {
        return; // freed chunk is too small insert into free list
    }
    addr_block->start = addr;
    addr_block->size = alloc_size + sizeof(int);
    addr_block->next_free = prev_root;
    addr_block->prev_free = 0;
    prev_root->prev_free = addr_block;
    free_root = addr_block;
}
