#include "msg_filter_msg_filter.h"

// This file will not be overwritten if codegen is rerun

void msg_filter_msg_filter_initialize(void) {
  printf("%s: msg_filter_msg_filter_initialize invoked\n", microkit_name);
}

void msg_filter_msg_filter_timeTriggered(void) {
  printf("%s: msg_filter_msg_filter_timeTriggered invoked\n", microkit_name);
}

void msg_filter_msg_filter_notify(microkit_channel channel) {
  // this method is called when the monitor does not handle the passed in channel
  switch (channel) {
    default:
      printf("%s: Unexpected channel %d\n", microkit_name, channel);
  }
}
