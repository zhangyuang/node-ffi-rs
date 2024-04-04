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
  double *doubleArray;
  int age;
  double doubleProps;
  const char *name;
  char **stringArray;
  int *i32Array;
  int testnum;
  bool boolTrue;
  bool boolFalse;
  // Parent parent;
} Person;

extern "C" Person *getStruct(Person *person) { return person; }

extern "C" Person *createPerson() {
  Person *person = (Person *)malloc(sizeof(Person));

  // Allocate and initialize doubleArray
  person->doubleArray = (double *)malloc(sizeof(double) * 3);
  person->doubleArray[0] = 1.0;
  person->doubleArray[1] = 2.0;
  person->doubleArray[2] = 3.0;

  // Initialize age and doubleProps
  person->age = 30;
  person->doubleProps = 1.23;

  // Allocate and initialize name
  person->name = strdup("John Doe");

  person->stringArray = (char **)malloc(sizeof(char *) * 2);
  person->stringArray[0] = strdup("Hello");
  person->stringArray[1] = strdup("World");

  person->i32Array = (int *)malloc(sizeof(int) * 3);
  person->i32Array[0] = 1;
  person->i32Array[1] = 2;
  person->i32Array[2] = 3;
  person->testnum = 123;
  person->boolTrue = true;
  person->boolFalse = false;

  return person;
}
typedef void (*FunctionPointer)(double a);

extern "C" void callFunction(FunctionPointer func) {
  printf("callFunction");
  double a = 100.11;
  // bool a = false;
  // char *a = "Hello, World!";
  // char *str = (char *)malloc(14 * sizeof(char));
  // strcpy(str, "Hello, World!");
  func(a);
}

extern "C" void bufferToFill(double bufferToFill[3]) {
  bufferToFill[0] = -0.5;
  bufferToFill[1] = 7.5;
  bufferToFill[2] = 3;
  printf("%f", bufferToFill[0]);
}
