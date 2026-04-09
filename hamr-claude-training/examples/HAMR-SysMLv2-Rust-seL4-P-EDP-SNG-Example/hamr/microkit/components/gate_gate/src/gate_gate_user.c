#include "gate_gate.h"

// This file will not be overwritten if codegen is rerun

void gate_gate_initialize(void) {
  printf("%s: gate_gate_initialize invoked\n", microkit_name);
}

void gate_gate_timeTriggered(void) {
  printf("%s: gate_gate_timeTriggered invoked\n", microkit_name);
}

void gate_gate_notify(microkit_channel channel) {
  // this method is called when the monitor does not handle the passed in channel
  switch (channel) {
    default:
      printf("%s: Unexpected channel %d\n", microkit_name, channel);
  }
}
