#include <stdio.h>
#include <string.h>
#include <stdlib.h>

void print_memory(char* ptr, int size) {
    for (int i = 0; i < size; i++) {
        printf("%02X ", (unsigned char)ptr[i]);
    }
    printf("\n");
}

char* vulnerable_function(char *user_input) {
    size_t input_length = strlen(user_input);

    char* buffer = (char*)malloc(5 * sizeof(char));
    printf("Before overflow: ");
    print_memory(buffer, input_length);

    strcpy(buffer, user_input);  // Overflow happens here

    printf("After overflow:  ");
    print_memory(buffer, input_length);
    return buffer;
}

int main() {
    char malicious_input[] = "hi"; // 11 bytes (10 + null terminator)

    char* malicious_buffer = vulnerable_function(malicious_input);
    
    printf("Buffer content: %s\n", malicious_buffer);
    free(malicious_buffer);

    return 0;
}
