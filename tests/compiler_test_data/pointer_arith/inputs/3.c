struct A{
    int x;
    int y;
    int z;
};

int main(){
    struct A* p = 100;
    p--;
    return p;
}