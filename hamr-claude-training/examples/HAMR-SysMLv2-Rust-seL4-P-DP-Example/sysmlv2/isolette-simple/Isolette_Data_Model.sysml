/*============================================================
    Isolette   D a t a   M o d e l

    Defines types used for inter-component communication using
    structures conforming to the data modeling portion of the 
    AADL libraries.  
    
    Notes: although this approach for representating data types
    is probably not the best from a native SysMLv2 perspective,
    we take this approach to be compliant with the OMG RTESC
    committee work.
  ============================================================*/
 
package Isolette_Data_Model {
    private import HAMR::*;

    part def Temp :> Data {
        part degrees : Base_Types::Integer_32;
    }

    part def Set_Points :> Data {
        part lower : Temp;
        part upper : Temp;
    }

    enum def On_Off {
        enum Onn;
        enum Off;
    }
 
}