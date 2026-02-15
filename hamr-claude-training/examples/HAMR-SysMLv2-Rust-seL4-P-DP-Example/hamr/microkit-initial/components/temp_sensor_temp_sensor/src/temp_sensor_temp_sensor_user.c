#include "temp_sensor_temp_sensor.h"

// This file will not be overwritten if codegen is rerun

void temp_sensor_temp_sensor_initialize(void) {
  printf("%s: temp_sensor_temp_sensor_initialize invoked\n", microkit_name);
}

void temp_sensor_temp_sensor_timeTriggered(void) {
  printf("%s: temp_sensor_temp_sensor_timeTriggered invoked\n", microkit_name);
}

void temp_sensor_temp_sensor_notify(microkit_channel channel) {
  // this method is called when the monitor does not handle the passed in channel
  switch (channel) {
    default:
      printf("%s: Unexpected channel %d\n", microkit_name, channel);
  }
}
