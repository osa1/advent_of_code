#ifndef __COMMON_H
#define __COMMON_H

#include <stdint.h>

typedef struct buffer_
{
    uint8_t* buf;
    int len;
    int cap;
} buffer;

// How many bytes to read in a fread(stdin, ...)
#define READ_CHUNK_SIZE 1024

void buffer_init(buffer*, int cap);
void buffer_free(buffer*);
void buffer_push(buffer*, uint8_t);
void buffer_push_str(buffer*, uint8_t*, int len);

void read_stdin(buffer* buf);

int mod(int i, int m);

#endif
