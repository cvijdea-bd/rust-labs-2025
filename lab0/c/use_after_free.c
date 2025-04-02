#include <stdio.h>
#include <stdlib.h>

int main() {
    int *ptr = malloc(sizeof(int));
    *ptr = 42;
    printf("Manually declared and initialized int pointer with value 42\n");
    
    printf("Value before free: %d\n", *ptr); // The actual will also be 42

    printf("Freeing memory...\n");
    free(ptr);
    printf("Memory freed.\n");

    printf("Value after free: %d\n", *ptr);

    return 0;
}