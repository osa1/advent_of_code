#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// How many bytes to read in a fread(stdin, ...)
#define READ_CHUNK_SIZE 1024

typedef struct buffer_
{
    uint8_t* buf;
    int len;
    int cap;
} buffer;

void buffer_init(buffer* buf, int cap)
{
    assert(cap > 0);
    buf->buf = malloc(cap);
    assert(buf->buf);
    buf->len = 0;
    buf->cap = cap;
}

void buffer_free(buffer* buf)
{
    free(buf->buf);
}

void buffer_reserve(buffer* buf, int amt)
{
    int new_cap = buf->cap;
    while (new_cap - buf->len < amt)
        new_cap *= 2;

    if (new_cap != buf->cap)
    {
        buf->buf = realloc(buf->buf, new_cap);
        buf->cap = new_cap;
    }
}

void read_stdin(buffer* buf)
{
    for (;;)
    {
        buffer_reserve(buf, READ_CHUNK_SIZE);
        int bytes_read = fread((void*)(buf->buf + buf->len), 1, READ_CHUNK_SIZE, stdin);
        buf->len += bytes_read;
        if (bytes_read < READ_CHUNK_SIZE)
            break;
    }
}

typedef enum { N, E, S, W } DIRS;

int mod(int i, int m)
{
    int ret = i % m;
    if (ret < 0)
        ret += m;
    return ret;
}

int main()
{
    buffer buf;
    buffer_init(&buf, 1000);
    read_stdin(&buf);
    --buf.len; // trailing newline

    int ret = 0;

    // Current direction
    DIRS dir = N;

    int walked_n = 0;
    int walked_e = 0;
    int walked_s = 0;
    int walked_w = 0;

    int i = 0;
    while (i < buf.len)
    {
        if (i != 0)
            i += 2; // skip ", "

        char c = (char)buf.buf[i];
        if (c == 'R')
            dir = mod(dir + 1, 4);
        else if (c == 'L')
            dir = mod(dir - 1, 4);
        else
        {
            fprintf(stderr, "Invalid turn: %c\n", c);
            ret = 1;
            goto exit;
        }

        ++i;
        c = (char)buf.buf[i];

        int num = 0;
        while (c >= '0' && c <= '9')
        {
            num *= 10;
            num += c - '0';
            ++i;
            c = (char)buf.buf[i];
        }

        if (dir == N)
            walked_n += num;
        else if (dir == E)
            walked_e += num;
        else if (dir == S)
            walked_s += num;
        else if (dir == W)
            walked_w += num;
    }

    printf("walked_n: %d\n", walked_n);
    printf("walked_e: %d\n", walked_e);
    printf("walked_s: %d\n", walked_s);
    printf("walked_w: %d\n", walked_w);

    printf("%d\n", abs(walked_n - walked_s) + abs(walked_e - walked_w));
exit:
    buffer_free(&buf);
    return ret;
}
