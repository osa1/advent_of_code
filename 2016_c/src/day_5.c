#include <assert.h>
#include <inttypes.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include <openssl/md5.h>

int main()
{
    char password[9] = { 'x', 'x', 'x', 'x', 'x', 'x', 'x', 'x', '\0' };

    for (uint64_t i = 0; ; ++i)
    {
        uint8_t digest[MD5_DIGEST_LENGTH];
        char md5_input[1000];
        snprintf(md5_input, 1000, "reyedfim%" PRIu64, i);

        MD5_CTX ctx;
        MD5_Init(&ctx);
        MD5_Update(&ctx, md5_input, strlen(md5_input));
        MD5_Final(digest, &ctx);

        bool indicates =
            digest[0] == 0x00 &&
            digest[1] == 0x00 &&
            digest[2] <= 0x0F;

        if (indicates)
        {
            char str[3];
            snprintf(str, 3, "%02x", digest[2]);

            if (str[1] >= '0' && str[1] <= '7' && password[str[1] - '0'] == 'x')
            {
                printf("Found a char:\n");
                for (int i = 0; i < MD5_DIGEST_LENGTH; ++i)
                    printf("%02x", digest[i]);
                printf("\n");
                printf("input: %s\n", md5_input);

                int password_char_idx = str[1] - '0';
                snprintf(str, 3, "%02x", digest[3]);
                password[password_char_idx] = str[0];
                printf("password char idx: %d\n", password_char_idx);
                printf("password char: %c\n\n", str[0]);
            }

            bool end = true;
            for (int x = 0; x < 8; ++x)
                if (password[x] == 'x')
                    end = false;

            if (end)
            {
                printf("password: \"%s\"\n", password);
                break;
            }
        }
    }

    return 0;
}
