int main() {
	int test = 0;
	test = 13;

	while (0 == 0) {
		__syscall(3755, &test, 0, 0, 0);
	}

	return 0;
}
