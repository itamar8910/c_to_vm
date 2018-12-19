struct A{
    int x;
    int z[3];
};

int main(){
    struct A a;
    a.z[0] = 1;
    return a.z[0];
}