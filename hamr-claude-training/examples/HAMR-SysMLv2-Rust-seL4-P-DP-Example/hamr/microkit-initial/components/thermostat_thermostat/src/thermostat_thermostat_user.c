#include "thermostat_thermostat.h"

// This file will not be overwritten if codegen is rerun

void thermostat_thermostat_initialize(void) {
  printf("%s: thermostat_thermostat_initialize invoked\n", microkit_name);
}

void thermostat_thermostat_timeTriggered(void) {
  printf("%s: thermostat_thermostat_timeTriggered invoked\n", microkit_name);
}

void thermostat_thermostat_notify(microkit_channel channel) {
  // this method is called when the monitor does not handle the passed in channel
  switch (channel) {
    default:
      printf("%s: Unexpected channel %d\n", microkit_name, channel);
  }
}
