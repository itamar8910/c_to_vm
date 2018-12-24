
#include <stdio.h>
#include <stdlib.h>
#define MEM_SIZE 10000
#define HEAP_START 5000
#define HEAP_END 8000


/*
We store a "free list" which is a linked list of free memory blocks.
when a block is allocated, the block's size is written as an int that's stored before the first byte of the block 
*/

char* MEM;


int rel_addr(void* p) {
    if (p) {
        return p - (void*)MEM;
    } else{
        return -1;
    }
}

typedef struct FreeBlock {
    struct FreeBlock* next_free;
    struct FreeBlock* prev_free;
    char* start; 
    int size;
} FreeBlock;

FreeBlock* free_root;


void print_free_mem() {
    FreeBlock* cur = free_root;
    while (cur) {
        printf("[start: %d, size: %d, next: %d, prev:%d], ", rel_addr(cur->start), cur->size, rel_addr(cur->next_free), rel_addr(cur->prev_free));
        cur = cur->next_free;
    }
    printf("\n");
}

void init(){
    free_root = (FreeBlock*) (MEM + HEAP_START);
    free_root->next_free = NULL;
    free_root->prev_free = NULL;
    free_root->start = (void*) (MEM + HEAP_START);
    free_root->size = HEAP_END - HEAP_START;
}

void* my_malloc(int alloc_size){
    // find free block with sufficient size & remove a chunk from it for allocation
    FreeBlock* cur = free_root;
    while (cur){
        if (cur->size >= alloc_size + sizeof(FreeBlock) + sizeof(int)){
            void* alloc_addr = cur->start + sizeof(int);
            FreeBlock* rest_of_cur = (FreeBlock*) (alloc_addr + alloc_size);
            if (cur->prev_free) {
                cur->prev_free->next_free = rest_of_cur;
            }
            if (cur->next_free) {
                cur->next_free->prev_free = rest_of_cur;
            }
            int* alloc_size_save = (int*) cur->start;
            FreeBlock* cur_next = cur->next_free;
            FreeBlock* cur_prev = cur->prev_free;
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
    return NULL;
}

void my_free(void* addr){
    // insert a new free block to the free list
    FreeBlock* prev_root = free_root;
    int alloc_size = *((int*)(addr-sizeof(int)));
    FreeBlock* addr_block = (FreeBlock*) addr - sizeof(int);
    if (alloc_size < sizeof(FreeBlock)) {
        return; // freed chunk is too small insert into free list
    }
    addr_block->start = addr;
    addr_block->size = alloc_size + sizeof(int);
    addr_block->next_free = prev_root;
    addr_block->prev_free = NULL;
    prev_root->prev_free = addr_block;
    free_root = addr_block;
}


int main(){
    MEM = (char* ) malloc(MEM_SIZE);
    init();

    int* myptr = (int*)my_malloc(sizeof(int) * 20);
    print_free_mem();
    my_free((void*)myptr);
    print_free_mem();
    myptr = (int*)my_malloc(sizeof(int) * 40);
    print_free_mem();
    my_free((void*)myptr);
    print_free_mem();
}