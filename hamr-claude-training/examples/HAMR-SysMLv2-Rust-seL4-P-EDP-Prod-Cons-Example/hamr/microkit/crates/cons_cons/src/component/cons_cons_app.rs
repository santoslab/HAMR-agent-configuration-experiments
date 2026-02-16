// This file will not be overwritten if codegen is rerun

use data::*;
use data::ProdCons::*;
use crate::bridge::cons_cons_api::*;
use vstd::prelude::*;

verus! {

  //-------------------------------------------
  //  Application State (as a struct)
  //-------------------------------------------
  pub struct cons_cons {
    // BEGIN MARKER STATE VARS
    pub payload_sum: i32,
    // END MARKER STATE VARS
  }

  impl cons_cons {
    //-------------------------------------------
    //  Application Component Constructor
    //-------------------------------------------
    pub fn new() -> Self
    {
      Self {
        // BEGIN MARKER STATE VAR INIT
        payload_sum: 0,
        // END MARKER STATE VAR INIT
      }
    }

    //-------------------------------------------
    //  Initialize Entry Point
    //-------------------------------------------
    pub fn initialize<API: cons_cons_Put_Api> (
      &mut self,
      api: &mut cons_cons_Application_Api<API>)
      ensures
        // BEGIN MARKER INITIALIZATION ENSURES
        // guarantee initSum
        self.payload_sum == Init_Payload_Sum(),
        // END MARKER INITIALIZATION ENSURES
    {
      log_info("initialize entrypoint invoked");
    }

    //-------------------------------------------
    //  Compute Entry Point
    //-------------------------------------------
    pub fn timeTriggered<API: cons_cons_Full_Api> (
      &mut self,
      api: &mut cons_cons_Application_Api<API>)
      requires
        // PLACEHOLDER MARKER TIME TRIGGERED REQUIRES
      ensures
        // PLACEHOLDER MARKER TIME TRIGGERED ENSURES
    {
      log_info("compute entrypoint invoked");

      let input_contents = api.get_input();
      match input_contents {
        Some(m) =>  { // message is present on port
            self.payload_sum = self.payload_sum + m.payload;
            log_message_received(m, self.payload_sum);
          }
        None => {  // no message present on port
            log_info("Cons:No message received");
          }
      };
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
  pub fn log_message_received(m: Message, payload_sum: i32)
  {
    log::info!("Cons: Message received: Payload={0} Control_Num={1} Payload Sum={2}", 
       m.payload, m.control_num, payload_sum);
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

  // BEGIN MARKER GUMBO METHODS
  pub open spec fn Init_Payload_Sum() -> i32
  {
    0i32
  }
  // END MARKER GUMBO METHODS

}
