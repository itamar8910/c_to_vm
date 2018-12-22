struct A{
    int x;
    int y;
    int z[3];
};

int main(){
    struct A a;
    struct A* a_ptr = &a;
    a_ptr->x = 2;
    int x = 1;
    return a_ptr->x;
}