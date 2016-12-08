#include "common.h"

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

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

    // Enable this for easier debugging:
    // buffer_reserve(buf, 1);
    // buf->buf[buf->len] = '\0';
}

int mod(int i, int m)
{
    int ret = i % m;
    if (ret < 0)
        ret += m;
    return ret;
}
