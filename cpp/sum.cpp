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

extern "C" Person createStruct() {
  Person person;

  // 初始化字段的含义
  person.doubleArray = NULL; // 双精度浮点数数组为空
  person.age = 0;            // 年龄为0
  person.doubleProps = 0.0;  // 双精度浮点数属性为0.0
  person.name = "Unknown";   // 姓名为"Unknown"

  // 初始化字符串数组
  person.stringArray = new char *[3];
  person.stringArray[0] = strdup("Hello");   // 第一个字符串为"Hello"
  person.stringArray[1] = strdup("World");   // 第二个字符串为"World"
  person.stringArray[2] = strdup("ChatGPT"); // 第三个字符串为"ChatGPT"

  // 初始化整数数组
  person.i32Array = new int[4]; // 整数数组，长度为4
  person.i32Array[0] = 10;      // 第一个整数为10
  person.i32Array[1] = 20;      // 第二个整数为20
  person.i32Array[2] = 30;      // 第三个整数为30
  person.i32Array[3] = 40;      // 第四个整数为40

  person.testnum = 0;  // 测试数值为0
  person.boolTrue = 1; // 布尔值为真（1）
  person.boolFalse = 0;
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
