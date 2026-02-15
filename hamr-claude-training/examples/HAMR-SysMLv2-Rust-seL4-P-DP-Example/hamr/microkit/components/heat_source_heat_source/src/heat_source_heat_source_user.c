#include "heat_source_heat_source.h"

// This file will not be overwritten if codegen is rerun

void heat_source_heat_source_initialize(void) {
  printf("%s: heat_source_heat_source_initialize invoked\n", microkit_name);
}

void heat_source_heat_source_timeTriggered(void) {
  printf("%s: heat_source_heat_source_timeTriggered invoked\n", microkit_name);
}

void heat_source_heat_source_notify(microkit_channel channel) {
  // this method is called when the monitor does not handle the passed in channel
  switch (channel) {
    default:
      printf("%s: Unexpected channel %d\n", microkit_name, channel);
  }
}
