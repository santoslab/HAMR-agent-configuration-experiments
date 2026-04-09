#include "test_receiver_test_receiver.h"

// This file will not be overwritten if codegen is rerun

void test_receiver_test_receiver_initialize(void) {
  printf("%s: test_receiver_test_receiver_initialize invoked\n", microkit_name);
}

void test_receiver_test_receiver_timeTriggered(void) {
  printf("%s: test_receiver_test_receiver_timeTriggered invoked\n", microkit_name);
}

void test_receiver_test_receiver_notify(microkit_channel channel) {
  // this method is called when the monitor does not handle the passed in channel
  switch (channel) {
    default:
      printf("%s: Unexpected channel %d\n", microkit_name, channel);
  }
}
