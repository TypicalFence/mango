#include <stdio.h>
#include <stdint.h>

extern void * new_mango_file(char *);
extern char * mangofile_get_name(void *);

int main(void) {
  void *file;
  char *name;
  file = new_mango_file("oni-chan Daisuki");
  name = mangofile_get_name(file);
  printf(name);
}
