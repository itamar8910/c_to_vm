struct A{
    int x;
    int y;
    int z[];
};

int main(){
    struct A a;
    struct A* a_ptr = &a;
    a_ptr->z[0] = 2;
    return a_ptr->z[0];
}