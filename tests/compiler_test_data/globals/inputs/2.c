int x;
int y;

void foo(){
    x += 1;
}

void bar(){
    y *= 2;
}

int main(){
    y = 3;
    foo();
    bar();
    return x == 1 && y == 6;
}