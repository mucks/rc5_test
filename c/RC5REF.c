#include <stdio.h>

/* RC5REF.C -- Reference implementation of RC5-32/12/16 in C.        */
/* Copyright (C) 1995 RSA Data Security, Inc.                        */
typedef unsigned long WORD;          /* Should be 32-bit = 4 bytes
                                      */
#define w 32                         /* word size in bits                 */
#define r 12                         /* number of rounds                  */
#define b 16                         /* number of bytes in key            */
#define c 4                          /* number  words in key = ceil(8*b/w)*/
#define t 26                         /* size of table S = 2*(r+1) words   */
WORD S[t];                           /* expanded key table                */
WORD P = 0xb7e15163, Q = 0x9e3779b9; /* magic constants             */
/* Rotation operators. x must be unsigned, to get logical right shift*/
#define ROTL(x, y) (((x) << (y & (w - 1))) | ((x) >> (w - (y & (w - 1)))))
#define ROTR(x, y) (((x) >> (y & (w - 1))) | ((x) << (w - (y & (w - 1)))))

void RC5_ENCRYPT(WORD *pt, WORD *ct) /* 2 WORD input pt/output ct    */
{
    printf("\nS[0] %d\n", S[0]);
    printf("pt[0]: %d\n", pt[0]);
    printf("S[1] %d\n", S[1]);
    printf("pt[1]: %d\n", pt[1]);

    WORD i, A = pt[0] + S[0], B = pt[1] + S[1];

    printf("A: %d\n", A);
    printf("B: %d\n", B);

    for (i = 1; i <= r; i++)
    {
        A = ROTL(A ^ B, B) + S[2 * i];
        B = ROTL(B ^ A, A) + S[2 * i + 1];
        printf("A%d: %d\n", i, A);
        printf("B%d: %d\n", i, B);
    }
    printf("A: %d\n", A);
    printf("B: %d\n", B);

    ct[0] = A;
    ct[1] = B;
}

void RC5_DECRYPT(WORD *ct, WORD *pt) /* 2 WORD input ct/output pt    */
{
    WORD i, B = ct[1], A = ct[0];
    for (i = r; i > 0; i--)
    {
        B = ROTR(B - S[2 * i + 1], A) ^ A;
        A = ROTR(A - S[2 * i], B) ^ B;
    }
    pt[1] = B - S[1];
    pt[0] = A - S[0];
}

void RC5_SETUP(unsigned char *K) /* secret input key K[0...b-1]      */
{
    WORD i, j, k, u = w / 8, A, B, L[c];
    /* Initialize L, then S, then mix key into S */
    for (i = b - 1, L[c - 1] = 0; i != -1; i--)
    {
        L[i / u] = (L[i / u] << 8) + K[i];
    }
    printf("\nL---\n");
    for (i = 0; i < c; i++)
    {
        printf("%d\n", L[i]);
    }
    printf("---");

    for (S[0] = P, i = 1; i < t; i++)
    {
        S[i] = S[i - 1] + Q;
    }
    printf("\nS---\n");
    for (i = 0; i < t; i++)
    {
        printf("%d\n", S[i]);
    }
    printf("%d\n", S[i]);
    printf("---");

    for (A = B = i = j = k = 0; k < 3 * t; k++, i = (i + 1) % t, j = (j + 1) % c) /* 3*t > 3*c */
    {
        A = S[i] = ROTL(S[i] + (A + B), 3);
        B = L[j] = ROTL(L[j] + (A + B), (A + B));
    }

    printf("\nS---\n");
    for (i = 0; i < sizeof(S); i++)
    {
        printf("%d\n", S[i]);
    }
}

void main()
{
    WORD i, j, pt1[2], pt2[2], ct[2] = {0, 0};
    unsigned char key[b];

    if (sizeof(WORD) != 4)
    {
        printf("R5 Error: Word has %d bytes. \n", sizeof(WORD));
    }

    for (i = 0; i < 6; i++)
    {
        pt1[0] = ct[0];
        pt1[1] = ct[1];
        unsigned char key[] = {
            0x00,
            0x01,
            0x02,
            0x03,
            0x04,
            0x05,
            0x06,
            0x07,
            0x08,
            0x09,
            0x0A,
            0x0B,
            0x0C,
            0x0D,
            0x0E,
            0x0F,
        };
        unsigned char pt[] = {0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77};

        RC5_SETUP(key);
        RC5_ENCRYPT(pt, ct);
    }
}