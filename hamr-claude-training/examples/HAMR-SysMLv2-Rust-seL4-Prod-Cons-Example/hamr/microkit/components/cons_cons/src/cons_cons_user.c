#include "cons_cons.h"

// This file will not be overwritten if codegen is rerun

void cons_cons_initialize(void) {
  printf("%s: cons_cons_initialize invoked\n", microkit_name);
}

void cons_cons_timeTriggered(void) {
  printf("%s: cons_cons_timeTriggered invoked\n", microkit_name);
}

void cons_cons_notify(microkit_channel channel) {
  // this method is called when the monitor does not handle the passed in channel
  switch (channel) {
    default:
      printf("%s: Unexpected channel %d\n", microkit_name, channel);
  }
}
