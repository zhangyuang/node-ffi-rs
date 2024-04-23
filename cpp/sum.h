#ifndef EXAMPLE_H
#define EXAMPLE_H

#ifdef __cplusplus
extern "C" {
#endif

// 函数声明
extern int sum(int a, int b);
extern double doubleSum(double a, double b);
extern const char *concatenateStrings(const char *str1, const char *str2);
extern void noRet();
extern int *createArrayi32(const int *arr, int size);
extern double *createArrayDouble(const double *arr, int size);
extern char **createArrayString(char **arr, int size);
typedef struct {
  int bytes[2];
} cpu_svn_t;
extern float *createArrayFloat(const float *arr, int size);
extern int pck_cert_select(const cpu_svn_t *platform_svn, int bytes[2]);
#ifdef __cplusplus
}
#endif

#endif // EXAMPLE_H
