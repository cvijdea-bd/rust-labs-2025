
#include <stdio.h>
#include <stdlib.h>

int main() {
    int *ptr = malloc(sizeof(int));
    free(ptr);
    free(ptr);  // This may go freestyle

    return 0;
}