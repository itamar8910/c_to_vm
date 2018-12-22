int main(){
    int i = 0;
    int j = 8;
    int z = 0;
    for(; i < 5; i ++, j--) {
        z++;
    }
    return i == 5 && j == 3 && z == 5;
}