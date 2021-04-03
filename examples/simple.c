int GetKey(int* key) {
	*key = __syscall(3755);

	return 0;
}

int main() {
	int local_test = 0;
	GetKey(&local_test);
	GetKey(&local_test);
	GetKey(&local_test);
	GetKey(&local_test);
	GetKey(&local_test);
	GetKey(&local_test);
	GetKey(&local_test);
	GetKey(&local_test);
	GetKey(&local_test);
	GetKey(&local_test);
	GetKey(&local_test);
	GetKey(&local_test);
	GetKey(&local_test);
	GetKey(&local_test);
	GetKey(&local_test);
	GetKey(&local_test);

	return 0;
}
