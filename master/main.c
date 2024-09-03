#include <stdio.h>
#include <string.h>
#include <time.h>

#define MAX_LENGTH 101 // Max line length is 100 characters plus the newline character.

static inline void calc(char *input, char *result) {
    const char *first = NULL;
    const char *last = NULL;

    // Single for loop to find the first and last digits - was the fastest method found through testing.
    for (const char *ptr = input; *ptr != '\0'; ptr++) {
        if (*ptr >= '0' && *ptr <= '9') {
            if (!first) {
                first = ptr;
            }
            last = ptr;
        }
    }

    result[0] = *first;
    result[1] = *last;
}

int main() {
    struct timespec start_time, end_time;
    // Start time
    clock_gettime(CLOCK_MONOTONIC, &start_time);

    char line[MAX_LENGTH] = {'0'}; // Initialize to 0.

    FILE *inputFile = fopen("input.txt", "r");
    if (inputFile == NULL) {
        perror("Error opening input file");
        return -1;
    }

    FILE *outputFile = fopen("output.txt", "w");
    if (outputFile == NULL) {
        perror("Error opening output file");
        fclose(inputFile);
        return -1;
    }

    char result[] = {'0', '0', '\0'};

    // Read the file line by line
    while (fgets(line, MAX_LENGTH, inputFile)) {
        calc(line, result);
        fputs(result, outputFile);
        fputc('\n', outputFile);
        memset(line, '0', sizeof(line)); // Reinitialize to 0.
    }

    // Close the files
    fclose(inputFile);
    fclose(outputFile);

    // End time
    clock_gettime(CLOCK_MONOTONIC, &end_time);
    // Calculate elapsed time in nanoseconds
    long elapsed_ns = (end_time.tv_sec - start_time.tv_sec) * 1e9 + (end_time.tv_nsec - start_time.tv_nsec);
    printf("Time: %ld ns\n", elapsed_ns);

    return 0;
}
