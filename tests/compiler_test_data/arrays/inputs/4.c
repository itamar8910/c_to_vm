int main(){
    int mul[10][10];
    for(int i = 0; i < 10; i++){
        for(int j = 0; j < 10; j++){
            mul[i][j] = i * j;
        }
    }
    return mul[5][9];
}