int main(){
    int a[2][3];
    a[0][0] = 1;
    a[0][1] = 2;
    a[0][2] = 3;
    a[1][0] = 4;
    int* p = &a[0][0];
    p += 3;
    return *p;
}