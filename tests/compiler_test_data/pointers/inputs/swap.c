void swap(int* x, int* y){
    int tmp = *x;
    *x = *y;
    *y = tmp;
}

void main(){
    int x = 1;
    int y = 2;
    swap(&x, &y);
    return x - y;
}