#include <cstdint>
#include <cstdio>
#include <cstring>

#include <string>

extern "C" int sum(int a, int b) { return a + b; }

extern "C" double doubleSum(double a, double b) { return a + b; }

extern "C" const char *concatenateStrings(const char *str1, const char *str2) {
  printf("%p", str1);
  std::string result = std::string(str1) + std::string(str2);
  char *cstr = new char[result.length() + 1];
  strcpy(cstr, result.c_str());
  return cstr;
}

extern "C" char *getStringFromPtr(void *ptr) { return (char *)ptr; };

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

typedef struct Person {
  int age;
  double *doubleArray;
  Person *parent;
  double doubleProps;
  const char *name;
  char **stringArray;
  int *i32Array;
  bool boolTrue;
  bool boolFalse;
  int64_t longVal;
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

  // Allocate and initialize name
  person->name = strdup("tom");

  char *stringArray[] = {strdup("tom")};
  person->stringArray = (char **)malloc(sizeof(stringArray));
  memcpy(person->stringArray, stringArray, sizeof(stringArray));

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
  double parentDoubleArray[] = {1.1, 2.2, 3.3};
  person->parent->doubleArray = (double *)malloc(sizeof(parentDoubleArray));
  memcpy(person->parent->doubleArray, parentDoubleArray,
         sizeof(parentDoubleArray));

  person->parent->age = 43;
  person->parent->doubleProps = 3.3;
  person->parent->name = strdup("tom father");

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
typedef void (*FunctionPointer)(int a, bool b, char *c, char **d, int *e,
                                Person *p);

extern "C" void callFunction(FunctionPointer func) {
  printf("callFunction\n");

  for (int i = 0; i < 2; i++) {
    int a = 100;
    bool b = false;
    double ddd = 100.11;
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
    func(a, b, c, stringArray, i32Array, p);
  }
}

typedef void (*FunctionPointerDouble)(double a);

extern "C" void callFunctionDouble(FunctionPointerDouble func) {

  double ddd = 100.11;
  printf("Memory address of ddd: %p\n", (void *)&ddd);

  func(ddd);
}

extern "C" void bufferToFill(double bufferToFill[3]) {
  bufferToFill[0] = -0.5;
  bufferToFill[1] = 7.5;
  bufferToFill[2] = 3;
  printf("%f", bufferToFill[0]);
}

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
