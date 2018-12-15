int main(){
    char a = 'a';
    char* a_ptr = &a;
    *a_ptr += 'e' - 'c';
    return *a_ptr  == 'c' && *a_ptr != 'b';
}