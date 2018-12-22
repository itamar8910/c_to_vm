struct A{
    int x;
    int y;
};

int main(){
    return sizeof(struct A) + 3*sizeof(int);
}