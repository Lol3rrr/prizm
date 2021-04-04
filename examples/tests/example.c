#include <display_syscalls.h>
#include <keyboard_syscalls.h>
 
void main(void) {
	int key;
	while (1) {
        Bdisp_AllClr_VRAM();
        GetKey(&key);
		switch (key) {
		}
	}
 
	return;
}