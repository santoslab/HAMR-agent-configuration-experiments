/*============================================================
    SNG   D a t a   M o d e l

    Defines types used for inter-component communication in
    the Simple Network Guard system.
  ============================================================*/

package SNG_Data_Model {
    private import HAMR::*;

    // Security classification for messages
    enum def SecurityLevel {
        enum Public;
        enum Secret;
        enum TopSecret;
    }

    // Message structure processed by the guard
    part def Message :> Data {
        attribute security_level : SecurityLevel;
        part payload : Base_Types::Integer_32;
    }
}
