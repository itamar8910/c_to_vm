
struct B{
    int z[3];
};

struct A{
    int x;
    struct B y;
};

int main(){
    struct B b;
    struct A a;
    a.y = b;
    a.y.z[1] = 2;
    return a.y.z[1];
}