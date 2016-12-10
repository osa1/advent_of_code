// Very messy and hacky solution here. It'd be fun if Advent of Code gave us
// huge inputs, I'd have to implement some abstractions here.

#include "common.h"

#include <assert.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>

typedef struct bot_
{
    int hi;
    int lo;
} bot;

/** Length of the line, including newline character. */
static
int line_len(char* input)
{
    int ret = 0;
    while (input[ret++] != '\n') {}
    return ret;
}

static
void give_to_bot(int val, int bot_idx, bot* bots)
{
    if ((val == 61 && (bots[bot_idx].lo == 17 || bots[bot_idx].hi == 17))
            || (val == 17 && (bots[bot_idx].lo == 61 && bots[bot_idx].hi)))
        printf("FOUND THE BOT: %d\n", bot_idx);

    if (bots[bot_idx].lo == -1 && bots[bot_idx].hi == -1)
        bots[bot_idx].lo = val;
    else if (bots[bot_idx].lo == -1)
    {
        if (val < bots[bot_idx].hi)
            bots[bot_idx].lo = val;
        else
        {
            bots[bot_idx].lo = bots[bot_idx].hi;
            bots[bot_idx].hi = val;
        }
    }
    else
    {
        if (val > bots[bot_idx].lo)
            bots[bot_idx].hi = val;
        else
        {
            bots[bot_idx].hi = bots[bot_idx].lo;
            bots[bot_idx].lo = val;
        }
    }
}

int main()
{
    buffer input_buf;
    buffer_init(&input_buf, 1000);
    read_stdin(&input_buf);

    bot bots[300];
    for (int i = 0; i < 300; ++i)
    {
        bots[i].lo = -1;
        bots[i].hi = -1;
    }

    int outputs[300];
    memset(outputs, 0, sizeof(int) * 300);

    // Need to run each line at most once. We linearly scan all the lines. if a
    // line can be run, we run it, and mark the first character of the line
    // with 's' (for "skip"). Otherwise we skip the line and run lines that we
    // can run. The iterate until no more lines can be run.

    char* input = (char*)input_buf.buf;
    int i = 0;

    bool cont = true;
    do {
        cont = false;
        i = 0;
        while (input[i] != '\0')
        {
            if (input[i] == 's')
            {
                // skip the line
                // printf("skipping line\n");
                i += line_len(input + i);
            }
            else if (input[i] == 'b')
            {
                cont = true;

                // "bot N gives low to <bot|output> M and high to <bot|output> T"

                ////////////////////////////////////////////////////////////////////
                // Parse N
                ////////////////////////////////////////////////////////////////////

                int n = 0;
                char* line = input + i;
                int n_idx = 4;

                while (isdigit(line[n_idx]))
                {
                    n *= 10;
                    n += line[n_idx] - '0';
                    ++n_idx;
                }

                assert(line[n_idx] == ' ');

                ////////////////////////////////////////////////////////////////////
                // Parse M
                ////////////////////////////////////////////////////////////////////

                int m = 0;
                int m_idx;
                bool m_is_bot;
                if (line[n_idx + 14] == 'b')
                {
                    m_idx = n_idx + 18;
                    m_is_bot = true;
                }
                else
                {
                    assert(line[n_idx + 14] == 'o');
                    m_idx = n_idx + 21;
                    m_is_bot = false;
                }

                assert(isdigit(line[m_idx]));
                while (isdigit(line[m_idx]))
                {
                    m *= 10;
                    m += line[m_idx] - '0';
                    ++m_idx;
                }

                assert(line[m_idx] == ' ');

                ////////////////////////////////////////////////////////////////////
                // Parse T
                ////////////////////////////////////////////////////////////////////

                int t = 0;
                int t_idx;
                bool t_is_bot;
                if (line[m_idx + 13] == 'b')
                {
                    t_idx = m_idx + 17;
                    t_is_bot = true;
                }
                else
                {
                    assert(line[m_idx + 13] == 'o');
                    t_idx = m_idx + 20;
                    t_is_bot = false;
                }

                assert(isdigit(line[t_idx]));
                while (isdigit(line[t_idx]))
                {
                    t *= 10;
                    t += line[t_idx] - '0';
                    ++t_idx;
                }

                assert(line[t_idx] == '\n');

                ////////////////////////////////////////////////////////////////////
                // Debugging
                ////////////////////////////////////////////////////////////////////

                // printf("bot %d gives low to %s %d and high to %s %d\n",
                //         n, m_is_bot ? "bot" : "output", m, t_is_bot ? "bot" : "output", t);

                ////////////////////////////////////////////////////////////////////
                // Run the action if possible
                ////////////////////////////////////////////////////////////////////

                if (bots[n].lo != -1 && bots[n].hi != -1)
                {
                    int lo = bots[n].lo;
                    int hi = bots[n].hi;
                    assert(lo < hi);

                    bots[n].lo = -1;
                    bots[n].hi = -1;

                    // give low
                    if (m_is_bot)
                        give_to_bot(lo, m, bots);
                    else
                        outputs[m] = lo;

                    // give high
                    if (t_is_bot)
                        give_to_bot(hi, t, bots);
                    else
                        outputs[t] = hi;

                    *line = 's';
                }

                i += t_idx + 1;
            }
            else if (input[i] == 'v')
            {
                cont = true;

                // "value N goes to bot M"

                char* line = input + i;
                int val = 0;
                int val_idx = 6;
                assert(isdigit(line[val_idx]));
                while (isdigit(line[val_idx]))
                {
                    val *= 10;
                    val += line[val_idx] - '0';
                    ++val_idx;
                }

                assert(line[val_idx] == ' ');

                int bot_idx = val_idx + 13;
                int bot = 0;
                assert(isdigit(line[bot_idx]));
                while (isdigit(line[bot_idx]))
                {
                    bot *= 10;
                    bot += line[bot_idx] - '0';
                    ++bot_idx;
                }

                assert(line[bot_idx] == '\n');

                ////////////////////////////////////////////////////////////////////
                // Debugging
                ////////////////////////////////////////////////////////////////////

                // printf("value %d goes to bot %d\n", val, bot);

                ////////////////////////////////////////////////////////////////////
                // Run the action if possible
                ////////////////////////////////////////////////////////////////////

                give_to_bot(val, bot, bots);
                *line = 's';

                i += bot_idx + 1;
            }
            else
                assert(false);
        }
    } while (cont);

    printf("PART 2: %d\n", outputs[0] * outputs[1] * outputs[2]);

    buffer_free(&input_buf);
    return 0;
}
