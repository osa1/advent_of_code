#include "common.h"
#include "queue.h"

#include <assert.h>
#include <stdlib.h>

void queue_init(queue* q, int cap)
{
    assert(cap > 0);
    q->buf = malloc(sizeof(void*) * cap);
    assert(q->buf);
    q->head = 0;
    q->len  = 0;
    q->cap  = cap;
}

void queue_free(queue* q)
{
    free(q->buf);
}

static
void queue_reserve(queue* q, int amt)
{
    int new_cap = q->cap;
    while (new_cap - q->len < amt)
        new_cap *= 2;

    if (new_cap != q->cap)
    {
        void** new_buf = malloc(sizeof(void*) * new_cap);
        assert(new_buf);
        for (int i = 0; i < q->len; ++i)
            new_buf[i] = q->buf[mod(q->head + i, q->cap)];

        free(q->buf);
        q->buf  = new_buf;
        q->cap  = new_cap;
        q->head = 0;
    }
}

void queue_push(queue* q, void* e)
{
    queue_reserve(q, 1);
    int tail = mod(q->head + q->len, q->cap);
    q->buf[tail] = e;
    ++q->len;
}

void* queue_pop(queue* q)
{
    assert(q->len != 0);
    void* ret = q->buf[q->head];
    q->head = mod(q->head + 1, q->cap);
    --q->len;
    return ret;
}
