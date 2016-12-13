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
        memmove(v->buf + i + 1, v->buf + i, sizeof(void*) * (v->len - i));
    // insert new element
    v->buf[i] = e;
    ++v->len;
}

// TODO: Remove recursion
static
int bin_search_by(void** buf, int len, void* e, bool* found, cmp_fn cmp)
{
    if (len == 0)
    {
        *found = false;
        return 0; // return insert idx
    }

    int mid     = len / 2;
    int cmp_ret = cmp(e, buf[mid]);

    if (cmp_ret == 0)
    {
        *found = true;
        return mid;
    }
    else if (cmp_ret > 0)
    {
        // search left
        return bin_search_by(buf, mid, e, found, cmp);
    }
    else
    {
        // search right
        return mid + 1 + bin_search_by(buf + mid + 1, len - mid - 1, e, found, cmp);
    }
}

static
int default_eq(void* e1, void* e2)
{
    if (e1 == e2)
        return 0;
    else if (e1 > e2)
        return 1;
    return -1;
}

int vector_bin_search(vector* v, void* e)
{
    bool found;
    int idx = bin_search_by(v->buf, v->len, e, &found, default_eq);
    return found ? idx : -1;
}

void vector_insert_sorted(vector* v, void* e)
{
    bool found;
    int idx = bin_search_by(v->buf, v->len, e, &found, default_eq);
    vector_insert(v, e, idx);
}

void vector_insert_sorted_by(vector* v, void* e, cmp_fn cmp)
{
    bool found;
    int idx = bin_search_by(v->buf, v->len, e, &found, cmp);
    vector_insert(v, e, idx);
}

int vector_bin_search_by(vector* v, void* e, cmp_fn cmp)
{
    bool found;
    int idx = bin_search_by(v->buf, v->len, e, &found, cmp);
    return found ? idx : -1;
}

void vector_print(vector* v)
{
    printf("[");
    for (int i = 0; i < v->len; ++i)
        printf("%p ", v->buf[i]);
    printf("]\n");
}

void vector_clone(vector* dst, vector* src)
{
    dst->len = src->len;
    dst->cap = src->cap;
    dst->buf = malloc(sizeof(void*) * dst->cap);
    memcpy(dst->buf, src->buf, sizeof(void*) * dst->len);
}
