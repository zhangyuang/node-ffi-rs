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
typedef struct Parent {
  const char *name;
  int age;
} Parent;

typedef void (*FunctionPointer)(int a, int b);

extern int callFunction(FunctionPointer func);

typedef struct Person {
  const char *name;
  int age;
  Parent parent;
} Person;
extern const Person *getStruct(const Person *p);

#ifdef __cplusplus
}
#endif

#endif // EXAMPLE_H
