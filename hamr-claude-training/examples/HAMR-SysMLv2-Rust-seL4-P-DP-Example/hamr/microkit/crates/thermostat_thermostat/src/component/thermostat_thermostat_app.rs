// This file will not be overwritten if codegen is rerun

//============================================================================
//  T h e r m o s t a t  -- Thread Component
//
//  This Thread component implements control logic that 
//  turns a heat source on/off based on the values of the following inputs...
//    - current temperature
//    - set points provided by an operator 
//        (these specify the lower and upper bounds (inclusive) for the 
//        desired temperature range.
//
//============================================================================

use data::*;
use data::Isolette_Data_Model::*; // Add for easier reference of data types
use crate::bridge::thermostat_thermostat_api::*;
use vstd::prelude::*;

verus! {

  //-------------------------------------------
  //  Application State (as a struct)
  //
  //  The application state includes a model-level GUMBO declared state variable
  //  `lastCmd` that is visible to the model-level contract for the thread.
  //-------------------------------------------
  pub struct thermostat_thermostat {
    // BEGIN MARKER STATE VARS
    pub lastCmd: Isolette_Data_Model::On_Off,
    // END MARKER STATE VARS
  }

  impl thermostat_thermostat {
    //-------------------------------------------
    //  Application Component Constructor
    //-------------------------------------------
    pub fn new() -> Self
    {
      Self {
        // BEGIN MARKER STATE VAR INIT
        lastCmd: Isolette_Data_Model::On_Off::default(),
        // END MARKER STATE VAR INIT
      }
    }

    //-------------------------------------------
    //  Initialize Entry Point
    //-------------------------------------------
    pub fn initialize<API: thermostat_thermostat_Put_Api> (
      &mut self,
      api: &mut thermostat_thermostat_Application_Api<API>)
      ensures
        // BEGIN MARKER INITIALIZATION ENSURES
        // guarantee initlastCmd
        self.lastCmd == Isolette_Data_Model::On_Off::Off,
        // guarantee REQ_THERM_1
        //   The Heat Control command shall be Off initially.
        api.heat_control == Isolette_Data_Model::On_Off::Off,
        // END MARKER INITIALIZATION ENSURES
    {
      log_info("initialize entrypoint invoked");

      self.lastCmd = On_Off::Off;
      // REQ_THERM_1: The Heat Control shall be initially Off
      let currentCmd = On_Off::Off;
      api.put_heat_control(currentCmd) 
    }

    //-------------------------------------------
    //  Compute Entry Point
    //-------------------------------------------
    pub fn timeTriggered<API: thermostat_thermostat_Full_Api> (
      &mut self,
      api: &mut thermostat_thermostat_Application_Api<API>)
      requires
        // BEGIN MARKER TIME TRIGGERED REQUIRES
        // assume ASSM_LDT_LE_UDT
        old(api).desired_temp.lower.degrees <= old(api).desired_temp.upper.degrees,
        // END MARKER TIME TRIGGERED REQUIRES
      ensures
        // BEGIN MARKER TIME TRIGGERED ENSURES
        // guarantee lastCmd
        //   Set lastCmd to value of output Cmd port
        self.lastCmd == api.heat_control,
        // case REQ_THERM_2
        //   If Current Temperature is less than
        //   the Lower Desired Temperature, the Heat Control shall be set to On.
        (old(api).current_temp.degrees < old(api).desired_temp.lower.degrees) ==>
          (api.heat_control == Isolette_Data_Model::On_Off::Onn),
        // case REQ_THERM_3
        //   If the Current Temperature is greater than
        //   the Upper Desired Temperature, the Heat Control shall be set to Off.
        (old(api).current_temp.degrees > old(api).desired_temp.upper.degrees) ==>
          (api.heat_control == Isolette_Data_Model::On_Off::Off),
        // case REQ_THERM_4
        //   If the Current Temperature is greater than or equal 
        //   to the Lower Desired Temperature
        //   and less than or equal to the Upper Desired Temperature, the value of
        //   the Heat Control shall not be changed.
        ((old(api).current_temp.degrees >= old(api).desired_temp.lower.degrees) &&
          (old(api).current_temp.degrees <= old(api).desired_temp.upper.degrees)) ==>
          (api.heat_control == old(self).lastCmd),
        // END MARKER TIME TRIGGERED ENSURES
    {
      log_info("compute entrypoint invoked");

       // -------------- Get values of input ports ------------------
      let desired_temp: Set_Points = api.get_desired_temp(); 
      let currentTemp: Temp = api.get_current_temp();

      //================ compute / control logic ===========================

      // current command defaults to value of last command (REQ-THERM-4)
      let mut currentCmd: On_Off = self.lastCmd;

      if (currentTemp.degrees > desired_temp.upper.degrees) {
         // REQ-THERM-3
         currentCmd = On_Off::Off;
      } else if (currentTemp.degrees < desired_temp.lower.degrees) {
         assert(api.current_temp.degrees < api.desired_temp.lower.degrees);
         // REQ-THERM-2
         //currentCmd = On_Off::Off; // seeded bug/error
         currentCmd = On_Off::Onn;
      }

      // -------------- Set values of output ports ------------------
      api.put_heat_control(currentCmd);
      self.lastCmd = currentCmd        

    }

    //-------------------------------------------
    //  seL4 / Microkit Error Handling
    //-------------------------------------------
    pub fn notify(
      &mut self,
      channel: microkit_channel)
    {
      // this method is called when the monitor does not handle the passed in channel
      match channel {
        _ => {
          log_warn_channel(channel)
        }
      }
    }
  }

  //-------------------------------------------
  //  Logging Helper Functions
  //-------------------------------------------
  #[verifier::external_body]
  pub fn log_info(msg: &str)
  {
    log::info!("{0}", msg);
  }

  #[verifier::external_body]
  pub fn log_warn_channel(channel: u32)
  {
    log::warn!("Unexpected channel: {0}", channel);
  }

  //-------------------------------------------
  //  GUMBO-derived functions/constants auto-generated by HAMR from 
  //  model.
  //-------------------------------------------

  // BEGIN MARKER GUMBO METHODS
  pub open spec fn Temp_Lower_Bound() -> i32
  {
    95i32
  }

  pub open spec fn Temp_Upper_Bound() -> i32
  {
    104i32
  }
  // END MARKER GUMBO METHODS

}
