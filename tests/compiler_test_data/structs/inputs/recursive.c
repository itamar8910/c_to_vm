
struct A{
    int x;
    int y;
};

struct B{
    int x;
    struct A a;
    int y;
};

int main(){
    struct B b;
    b.y = 1;
    b.a.y = 6;
    b.x = 2;
    return b.a.y - b.x + b.y;
}