/*============================================================
    SNG   P r o p e r t i e s

    Scheduling domain identifiers and other shared properties
    for the SNG system.
  ============================================================*/

package SNG_Properties {
    private import HAMR::*;

    // Scheduling domain identifiers
    // Ordered to support pipeline flow: sender -> gate -> filter -> receiver
    attribute Domain_TestSender : CASE_Scheduling::Domain = 2;
    attribute Domain_Gate : CASE_Scheduling::Domain = 3;
    attribute Domain_Filter : CASE_Scheduling::Domain = 4;
    attribute Domain_TestReceiver : CASE_Scheduling::Domain = 5;
}
