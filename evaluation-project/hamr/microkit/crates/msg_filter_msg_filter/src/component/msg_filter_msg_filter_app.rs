// This file will not be overwritten if codegen is rerun

use data::*;
use crate::bridge::msg_filter_msg_filter_api::*;
use vstd::prelude::*;

verus! {

  pub struct msg_filter_msg_filter {
    // PLACEHOLDER MARKER STATE VARS
  }

  impl msg_filter_msg_filter {
    pub fn new() -> Self
    {
      Self {
        // PLACEHOLDER MARKER STATE VAR INIT
      }
    }

    pub fn initialize<API: msg_filter_msg_filter_Put_Api> (
      &mut self,
      api: &mut msg_filter_msg_filter_Application_Api<API>)
      ensures
        // PLACEHOLDER MARKER INITIALIZATION ENSURES
    {
      log_info("initialize entrypoint invoked");
      // No output ports to initialize (EventDataPort does not require initialization)
    }

    pub fn timeTriggered<API: msg_filter_msg_filter_Full_Api> (
      &mut self,
      api: &mut msg_filter_msg_filter_Application_Api<API>)
      requires
        // PLACEHOLDER MARKER TIME TRIGGERED REQUIRES
      ensures
        // PLACEHOLDER MARKER TIME TRIGGERED ENSURES
    {
      // Filter implements payload sanitization:
      //   Req_P: Public messages pass unchanged
      //   Req_R_2: Restricted message payloads are clamped to [0,100]
      // Note: Critical messages never arrive here (guaranteed by Gate upstream,
      //       enforced by GUMBO integration assume No_Critical_Input)

      let input_contents = api.get_input();
      match input_contents {
        Some(msg) => {
          match msg.security_level {
            SNG_Data_Model::SecurityLevel::Public => {
              // Req_P: pass Public messages unchanged
              api.put_output(msg);
              log_message_passed(msg);
            }
            _ => {
              // Restricted messages: clamp payload to [0, 100]
              let clamped_payload: i32;
              if msg.payload > 100 {
                // Req_R_2a: payload > 100 clamped to 100
                clamped_payload = 100;
              } else if msg.payload < 0 {
                // Req_R_2b: payload < 0 clamped to 0
                clamped_payload = 0;
              } else {
                // Req_R_2c: payload in [0,100] unchanged
                clamped_payload = msg.payload;
              }
              let output_msg = SNG_Data_Model::Message {
                security_level: msg.security_level,
                payload: clamped_payload,
              };
              api.put_output(output_msg);
              log_message_filtered(msg, output_msg);
            }
          }
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
  pub fn log_message_passed(msg: SNG_Data_Model::Message)
  {
    log::info!("Filter: PASSED Public message unchanged (payload={0})",
      msg.payload);
  }

  #[verifier::external_body]
  pub fn log_message_filtered(input: SNG_Data_Model::Message, output: SNG_Data_Model::Message)
  {
    log::info!("Filter: Restricted message filtered (payload: {0} -> {1})",
      input.payload, output.payload);
  }

  #[verifier::external_body]
  pub fn log_warn_channel(channel: u32)
  {
    log::warn!("Unexpected channel: {0}", channel);
  }

  // PLACEHOLDER MARKER GUMBO METHODS

}
