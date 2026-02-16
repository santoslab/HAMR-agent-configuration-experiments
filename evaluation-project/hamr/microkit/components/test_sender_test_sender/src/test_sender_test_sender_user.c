#include "test_sender_test_sender.h"

// This file will not be overwritten if codegen is rerun

void test_sender_test_sender_initialize(void) {
  printf("%s: test_sender_test_sender_initialize invoked\n", microkit_name);
}

void test_sender_test_sender_timeTriggered(void) {
  printf("%s: test_sender_test_sender_timeTriggered invoked\n", microkit_name);
}

void test_sender_test_sender_notify(microkit_channel channel) {
  // this method is called when the monitor does not handle the passed in channel
  switch (channel) {
    default:
      printf("%s: Unexpected channel %d\n", microkit_name, channel);
  }
}
