#include <stddef.h>
#include <stdlib.h>
#include <stdio.h>

static const char* parse_line(const char* s, int* out) {
    if (!*s) {
        *out = 0;
        return NULL;
    }

    int sign = *s++ == 'L' ? -1 : 1;
    int num = 0;
    for (; *s != '\0' && *s != '\n'; s++)
        num = num * 10 + *s - '0';
    *out = sign * num;

    return *s ? s+1 : NULL;
}

int day01_part1(const char* input) {
    int total = 0;
    int angle = 50;
    do {
        int value;
        input = parse_line(input, &value);

        angle = (angle + value) % 100;
        angle += angle < 0 ? 100 : 0;
        total += angle == 0;
    } while (input);

    return total;
}

int day01_part2(const char *input) {
    int total = 0;
    int angle = 50;
    do {
        int value;
        input = parse_line(input, &value);

        bool is_neg = value < 0;
        int offset = angle - is_neg * 100;
        int sum = offset + value;
        int div = sum / 100;
        total += abs(div) - (offset == -100);

        angle = sum % 100;
        angle += angle < 0 ? 100 : 0;
    } while (input);

    return total;
}
