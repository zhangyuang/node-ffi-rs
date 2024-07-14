#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <string>

extern "C" int sum(int a, int b) { return a + b; }

extern "C" double doubleSum(double a, double b) { return a + b; }

extern "C" double floatSum(float a, float b) { return a + b; }

extern "C" int64_t testbigint(int64_t a) {
   return a;
}

extern "C" const char *concatenateStrings(const char *str1, const char *str2) {
  std::string result = std::string(str1) + std::string(str2);
  char *cstr = new char[result.length() + 1];
  strcpy(cstr, result.c_str());
  return cstr;
}

extern "C" const wchar_t *concatenateWideStrings(const wchar_t *str1,
                                                 const wchar_t *str2) {
  std::wstring result = std::wstring(str1) + std::wstring(str2);
  wchar_t *wcstr = new wchar_t[result.length() + 1];
  wcscpy(wcstr, result.c_str());
  return wcstr;
}

extern "C" char *getStringFromPtr(void *ptr) { return (char *)ptr; };

extern "C" int getValueFromDoublePointer(int **ptr) { return *ptr[0]; };

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

extern "C" double *createArrayFloat(const float *arr, int size) {
  double *vec = (double *)malloc((size) * sizeof(double));
  for (int i = 0; i < size; i++) {
    vec[i] = arr[i];
  }
  return vec;
}

extern "C" char **createArrayString(char **arr, int size) {
  char **vec = (char **)malloc(size * sizeof(char *));
  if (vec == NULL) {
    return NULL;
  }

  for (int i = 0; i < size; i++) {
    vec[i] = strdup(arr[i]);
  }
  return vec;
}
extern "C" bool return_opposite(bool input) { return !input; }

typedef struct stackStruct {
  int age;
} stackStruct;

typedef struct Person {
  int age;
  double *doubleArray;
  Person *parent;
  double doubleProps;
  const char *name;
  char **stringArray;
  int *i32Array;
  uint8_t staticBytes[16];
  bool boolTrue;
  bool boolFalse;
  int64_t longVal;
  struct stackStruct stackStruct;
  char byte;
  char *byteArray;
} Person;

extern "C" Person *getStruct(Person *person) { return person; }

extern "C" Person *createPerson() {
  Person *person = (Person *)malloc(sizeof(Person));

  // Allocate and initialize doubleArray
  double initDoubleArray[] = {1.1, 2.2, 3.3};
  person->doubleArray = (double *)malloc(sizeof(initDoubleArray));
  memcpy(person->doubleArray, initDoubleArray, sizeof(initDoubleArray));

  // Initialize age and doubleProps
  person->age = 23;
  person->doubleProps = 1.1;
  person->byte = 'A';
  uint8_t bytes[16];
  memset(bytes, 99, sizeof(bytes));

  memcpy(person->staticBytes, bytes, sizeof(bytes));

  // Allocate and initialize name
  person->name = strdup("tom");

  char *stringArray[] = {strdup("tom")};
  person->stringArray = (char **)malloc(sizeof(stringArray));
  memcpy(person->stringArray, stringArray, sizeof(stringArray));
  person->stackStruct.age = 16;
  // Allocate and initialize byteArray
  char initByteArray[] = {101, 102};
  person->byteArray = (char *)malloc(sizeof(initByteArray));
  memcpy(person->byteArray, initByteArray, sizeof(initByteArray));

  int initI32Array[] = {1, 2, 3, 4};
  person->i32Array = (int *)malloc(sizeof(initI32Array));
  memcpy(person->i32Array, initI32Array, sizeof(initI32Array));

  person->boolTrue = true;
  person->boolFalse = false;
  person->longVal = 4294967296;

  // Allocate and initialize parent
  person->parent = (Person *)malloc(sizeof(Person));
  person->parent->stackStruct.age = 22;
  double parentDoubleArray[] = {1.1, 2.2, 3.3};
  person->parent->doubleArray = (double *)malloc(sizeof(parentDoubleArray));
  memcpy(person->parent->doubleArray, parentDoubleArray,
         sizeof(parentDoubleArray));

  person->parent->age = 43;
  person->parent->doubleProps = 3.3;
  person->parent->name = strdup("tom father");
  uint8_t pbytes[16];
  memset(pbytes, 88, sizeof(bytes));
  memcpy(person->parent->staticBytes, pbytes, sizeof(pbytes));

  char *pstringArray[] = {strdup("tom"), strdup("father")};
  person->parent->stringArray = (char **)malloc(sizeof(pstringArray));

  memcpy(person->parent->stringArray, pstringArray, sizeof(pstringArray));

  int parentI32Array[] = {5, 6, 7};
  person->parent->i32Array = (int *)malloc(sizeof(parentI32Array));
  memcpy(person->parent->i32Array, parentI32Array, sizeof(parentI32Array));

  person->parent->boolTrue = true;
  person->parent->boolFalse = false;
  person->parent->longVal = 5294967296;
  person->parent->byte = 'B';

  char parentByteArray[] = {103, 104};
  person->parent->byteArray = (char *)malloc(sizeof(parentByteArray));
  memcpy(person->parent->byteArray, parentByteArray, sizeof(parentByteArray));

  return person;
}
typedef const int (*FunctionPointer)(int a, bool b, char *c, double d, char **e,
                                     int *f, Person *g);

extern "C" void callFunction(FunctionPointer func) {
  int a = 100;
  bool b = false;
  double d = 100.11;
  char *c = (char *)malloc(14 * sizeof(char));
  strcpy(c, "Hello, World!");

  char **stringArray = (char **)malloc(sizeof(char *) * 2);
  stringArray[0] = strdup("Hello");
  stringArray[1] = strdup("world");

  int *i32Array = (int *)malloc(sizeof(int) * 3);
  i32Array[0] = 101;
  i32Array[1] = 202;
  i32Array[2] = 303;

  Person *p = createPerson();
  int res = func(a, b, c, d, stringArray, i32Array, p);
  printf("function ret %d\n", res);
}

// 定义 C++ 类
class MyClass {
public:
  std::string name;
  int age;

  MyClass(std::string n, int a) : name(n), age(a) {}

  void print() {
    std::cout << "Name: " << name << ", Age: " << age << std::endl;
  }
};

MyClass *createMyClass(std::string name, int age) {
  return new MyClass(name, age);
}

extern "C" MyClass *createMyClassFromC(const char *name, int age) {
  return createMyClass(std::string(name), age);
}

extern "C" void printMyClass(MyClass *instance) { instance->print(); }

// typedef void (*CallbackType)(const char *);
// extern "C" void call_callback_async() {
//   dispatch_async(dispatch_get_main_queue(), ^{
//     printf("dispatch_async\n");
//     // callback("Hello from dispatched block");
//   });
//   // dispatch_main();
// }
// int call_callback_async(CallbackType callback) {
//   std::async(std::launch::async, [=]() { callback("Hello from async task");
//   }); return 0;
// }
