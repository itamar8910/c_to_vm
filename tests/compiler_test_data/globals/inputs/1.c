int x;

void foo(){
    x += 1;
}

void bar(){
    x *= 2;
}

int main(){
    x = 3;
    foo();
    bar();
    return x;
}