int main() {
	int key = 0;

	for (int x = 0; x < 10; x = x + 1) {
		__syscall(3755, &key, 0, 0, 0);
	}

	return 0;
}
