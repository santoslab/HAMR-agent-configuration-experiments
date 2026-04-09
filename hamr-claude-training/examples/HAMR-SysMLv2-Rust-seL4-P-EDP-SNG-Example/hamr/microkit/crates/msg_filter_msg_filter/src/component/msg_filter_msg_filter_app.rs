// This file will not be overwritten if codegen is rerun

//============================================================================
//  F i l t e r  -- Thread Component
//
//  Guard pipeline stage 2: sanitizes payloads for non-Critical messages.
//  Public messages pass unchanged; Restricted message payloads are
//  clamped to [0, 100].  Critical messages never arrive here (dropped
//  by Gate upstream, enforced by GUMBO integration assume).
//
//  Data flow: TestSender -> Gate -> [Filter] -> TestReceiver
//============================================================================

use data::*;
use crate::bridge::msg_filter_msg_filter_api::*;
use vstd::prelude::*;

verus! {

  //-------------------------------------------
  //  Application State (as a struct)
  //-------------------------------------------
  pub struct msg_filter_msg_filter {
    // PLACEHOLDER MARKER STATE VARS
  }

  impl msg_filter_msg_filter {
    //-------------------------------------------
    //  Application Component Constructor
    //-------------------------------------------
    pub fn new() -> Self
    {
      Self {
        // PLACEHOLDER MARKER STATE VAR INIT
      }
    }

    //-------------------------------------------
    //  Initialize Entry Point
    //-------------------------------------------
    pub fn initialize<API: msg_filter_msg_filter_Put_Api> (
      &mut self,
      api: &mut msg_filter_msg_filter_Application_Api<API>)
      ensures
        // PLACEHOLDER MARKER INITIALIZATION ENSURES
    {
      log_info("initialize entrypoint invoked");
      // No output ports to initialize (EventDataPort does not require initialization)
    }

    //-------------------------------------------
    //  Compute Entry Point
    //-------------------------------------------
    pub fn timeTriggered<API: msg_filter_msg_filter_Full_Api> (
      &mut self,
      api: &mut msg_filter_msg_filter_Application_Api<API>)
      requires
        // BEGIN MARKER TIME TRIGGERED REQUIRES
        // assume AADL_Requirement
        //   All outgoing event ports must be empty
        old(api).output.is_none(),
        // END MARKER TIME TRIGGERED REQUIRES
      ensures
        // BEGIN MARKER TIME TRIGGERED ENSURES
        // case Req_P_Public_Pass
        (old(api).input.is_some() &&
          (old(api).input.unwrap().security_level == SNG_Data_Model::SecurityLevel::Public)) ==>
          (api.output.is_some() && GumboLib::equalMessage_spec(api.input.unwrap(), api.output.unwrap())),
        // case Req_R2_Restricted_Clamp
        (old(api).input.is_some() &&
          (old(api).input.unwrap().security_level == SNG_Data_Model::SecurityLevel::Restricted)) ==>
          (api.output.is_some() &&
             (GumboLib::equalSecurityLevel_spec(api.input.unwrap(), api.output.unwrap()) &&
               (GumboLib::clampedPayload_spec(api.output.unwrap()) &&
                 (GumboLib::clampedPayload_spec(api.input.unwrap()) ==> GumboLib::equalPayload_spec(api.input.unwrap(), api.output.unwrap()))))),
        // case No_Input
        (!(old(api).input.is_some())) ==>
          (api.output.is_none()),
        // END MARKER TIME TRIGGERED ENSURES
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

  //-------------------------------------------
  //  GUMBO-derived functions/constants auto-generated by HAMR from
  //  model.
  //-------------------------------------------

  // PLACEHOLDER MARKER GUMBO METHODS

}
