/*============================================================
    D e v i c e s

    Defines the software components (threads), intuitively 
    corresponding to components that encapsulate drivers 
    interacting with lower-level hardware components of the 
    system.

    In a full development context, where we want to emphasize
    managed collections of reusable resources, each physical 
    device might be accompanied by both software (as illustrated here)
    and hardware oriented modeling elements (not shown here).
    We might have a separate package for each physical device
    or families of physical devices.

  ============================================================*/

package Devices {
  private import HAMR::*;

  //===============================================================================
  // 
  //  T e m p S e n s o r   Thread 
  // 
  //===============================================================================
  part def TempSensor :> Thread {
    //-- I n t e r f a c e  --
    out port current_temp : DataPort { :>> type : Isolette_Data_Model::Temp; }

    //-- P r o p e r t i e s  --
    attribute :>> Period = 1000 [ms];
    attribute :>> Dispatch_Protocol = Supported_Dispatch_Protocols::Periodic;
    attribute :>> Implementation_Language = Implementation_Languages::Rust;

    //-- B e h a v i o r    C o n s t r a i n t s --
    language "GUMBO" /*{
       integration
          guarantee temp_range:
              96 [i32] <= current_temp.degrees & current_temp.degrees <= 103 [i32];
    }*/
  }

  //===============================================================================
  // 
  //  H e a t S o u r c e   Thread 
  // 
  //===============================================================================
  part def HeatSource :> Thread {
    //-- I n t e r f a c e  --
    in port heat_control : DataPort { :>> type : Isolette_Data_Model::On_Off; }

    //-- P r o p e r t i e s  --
    attribute :>> Period = 1000 [ms];
    attribute :>> Dispatch_Protocol = Supported_Dispatch_Protocols::Periodic;
    attribute :>> Implementation_Language = Implementation_Languages::Rust;
  }
}
