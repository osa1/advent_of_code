#include "common.h"

#include <assert.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef enum
{
    REG,
    VAL,
} arg_ty;

typedef struct
{
    arg_ty ty;
    int    val;
} arg;

typedef enum
{
    CPY, INC, DEC, JNZ
} instr_ty;

typedef struct
{
    instr_ty ty;
    arg      arg1;
    arg      arg2;
} instr;

void parse_arg(char* str, arg* arg, int* skip)
{
    if (isdigit(*str) || *str == '-')
    {
        int ret = 0;
        int i   = 0;

        bool negate = false;
        if (*str == '-')
        {
            negate = true;
            ++i;
        }

        while (isdigit(*(str + i)))
        {
            ret *= 10;
            ret += *(str + i) - '0';
            ++i;
        }
        *skip = i;
        arg->ty  = VAL;
        arg->val = negate ? -ret : ret;
    }
    else
    {
        assert(*str == 'a' || *str == 'b' || *str == 'c' || *str == 'd');
        *skip = 1;
        arg->ty = REG;
        arg->val = *str - 'a';
    }
}

void parse_instr(char** str, instr* instr)
{
    char* line = *str;

    if (*line == 'c')
    {
        // cpy
        instr->ty = CPY;
        int skip;
        line += 4;
        parse_arg(line, &instr->arg1, &skip);
        line += skip + 1;
        parse_arg(line, &instr->arg2, &skip);
        line += skip;
    }
    else if (*line == 'i')
    {
        // inc
        instr->ty = INC;
        int skip;
        line += 4;
        parse_arg(line, &instr->arg1, &skip);
        line += skip;
    }
    else if (*line == 'd')
    {
        // dec
        instr->ty = DEC;
        int skip;
        line += 4;
        parse_arg(line, &instr->arg1, &skip);
        line += skip;
    }
    else if (*line == 'j')
    {
        // jnz
        instr->ty = JNZ;
        int skip;
        line += 4;
        parse_arg(line, &instr->arg1, &skip);
        line += skip + 1;
        parse_arg(line, &instr->arg2, &skip);
        line += skip;
    }
    else
    {
        int newline_pos = 0;
        while (line[newline_pos++] != '\n') {}
        line[newline_pos] = '\0';
        fprintf(stderr, "Unrecognized line: \"%s\"\n", line);
        exit(1);
    }

    assert(*line == '\n');
    *str = line + 1;
}

int main()
{
    buffer input_buf;
    buffer_init(&input_buf, 1000);
    read_stdin(&input_buf);

    // int   regs[4] = { 0, 0, 0, 0 }; // Part 1
    int   regs[4] = { 0, 0, 1, 0 }; // Part 2

    instr instrs[1000];

    int   instr_idx = 0;
    char* input     = (char*)input_buf.buf;
    while (*input)
    {
        parse_instr(&input, instrs + instr_idx);
        ++instr_idx;
    }

    printf("Parsed %d instructions.\n", instr_idx);

    int ip = 0;
    while (ip < instr_idx)
    {
        instr* i = instrs + ip;

        if (i->ty == CPY)
        {
            if (i->arg1.ty == REG)
                regs[i->arg2.val] = regs[i->arg1.val];
            else
                regs[i->arg2.val] = i->arg1.val;
            ++ip;
        }
        else if (i->ty == INC)
        {
            ++regs[i->arg1.val];
            ++ip;
        }
        else if (i->ty == DEC)
        {
            --regs[i->arg1.val];
            ++ip;
        }
        else if (i->ty == JNZ)
        {
            if ((i->arg1.ty == REG && regs[i->arg1.val]) || (i->arg1.ty == VAL && i->arg1.val))
                ip += i->arg2.val;
            else
                ++ip;
        }
        else
            assert(false);
    }

    printf("reg a: %d\n", regs[0]);

    buffer_free(&input_buf);
    return 0;
}
