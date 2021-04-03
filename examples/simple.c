int GetKey(int* key) {
	*key = __syscall(3755);

	return 0;
}

int main() {
	__syscall(626);
	__syscall(607);

	int local_test = 0;
	while (0 == 0) {
		__syscall(626);
		__syscall(607);
	}

	return 0;
}
