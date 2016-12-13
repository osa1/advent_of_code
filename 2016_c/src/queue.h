#ifndef __QUEUE_H
#define __QUEUE_H

/** A queue for pointer-sized things. */
typedef struct
{
    void** buf;
    int    head;
    int    len;
    int    cap;
} queue;

void queue_init(queue*, int cap);
void queue_free(queue*);
void queue_push(queue*, void*);

/** Check 'len' before using this, we don't signal when queue is empty. */
void* queue_pop(queue*);

#endif
