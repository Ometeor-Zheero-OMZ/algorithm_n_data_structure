#include <stdio.h>

// 2^n - 1 = O(2^n)
void fn(int n)
{
    if (n > 0)
    {
        printf("%d ", n);
        fn(n - 1);
        fn(n - 1);
    }
}

int main()
{
    int n = 3;
    fn(n);
    return 0;
}