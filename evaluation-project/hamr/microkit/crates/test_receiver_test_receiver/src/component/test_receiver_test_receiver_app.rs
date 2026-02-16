// This file will not be overwritten if codegen is rerun

use data::*;
use crate::bridge::test_receiver_test_receiver_api::*;
use vstd::prelude::*;

verus! {

  pub struct test_receiver_test_receiver {
    // PLACEHOLDER MARKER STATE VARS

    // Counter of messages received
    pub num_received: i32,
  }

  impl test_receiver_test_receiver {
    pub fn new() -> Self
    {
      Self {
        // PLACEHOLDER MARKER STATE VAR INIT

        num_received: 0,
      }
    }

    pub fn initialize<API: test_receiver_test_receiver_Put_Api> (
      &mut self,
      api: &mut test_receiver_test_receiver_Application_Api<API>)
      ensures
        // PLACEHOLDER MARKER INITIALIZATION ENSURES
    {
      log_info("initialize entrypoint invoked");
      self.num_received = 0;
    }

    pub fn timeTriggered<API: test_receiver_test_receiver_Full_Api> (
      &mut self,
      api: &mut test_receiver_test_receiver_Application_Api<API>)
      requires
        // PLACEHOLDER MARKER TIME TRIGGERED REQUIRES
      ensures
        // PLACEHOLDER MARKER TIME TRIGGERED ENSURES
    {
      let input_contents = api.get_input();
      match input_contents {
        Some(msg) => {
          self.num_received = self.num_received + 1;
          log_message_received(self.num_received, msg);
        }
        None => {
          // no message present on input port
        }
      };
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
  pub fn log_message_received(count: i32, msg: SNG_Data_Model::Message)
  {
    log::info!("TestReceiver: [msg #{0}] received (security_level={1:?}, payload={2})",
      count, msg.security_level, msg.payload);
  }

  #[verifier::external_body]
  pub fn log_warn_channel(channel: u32)
  {
    log::warn!("Unexpected channel: {0}", channel);
  }

  // PLACEHOLDER MARKER GUMBO METHODS

}
