struct A{
    int x;
    int y;
    int z;
};

int main(){
    struct A a;
    a.x = 7;
    a.y = 4;
    a.z = 2;
    return a.y;
}