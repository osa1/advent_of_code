#include "vector.h"

#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void vector_init(vector* v, int cap)
{
    assert(cap > 0);
    v->buf = malloc(sizeof(void*) * cap);
    assert(v->buf);
    v->len = 0;
    v->cap = cap;
}

void vector_free(vector* v)
{
    free(v->buf);
    v->cap = 0;
    v->len = 0;
}

static
void vector_reserve(vector* v, int amt)
{
    int new_cap = v->cap;
    while (new_cap - v->len < amt)
        new_cap *= 2;

    if (new_cap != v->cap)
    {
        v->buf = realloc(v->buf, sizeof(void*) * new_cap);
        v->cap = new_cap;
    }
}

void vector_push(vector* v, void* val)
{
    vector_reserve(v, 1);
    v->buf[v->len] = val;
    ++v->len;
}

void* vector_pop(vector* v)
{
    assert(v->len > 0);
    void* ret = v->buf[v->len - 1];
    --v->len;
    return ret;
}

void vector_insert(vector* v, void* e, int i)
{
    assert(i >= 0);
    assert(i <= v->len);

    vector_reserve(v, 1);
    // shift elements
    if (i != v->len)
        memmove(v->buf + i + 1, v->buf + i, sizeof(void*) * v->len - i);
    // insert new element
    v->buf[i] = e;
    ++v->len;
}

// TODO: Remove recursion
static
int bin_search(void** buf, int len, void* e, bool* found)
{
    if (len == 0)
    {
        *found = false;
        return 0; // return insert idx
    }

    int mid = len / 2;
    if (buf[mid] == e)
    {
        *found = true;
        return mid;
    }
    else if (buf[mid] > e)
    {
        // search left
        return bin_search(buf, mid, e, found);
    }
    else
    {
        // search right
        return mid + 1 + bin_search(buf + mid + 1, len - mid - 1, e, found);
    }
}

int vector_bin_search(vector* v, void* e)
{
    bool found;
    int idx = bin_search(v->buf, v->len, e, &found);
    return found ? idx : -1;
}

void vector_insert_sorted(vector* v, void* e)
{
    bool found;
    int idx = bin_search(v->buf, v->len, e, &found);
    vector_insert(v, e, idx);
}

void vector_print(vector* v)
{
    printf("[");
    for (int i = 0; i < v->len; ++i)
    {
        printf("%p ", v->buf[i]);
    }
    printf("]\n");
}
