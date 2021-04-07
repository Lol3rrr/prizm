int main() {
	int key = 0;
	void* vram_start = 2885681152;
	void* vram_other = 2885681154;

	__syscall(3755, &key, 0, 0, 0);

	while (0 == 0) {
		__syscall(626, 0, 0, 0, 0);

		*vram_start = 0;
		*vram_other = 0;
		
		__syscall(607, 0, 0, 0, 0);

		__syscall(3755, &key, 0, 0, 0);
	}

	return 0;
}
