void mul(int* x, int y){
    *x *= y;
}

int main(){
    int x = 7;
    mul(&x, 3);
    return x;
}