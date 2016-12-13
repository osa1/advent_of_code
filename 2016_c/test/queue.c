#include "src/queue.h"

#include <assert.h>
#include <stdint.h>
#include <stdlib.h>

int main()
{
    queue q;
    queue_init(&q, 1);

    for (uint64_t i = 0; i < 500; ++i)
        queue_push(&q, NULL);

    for (uint64_t i = 0; i < 500; ++i)
        queue_pop(&q);

    for (uint64_t i = 0; i < 10000; ++i)
        queue_push(&q, (void*)i);

    for (uint64_t i = 0; i < 10000; ++i)
    {
        assert(queue_pop(&q) == (void*)i);
    }

    queue_free(&q);
    queue_init(&q, 1);

    queue_push(&q, (void*)1);
    queue_push(&q, (void*)2);
    assert(queue_pop(&q) == (void*)1);
    assert(queue_pop(&q) == (void*)2);
    queue_push(&q, (void*)3);
    queue_push(&q, (void*)4);
    assert(queue_pop(&q) == (void*)3);
    assert(queue_pop(&q) == (void*)4);

    queue_free(&q);
    return 0;
}
