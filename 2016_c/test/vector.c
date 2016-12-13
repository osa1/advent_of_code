#include "../src/vector.h"

#include <assert.h>
#include <stdint.h>
#include <stdio.h>

int main()
{
    vector v;
    vector_init(&v, 1);

    vector_push(&v, (void*)1);
    vector_push(&v, (void*)2);

    assert(vector_pop(&v) == (void*)2);
    assert(vector_pop(&v) == (void*)1);

    assert(vector_bin_search(&v, (void*)0) == -1);
    assert(vector_bin_search(&v, (void*)11) == -1);

    for (uint64_t i = 1; i <= 10; ++i)
    {
        vector_push(&v, (void*)i);
        assert(vector_bin_search(&v, (void*)0) == -1);
        assert(vector_bin_search(&v, (void*)11) == -1);
    }

    // printf("9 is at idx %d\n", vector_bin_search(&v, (void*)(uint64_t)9));
    for (int i = 1; i <= 10; ++i)
    {
        int idx = vector_bin_search(&v, (void*)(uint64_t)i);
        printf("%d at idx %d\n", i, idx);
        assert(i == idx + 1);
    }

    vector_print(&v);
    vector_insert_sorted(&v, (void*)0);
    vector_print(&v);

    vector_free(&v);
    vector_init(&v, 1);

    for (int i = 9; i >= 0; --i)
        vector_insert_sorted(&v, (void*)(uint64_t)i);

    for (int i = 0; i < 10; ++i)
    {
        int idx = vector_bin_search(&v, (void*)(uint64_t)i);
        printf("%d at idx %d\n", i, idx);
        assert(i == idx);
    }

    vector_free(&v);
    return 0;
}
