#include "operator_interface_operator_interface.h"

// This file will not be overwritten if codegen is rerun

void operator_interface_operator_interface_initialize(void) {
  printf("%s: operator_interface_operator_interface_initialize invoked\n", microkit_name);
}

void operator_interface_operator_interface_timeTriggered(void) {
  printf("%s: operator_interface_operator_interface_timeTriggered invoked\n", microkit_name);
}

void operator_interface_operator_interface_notify(microkit_channel channel) {
  // this method is called when the monitor does not handle the passed in channel
  switch (channel) {
    default:
      printf("%s: Unexpected channel %d\n", microkit_name, channel);
  }
}
