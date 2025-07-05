#ifndef RUNTIME_H
#define RUNTIME_H

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Type tags
#define TAG_MASK 0b111
#define INT_TAG 0b001
#define BOOL_TAG 0b010
#define CHAR_TAG 0b011
#define NULL_VAL ((void *)0x14)

#define FALSE_VAL ((void *)BOOL_TAG)
#define TRUE_VAL ((void *)(0b110))

static inline void *int_to_val(int64_t i) {
  return (void *)((i << 3) | INT_TAG);
}

static inline int64_t val_to_int(void *val) { return ((int64_t)val >> 3); }

static inline void *char_to_val(char c) {
  return (void *)(((uint64_t)c << 3) | CHAR_TAG);
}
static inline char val_to_char(void *v) { return (char)(((uint64_t)v) >> 3); }

static inline int is_int(void *v) {
  return ((uintptr_t)v & TAG_MASK) == INT_TAG;
}
static inline int is_bool(void *v) {
  return ((uintptr_t)v & TAG_MASK) == BOOL_TAG;
}
static inline int is_char(void *v) {
  return ((uintptr_t)v & TAG_MASK) == CHAR_TAG;
}
static inline int is_heap_ptr(void *v) {
  return ((uintptr_t)v & TAG_MASK) == 0;
}

// Heap object types
#define TYPE_DOUBLE_TAG 0
#define TYPE_STRING_TAG 1
#define TYPE_PAIR_TAG 2
#define TYPE_SYMBOL_TAG 3

typedef struct {
  uint8_t tag;
  union {
    double as_double;
    char *as_string;
    struct {
      void *car;
      void *cdr;
    } as_pair;
    char *as_symbol;
  };
} HeapObject;

// Boxed constructors
static inline void *box_double(double f) {
  HeapObject *obj = (HeapObject *)malloc(sizeof(HeapObject));
  obj->tag = TYPE_DOUBLE_TAG;
  obj->as_double = f;
  return (void *)obj;
}

static inline void *box_string(const char *s) {
  HeapObject *obj = (HeapObject *)malloc(sizeof(HeapObject));
  obj->tag = TYPE_STRING_TAG;
  obj->as_string = strdup(s);
  return (void *)obj;
}

static inline void *cons(void *car, void *cdr) {
  HeapObject *obj = (HeapObject *)malloc(sizeof(HeapObject));
  obj->tag = TYPE_PAIR_TAG;
  obj->as_pair.car = car;
  obj->as_pair.cdr = cdr;
  return (void *)obj;
}

static inline void *box_symbol(const char *s) {
  HeapObject *obj = (HeapObject *)malloc(sizeof(HeapObject));
  obj->tag = TYPE_SYMBOL_TAG;
  obj->as_symbol = strdup(s);
  return (void *)obj;
}

static inline void print_value(void *v) {
  if (v == NULL_VAL) {
    printf("null");
  } else if (is_int(v)) {
    printf("%lld", val_to_int(v));
  } else if (is_bool(v)) {
    printf("%s", v == TRUE_VAL ? "#t" : "#f");
  } else if (is_char(v)) {
    printf("#\\%c", val_to_char(v));
  } else if (is_heap_ptr(v)) {
    HeapObject *obj = (HeapObject *)v;
    switch (obj->tag) {
    case TYPE_DOUBLE_TAG:
      printf("%f", obj->as_double);
      break;
    case TYPE_STRING_TAG:
      printf("\"%s\"", obj->as_string);
      break;
    case TYPE_PAIR_TAG:
      printf("(");
      print_value(obj->as_pair.car);
      printf(" . ");
      print_value(obj->as_pair.cdr);
      printf(")");
      break;
    case TYPE_SYMBOL_TAG:
      printf("'%s'", obj->as_symbol);
      break;
    default:
      printf("<unknown>");
    }
  } else {
    printf("<badval>");
  }
}

#endif