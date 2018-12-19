struct A{
    int x;
    int y[3];
};

int main(){
    struct A a[2];
    a[1].y[2] = 5;
    return a[1].y[2];
}