#ifndef EXAMPLE_H
#define EXAMPLE_H

#ifdef __cplusplus
extern "C" {
#endif

int sum(int a, int b);
double doubleSum(double a, double b);

const char *concatenateStrings(const char *str1, const char *str2);
void noRet();
int *createArrayi32(const int *arr, int size);

char **createArrayString(const char **arr, int size);

#ifdef __cplusplus
}
#endif

#endif // EXAMPLE_H
