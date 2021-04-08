int main() {
	int key = 0;
	void* vram_start = 2885681152;

	__syscall(3755, &key, 0, 0, 0);

	while (0 == 0) {
		__syscall(626, 0, 0, 0, 0);

		for (int i = 0; i < 10; i = i + 1) {
			*(vram_start + i) = 0;
		}
		
		__syscall(607, 0, 0, 0, 0);

		__syscall(3755, &key, 0, 0, 0);
	}

	return 0;
}
