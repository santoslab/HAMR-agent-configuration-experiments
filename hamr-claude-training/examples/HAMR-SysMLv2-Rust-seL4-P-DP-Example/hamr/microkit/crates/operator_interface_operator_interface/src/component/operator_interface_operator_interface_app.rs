// This file will not be overwritten if codegen is rerun

//============================================================================
//  O p e r a t o r    I n t e r f a c e   -- Thread Component
//
//  This Thread component implements a (simulated) operator interface
//  for the Isolette.  
//
//  It uses a simple notion of simulation similer to that of the Temp Sensor
//  thread to move the values of the set point outputs (desired temperature
//  range for the Isolette) up and down.
//
//============================================================================

use data::*;
use data::Isolette_Data_Model::*; // Add for easier reference to data types
use crate::bridge::operator_interface_operator_interface_api::*;
use vstd::prelude::*;

verus! {

  //-------------------------------------------
  //  Application State (as a struct)
  //
  //  The application state includes a non-GUMBO state variable 
  //  that supports the "simulation" of the operator interface.
  //
  //  There is no GUMBO declared application state for this component.
  //-------------------------------------------
  pub struct operator_interface_operator_interface {
    // PLACEHOLDER MARKER STATE VARS

    // The variables below are used for operator input (set points) simulation.
    // This also illustrates non-GUMBO declared state variable for a component
    pub lower_desired_temp: i32, 
    pub upper_desired_temp: i32,
    pub lower_desired_temp_trajectory: i32,
    pub upper_desired_temp_trajectory: i32,
    pub activations_until_update: i32,
  }

  impl operator_interface_operator_interface {
    //-------------------------------------------
    //  Application Component Constructor
    //-------------------------------------------
    pub fn new() -> Self
    {
      Self {
        // PLACEHOLDER MARKER STATE VAR INIT

        // initialization of non-GUMBO declared state variable
        // ...simulated operator input
        lower_desired_temp: 98, 
        upper_desired_temp: 101,
        lower_desired_temp_trajectory: -1, // value with either be +1 or -1
        upper_desired_temp_trajectory: 1, // value with either be +1 or -1
        activations_until_update: 5, // activations of compute entry point until send
      }
    }

    //-------------------------------------------
    //  Initialize Entry Point
    //-------------------------------------------
    pub fn initialize<API: operator_interface_operator_interface_Put_Api> (
      &mut self,
      api: &mut operator_interface_operator_interface_Application_Api<API>)
      ensures
        // PLACEHOLDER MARKER INITIALIZATION ENSURES
    {
      log_info("initialize entrypoint invoked");

      // initialize operator input simulation
      self.lower_desired_temp = 98;  // ToDo: switch to constant value
      self.upper_desired_temp = 101;  // ToDo: switch to constant value
      //  ...set the temperature trajectory to be increaing
      self.lower_desired_temp_trajectory = -1;
      self.upper_desired_temp_trajectory = 1;
      self.activations_until_update = 5;

      // put initial (default) desired temp on output port
      api.put_desired_temp(
        Set_Points { lower: Temp { degrees: self.lower_desired_temp}, 
                       upper: Temp { degrees: self.upper_desired_temp} }
      );
    }

    //-------------------------------------------
    //  Compute Entry Point
    //-------------------------------------------
    pub fn timeTriggered<API: operator_interface_operator_interface_Full_Api> (
      &mut self,
      api: &mut operator_interface_operator_interface_Application_Api<API>)
      requires
        // PLACEHOLDER MARKER TIME TRIGGERED REQUIRES
      ensures
        // PLACEHOLDER MARKER TIME TRIGGERED ENSURES
    {
      log_info("compute entrypoint invoked");

      // --------- Process operator's configuration of set points ------------

      // set point simulation
      if self.activations_until_update > 0 {
        // Case: Not updating set points yet...
        self.activations_until_update = self.activations_until_update - 1;
      } else {
        // Case: Updating set points

        //   ..update lower_desired_temp simulation according to trajectory
        self.lower_desired_temp = self.lower_desired_temp + self.lower_desired_temp_trajectory;

        //   ..update the simulation trajectory when temp reaches bounds
        if self.lower_desired_temp >= 99 {
          self.lower_desired_temp_trajectory = -1
        } else if self.lower_desired_temp <= 97 {
          self.lower_desired_temp_trajectory = 1
        };

        //   ..update upper_desired_temp simulation according to trajectory
        self.upper_desired_temp = self.upper_desired_temp + self.upper_desired_temp_trajectory;

        //   ..update the simulation trajectory when temp reaches bounds
        if self.upper_desired_temp >= 102 {
          self.upper_desired_temp_trajectory = -1
        } else if self.upper_desired_temp <= 99 {
          self.upper_desired_temp = 1
        };

        // reset activation count
        self.activations_until_update = 5;
      }
      log_set_point_simulation(
        self.lower_desired_temp,
        self.upper_desired_temp,
        self.lower_desired_temp_trajectory,
        self.upper_desired_temp_trajectory,
        self.activations_until_update,
      );

      // build set points struct
      let set_points = 
        Set_Points { lower: Temp { degrees: self.lower_desired_temp}, 
                     upper: Temp { degrees: self.upper_desired_temp} };
      api.put_desired_temp(set_points);
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
  pub fn log_set_point_simulation(
    lower_desired_temp: i32,
    upper_desired_temp: i32,
    lower_desired_temp_trajectory: i32,
    upper_desired_temp_trajectory: i32,
    activations_until_update: i32,
  )
  {
     log::info!("LDT: {}", lower_desired_temp);
     log::info!("UDT: {}", upper_desired_temp);
     log::info!("LDT Trajectory: {}", lower_desired_temp_trajectory);
     log::info!("UDT Trajectory: {}", upper_desired_temp_trajectory);
     log::info!("Activations until update: {}", activations_until_update);
  }

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

  // PLACEHOLDER MARKER GUMBO METHODS

}
