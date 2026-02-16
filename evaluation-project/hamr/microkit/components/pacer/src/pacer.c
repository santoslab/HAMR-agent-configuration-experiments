#include <stdint.h>
#include <microkit.h>

#define PORT_TO_GATE_GATE_MON 61
#define PORT_TO_MSG_FILTER_MSG_FILTER_MON 59
#define PORT_TO_TEST_SENDER_TEST_SENDER_MON 57
#define PORT_TO_TEST_RECEIVER_TEST_RECEIVER_MON 55

void init(void) {}

void notified(microkit_channel channel) {
  switch(channel) {
    case PORT_TO_GATE_GATE_MON:
      microkit_notify(PORT_TO_GATE_GATE_MON);
      break;
    case PORT_TO_MSG_FILTER_MSG_FILTER_MON:
      microkit_notify(PORT_TO_MSG_FILTER_MSG_FILTER_MON);
      break;
    case PORT_TO_TEST_SENDER_TEST_SENDER_MON:
      microkit_notify(PORT_TO_TEST_SENDER_TEST_SENDER_MON);
      break;
    case PORT_TO_TEST_RECEIVER_TEST_RECEIVER_MON:
      microkit_notify(PORT_TO_TEST_RECEIVER_TEST_RECEIVER_MON);
      break;
  }
}
