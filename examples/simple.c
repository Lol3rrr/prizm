int main() {
	int local_test = 0;
	int yikes = 123;

	while (0 == 0) {
		__syscall(3755, &local_test, 0, 0, 0);
		yikes = 3;
	}

	return 0;
}
