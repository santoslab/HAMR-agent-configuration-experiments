// This file will not be overwritten if codegen is rerun

use data::*;
use crate::bridge::test_sender_test_sender_api::*;
use vstd::prelude::*;

verus! {

  // Number of distinct test messages in the cycle
  const NUM_TEST_CASES: i32 = 7;

  pub struct test_sender_test_sender {
    // PLACEHOLDER MARKER STATE VARS

    // Counter to cycle through test messages
    pub test_case_index: i32,
  }

  impl test_sender_test_sender {
    pub fn new() -> Self
    {
      Self {
        // PLACEHOLDER MARKER STATE VAR INIT

        test_case_index: 0,
      }
    }

    pub fn initialize<API: test_sender_test_sender_Put_Api> (
      &mut self,
      api: &mut test_sender_test_sender_Application_Api<API>)
      ensures
        // PLACEHOLDER MARKER INITIALIZATION ENSURES
    {
      log_info("initialize entrypoint invoked");
      self.test_case_index = 0;
    }

    pub fn timeTriggered<API: test_sender_test_sender_Full_Api> (
      &mut self,
      api: &mut test_sender_test_sender_Application_Api<API>)
      requires
        // PLACEHOLDER MARKER TIME TRIGGERED REQUIRES
      ensures
        // PLACEHOLDER MARKER TIME TRIGGERED ENSURES
    {
      // Generate test messages that exercise all requirements:
      //   Case 0: Public,     payload=42   -> Gate passes, Filter passes unchanged
      //   Case 1: Restricted, payload=50   -> Gate passes, Filter passes unchanged (in range)
      //   Case 2: Critical,   payload=99   -> Gate DROPS
      //   Case 3: Restricted, payload=150  -> Gate passes, Filter clamps to 100
      //   Case 4: Public,     payload=0    -> Gate passes, Filter passes unchanged
      //   Case 5: Restricted, payload=-10  -> Gate passes, Filter clamps to 0
      //   Case 6: Critical,   payload=200  -> Gate DROPS

      let msg = build_test_message(self.test_case_index);
      api.put_output(msg);
      log_message_sent(self.test_case_index, msg);

      // advance to next test case, wrapping around
      self.test_case_index = self.test_case_index + 1;
      if self.test_case_index >= NUM_TEST_CASES {
        self.test_case_index = 0;
      }
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

  // Build a test message based on the test case index.
  // Uses if-else chain (rather than array) for Verus compatibility.
  #[verifier::external_body]
  pub fn build_test_message(index: i32) -> (res: SNG_Data_Model::Message)
  {
    if index == 0 {
      SNG_Data_Model::Message { security_level: SNG_Data_Model::SecurityLevel::Public, payload: 42 }
    } else if index == 1 {
      SNG_Data_Model::Message { security_level: SNG_Data_Model::SecurityLevel::Restricted, payload: 50 }
    } else if index == 2 {
      SNG_Data_Model::Message { security_level: SNG_Data_Model::SecurityLevel::Critical, payload: 99 }
    } else if index == 3 {
      SNG_Data_Model::Message { security_level: SNG_Data_Model::SecurityLevel::Restricted, payload: 150 }
    } else if index == 4 {
      SNG_Data_Model::Message { security_level: SNG_Data_Model::SecurityLevel::Public, payload: 0 }
    } else if index == 5 {
      SNG_Data_Model::Message { security_level: SNG_Data_Model::SecurityLevel::Restricted, payload: -10 }
    } else {
      SNG_Data_Model::Message { security_level: SNG_Data_Model::SecurityLevel::Critical, payload: 200 }
    }
  }

  #[verifier::external_body]
  pub fn log_info(msg: &str)
  {
    log::info!("{0}", msg);
  }

  #[verifier::external_body]
  pub fn log_message_sent(index: i32, msg: SNG_Data_Model::Message)
  {
    log::info!("TestSender: [case {0}] sent message (security_level={1:?}, payload={2})",
      index, msg.security_level, msg.payload);
  }

  #[verifier::external_body]
  pub fn log_warn_channel(channel: u32)
  {
    log::warn!("Unexpected channel: {0}", channel);
  }

  // PLACEHOLDER MARKER GUMBO METHODS

}
