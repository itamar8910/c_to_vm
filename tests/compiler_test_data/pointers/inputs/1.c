int main(){
    int a = 3;
    int* b = &a;
    *b = *b + 1;
    return a + *b;
}
