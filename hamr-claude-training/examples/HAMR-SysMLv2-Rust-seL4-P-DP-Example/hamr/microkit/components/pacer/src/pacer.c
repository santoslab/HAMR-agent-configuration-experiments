#include <stdint.h>
#include <microkit.h>

#define PORT_TO_TEMP_SENSOR_TEMP_SENSOR_MON 61
#define PORT_TO_OPERATOR_INTERFACE_OPERATOR_INTERFACE_MON 59
#define PORT_TO_THERMOSTAT_THERMOSTAT_MON 57
#define PORT_TO_HEAT_SOURCE_HEAT_SOURCE_MON 55

void init(void) {}

void notified(microkit_channel channel) {
  switch(channel) {
    case PORT_TO_TEMP_SENSOR_TEMP_SENSOR_MON:
      microkit_notify(PORT_TO_TEMP_SENSOR_TEMP_SENSOR_MON);
      break;
    case PORT_TO_OPERATOR_INTERFACE_OPERATOR_INTERFACE_MON:
      microkit_notify(PORT_TO_OPERATOR_INTERFACE_OPERATOR_INTERFACE_MON);
      break;
    case PORT_TO_THERMOSTAT_THERMOSTAT_MON:
      microkit_notify(PORT_TO_THERMOSTAT_THERMOSTAT_MON);
      break;
    case PORT_TO_HEAT_SOURCE_HEAT_SOURCE_MON:
      microkit_notify(PORT_TO_HEAT_SOURCE_HEAT_SOURCE_MON);
      break;
  }
}
