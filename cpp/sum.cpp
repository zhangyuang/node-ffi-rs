#include <cstdio>
#include <cstring>
#include <iostream>
#include <string>
#include <vector>

extern "C" int sum(int a, int b) { return a + b; }

extern "C" double doubleSum(double a, double b) { return a + b; }

extern "C" const char *concatenateStrings(const char *str1, const char *str2) {
  std::string result = std::string(str1) + std::string(str2);
  char *cstr = new char[result.length() + 1];
  strcpy(cstr, result.c_str());
  return cstr;
}

extern "C" void noRet() { printf("%s", "hello world\n"); }

extern "C" int *createArrayi32(const int *arr, int size) {
  int *vec = (int *)malloc((size) * sizeof(int));
  for (int i = 0; i < size; i++) {
    vec[i] = arr[i];
  }
  return vec;
}

extern "C" double *createArrayDouble(const double *arr, int size) {
  double *vec = (double *)malloc((size) * sizeof(double));
  for (int i = 0; i < size; i++) {
    vec[i] = arr[i];
  }
  return vec;
}

extern "C" char **createArrayString(char **arr, int size) {
  char **vec = (char **)malloc((size) * sizeof(char *));
  for (int i = 0; i < size; i++) {
    vec[i] = arr[i];
  }
  return vec;
}

extern "C" bool return_opposite(bool input) { return !input; }

typedef struct Parent {
  int age;
  // const char *name;
} Parent;

typedef struct Person {
  const char *name;
  int age;
  double doubleProps;
  char **stringArray;
  double *doubleArray;
  int *i32Array;
  // Parent parent;
} Person;

extern "C" const Person *getStruct(const Person *person) {
  printf("Name: %s\n", person->name);
  printf("Age: %d\n", person->age);
  printf("doubleProps: %f \n", person->doubleProps);
  printf("stringArray: %s\n", person->stringArray[0]);
  printf("stringArray: %s\n", person->stringArray[1]);
  printf("doubleArray: %f\n", person->doubleArray[0]);
  printf("doubleArray: %f\n", person->doubleArray[1]);
  printf("i32Array: %d\n", person->i32Array[0]);
  // printf("Parent Age: %d\n", person->parent.age);
  // printf("Parent Name: %s\n", person->parent.name);
  return person;
}

typedef int (*FunctionPointer)(int a, int b);

extern "C" void callFunction(FunctionPointer func) {
  printf("callFunction");
  int a = 1;
  int b = 2;
  func(a, b);
}

extern "C" void bufferToFill(double bufferToFill[3]) {
  bufferToFill[0] = -0.5;
  bufferToFill[1] = 7.5;
  bufferToFill[2] = 3;
  printf("%f", bufferToFill[0]);
}
