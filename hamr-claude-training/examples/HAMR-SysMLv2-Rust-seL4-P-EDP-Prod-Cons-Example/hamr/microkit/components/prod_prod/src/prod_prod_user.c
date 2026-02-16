#include "prod_prod.h"

// This file will not be overwritten if codegen is rerun

void prod_prod_initialize(void) {
  printf("%s: prod_prod_initialize invoked\n", microkit_name);
}

void prod_prod_timeTriggered(void) {
  printf("%s: prod_prod_timeTriggered invoked\n", microkit_name);
}

void prod_prod_notify(microkit_channel channel) {
  // this method is called when the monitor does not handle the passed in channel
  switch (channel) {
    default:
      printf("%s: Unexpected channel %d\n", microkit_name, channel);
  }
}
