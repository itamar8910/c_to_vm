int main(){
    int a = 3;
    int b = a++;
    int c = ++a;
    return (a==5 && b==3 && c == 5);
}