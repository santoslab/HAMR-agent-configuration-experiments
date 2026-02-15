// This file will not be overwritten if codegen is rerun

//============================================================================
//  H e a t   S o u r c e   -- Thread Component
//
//  This Thread component implements a (simulated) heater for the air 
//  in the Isolette.  It takes as input commands indicating the desired
//  state of the heater (on or off).
//
//  There is no software application level output for this component.  
//  The component encapsulates a hardware driver that interacts with simulated hardware 
//  interface.  The (very simple!) simulation just holds the current 
//  state of the heater.
//
//============================================================================

use data::*;
use data::Isolette_Data_Model::*;
use crate::bridge::heat_source_heat_source_api::*;
use vstd::prelude::*;

verus! {

  //-------------------------------------------
  //  Application State (as a struct)
  //
  //  The application state includes a non-GUMBO state variable 
  //  that supports the "simulation" of the heater (i.e., an 
  //  indication of the state of the heater (on/off)).
  //
  //  There is no GUMBO declared application state for this component.
  //-------------------------------------------
  pub struct heat_source_heat_source {
    // PLACEHOLDER MARKER STATE VARS

    // local state used in simulation to hold the state of the heater.
    pub heater_state: On_Off, 
  }

  
  impl heat_source_heat_source {
    //-------------------------------------------
    //  Application Component Constructor
    //-------------------------------------------
    pub fn new() -> Self
    {
      Self {
        // PLACEHOLDER MARKER STATE VAR INIT

        heater_state: On_Off::Off,
      }
    }

    //-------------------------------------------
    //  Initialize Entry Point
    //-------------------------------------------
    pub fn initialize<API: heat_source_heat_source_Put_Api> (
      &mut self,
      api: &mut heat_source_heat_source_Application_Api<API>)
      ensures
        // PLACEHOLDER MARKER INITIALIZATION ENSURES
    {
      log_info("initialize entrypoint invoked");

      // initialize simulation
      self.heater_state = On_Off::Off;
    }

    //-------------------------------------------
    //  Compute Entry Point
    //-------------------------------------------
    pub fn timeTriggered<API: heat_source_heat_source_Full_Api> (
      &mut self,
      api: &mut heat_source_heat_source_Application_Api<API>)
      requires
        // PLACEHOLDER MARKER TIME TRIGGERED REQUIRES
      ensures
        // PLACEHOLDER MARKER TIME TRIGGERED ENSURES
    {
      log_info("compute entrypoint invoked");

       // get values of input ports 
      let heat_control: On_Off = api.get_heat_control();
    
      // update simulation
      self.heater_state = heat_control;

      // log state of temperature simulation
      log_heat_source_simulation(&self);
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

  pub fn log_heat_source_simulation(state: &heat_source_heat_source)
  {
    log::info!("Heater State: {:?}", state.heater_state);
  }

  // PLACEHOLDER MARKER GUMBO METHODS

}
