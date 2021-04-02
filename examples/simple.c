int GetKey(int* key) {
	*key = __syscall(3755); 

	return 0;
}

int main() {
	int test = 0;
	GetKey(&test);

	return 0;
}
