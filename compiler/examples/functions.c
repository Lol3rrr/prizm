int store(int value_1, int value_2) {
return (value_1 + value_2);
}

int main() {
int* target = 13123;
*target = store(1, 2);
return 0;
}
