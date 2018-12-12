/*
This test case is not yet supported
passing bare structs (i.e passing structs as value) is not yet supported
*/
struct A{
    int x;
    int y;
    int z;
};

void foo(struct A b){
    b.x = b.y - b.z;
}

int main(){
    struct A a;
    a.x = 7;
    a.y = 4;
    a.z = 2;
    foo(a);
    return a.x;
}