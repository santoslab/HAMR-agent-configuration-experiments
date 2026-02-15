#include <stdint.h>
#include <microkit.h>

#define PORT_TO_PROD_PROD_MON 61
#define PORT_TO_CONS_CONS_MON 59

void init(void) {}

void notified(microkit_channel channel) {
  switch(channel) {
    case PORT_TO_PROD_PROD_MON:
      microkit_notify(PORT_TO_PROD_PROD_MON);
      break;
    case PORT_TO_CONS_CONS_MON:
      microkit_notify(PORT_TO_CONS_CONS_MON);
      break;
  }
}
