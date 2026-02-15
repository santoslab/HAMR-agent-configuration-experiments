// This file will not be overwritten if codegen is rerun

use data::*;
use crate::bridge::cons_cons_api::*;
use vstd::prelude::*;

verus! {

  pub struct cons_cons {
    // BEGIN MARKER STATE VARS
    pub payload_sum: i32,
    // END MARKER STATE VARS
  }

  impl cons_cons {
    pub fn new() -> Self
    {
      Self {
        // BEGIN MARKER STATE VAR INIT
        payload_sum: 0,
        // END MARKER STATE VAR INIT
      }
    }

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

    pub fn timeTriggered<API: cons_cons_Full_Api> (
      &mut self,
      api: &mut cons_cons_Application_Api<API>)
      requires
        // PLACEHOLDER MARKER TIME TRIGGERED REQUIRES
      ensures
        // PLACEHOLDER MARKER TIME TRIGGERED ENSURES
    {
      log_info("compute entrypoint invoked");
    }

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
