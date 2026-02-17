// This file will not be overwritten if codegen is rerun

//================================================================
//  Gate Component Tests
//
//  The Gate implements message drop/pass policies:
//    Req_C:   Critical messages are dropped (no output)
//    Req_R_1: Restricted messages are passed through unchanged
//    Req_P:   Public messages are passed through unchanged
//
//  Three styles of testing are illustrated:
//    1. Manual unit tests - directly verify inputs/outputs
//    2. Manual GUMBOX tests - use contract-based harness
//    3. Automated GUMBOX tests - PropTest with random generation
//================================================================

mod tests {
  // NOTE: need to run tests sequentially to prevent race conditions
  //       on the app and the testing apis which are static
  use serial_test::serial;

  use crate::test::util::*;
  use data::*;

  #[test]
  #[serial]
  fn test_initialization() {
    crate::gate_gate_initialize();

    // After initialization, output port should have no value
    // (EventDataPort does not require initialization)
    let output = test_apis::get_output();
    assert!(output.is_none());
  }

  //========================================================================
  //  Helper: initialize, set input, run compute, return output
  //========================================================================
  fn run_gate(input: Option<SNG_Data_Model::Message>) -> Option<SNG_Data_Model::Message>
  {
    crate::gate_gate_initialize();
    test_apis::put_input(input);
    crate::gate_gate_timeTriggered();
    test_apis::get_output()
  }

  //========================================================================
  //  Req_C: Critical messages are dropped
  //========================================================================

  #[test]
  #[serial]
  fn test_Req_C_drop_critical() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Critical,
      payload: 42,
    };
    let output = run_gate(Some(msg));
    assert!(output.is_none(), "Critical message should be dropped");
  }

  #[test]
  #[serial]
  fn test_Req_C_drop_critical_negative_payload() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Critical,
      payload: -100,
    };
    let output = run_gate(Some(msg));
    assert!(output.is_none(), "Critical message should be dropped regardless of payload");
  }

  #[test]
  #[serial]
  fn test_Req_C_drop_critical_zero_payload() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Critical,
      payload: 0,
    };
    let output = run_gate(Some(msg));
    assert!(output.is_none(), "Critical message should be dropped regardless of payload");
  }

  #[test]
  #[serial]
  fn test_Req_C_drop_critical_max_payload() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Critical,
      payload: i32::MAX,
    };
    let output = run_gate(Some(msg));
    assert!(output.is_none(), "Critical message should be dropped regardless of payload");
  }

  //========================================================================
  //  Req_R_1: Restricted messages are passed through unchanged
  //========================================================================

  #[test]
  #[serial]
  fn test_Req_R1_pass_restricted() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: 50,
    };
    let output = run_gate(Some(msg));
    assert!(output == Some(msg), "Restricted message should be passed through unchanged");
  }

  #[test]
  #[serial]
  fn test_Req_R1_pass_restricted_negative_payload() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: -10,
    };
    let output = run_gate(Some(msg));
    assert!(output == Some(msg), "Restricted message should pass through unchanged by Gate");
  }

  #[test]
  #[serial]
  fn test_Req_R1_pass_restricted_large_payload() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: 150,
    };
    let output = run_gate(Some(msg));
    assert!(output == Some(msg),
      "Gate should pass Restricted messages through without modifying payload (Filter handles clamping)");
  }

  //========================================================================
  //  Req_P: Public messages are passed through unchanged
  //========================================================================

  #[test]
  #[serial]
  fn test_Req_P_pass_public() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Public,
      payload: 42,
    };
    let output = run_gate(Some(msg));
    assert!(output == Some(msg), "Public message should be passed through unchanged");
  }

  #[test]
  #[serial]
  fn test_Req_P_pass_public_zero_payload() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Public,
      payload: 0,
    };
    let output = run_gate(Some(msg));
    assert!(output == Some(msg), "Public message should be passed through unchanged");
  }

  #[test]
  #[serial]
  fn test_Req_P_pass_public_min_payload() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Public,
      payload: i32::MIN,
    };
    let output = run_gate(Some(msg));
    assert!(output == Some(msg), "Public message should be passed through unchanged");
  }

  //========================================================================
  //  No input: output should be None
  //========================================================================

  #[test]
  #[serial]
  fn test_no_input() {
    let output = run_gate(None);
    assert!(output.is_none(), "No input should produce no output");
  }

  //========================================================================
  //  Integration constraint: output never has Critical security_level
  //========================================================================

  #[test]
  #[serial]
  fn test_integration_output_never_critical() {
    // Verify that for every security level, the output is never Critical
    let levels = [
      SNG_Data_Model::SecurityLevel::Public,
      SNG_Data_Model::SecurityLevel::Restricted,
      SNG_Data_Model::SecurityLevel::Critical,
    ];
    for level in levels {
      let msg = SNG_Data_Model::Message {
        security_level: level,
        payload: 42,
      };
      let output = run_gate(Some(msg));
      if let Some(out_msg) = output {
        assert!(out_msg.security_level != SNG_Data_Model::SecurityLevel::Critical,
          "Output should never have Critical security level (GUMBO guarantee No_Critical_Output)");
      }
    }
  }
}

//================================================================
//  Manual GUMBOX (contract-based) Tests
//
//  These use cb_apis::testComputeCB to automatically check
//  the GUMBO integration constraints (No_Critical_Output)
//  against the component's actual output.
//================================================================

mod GUMBOX_manual_tests {
  use serial_test::serial;

  use crate::test::util::*;
  use data::*;

  //-- Req_C: Critical messages dropped --

  #[test]
  #[serial]
  fn test_GUMBOX_Req_C_critical_dropped() {
    let input = Some(SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Critical,
      payload: 99,
    });
    let result = cb_apis::testComputeCB(input);
    assert!(matches!(result, cb_apis::HarnessResult::Passed));
  }

  //-- Req_R_1: Restricted messages passed --

  #[test]
  #[serial]
  fn test_GUMBOX_Req_R1_restricted_passed() {
    let input = Some(SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: 50,
    });
    let result = cb_apis::testComputeCB(input);
    assert!(matches!(result, cb_apis::HarnessResult::Passed));
  }

  //-- Req_P: Public messages passed --

  #[test]
  #[serial]
  fn test_GUMBOX_Req_P_public_passed() {
    let input = Some(SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Public,
      payload: 42,
    });
    let result = cb_apis::testComputeCB(input);
    assert!(matches!(result, cb_apis::HarnessResult::Passed));
  }

  //-- No input --

  #[test]
  #[serial]
  fn test_GUMBOX_no_input() {
    let result = cb_apis::testComputeCB(None);
    assert!(matches!(result, cb_apis::HarnessResult::Passed));
  }

  //-- Boundary payloads with all passing security levels --

  #[test]
  #[serial]
  fn test_GUMBOX_boundary_payloads() {
    let payloads = [i32::MIN, -1, 0, 1, 100, 101, i32::MAX];
    let levels = [
      SNG_Data_Model::SecurityLevel::Public,
      SNG_Data_Model::SecurityLevel::Restricted,
      SNG_Data_Model::SecurityLevel::Critical,
    ];
    for level in levels {
      for payload in payloads {
        let input = Some(SNG_Data_Model::Message {
          security_level: level,
          payload,
        });
        let result = cb_apis::testComputeCB(input);
        assert!(matches!(result, cb_apis::HarnessResult::Passed),
          "Failed for security_level={:?}, payload={}", level, payload);
      }
    }
  }
}

//================================================================
//  Automated GUMBOX Tests (property-based testing)
//
//  Uses PropTest to automatically generate random inputs and
//  verify GUMBO contracts hold for all generated test cases.
//================================================================

mod GUMBOX_tests {
  use serial_test::serial;
  use proptest::prelude::*;

  use crate::test::util::*;
  use crate::testInitializeCB_macro;
  use crate::testComputeCB_macro;

  const numValidComputeTestCases: u32 = 100;
  const computeRejectRatio: u32 = 5;
  const verbosity: u32 = 2;

  testInitializeCB_macro! {
    prop_testInitializeCB_macro,
    config: ProptestConfig {
      cases: numValidComputeTestCases,
      max_global_rejects: numValidComputeTestCases * computeRejectRatio,
      verbose: verbosity,
      ..ProptestConfig::default()
    }
  }

  // Default strategy: uniform random security levels and payloads
  testComputeCB_macro! {
    prop_testComputeCB_macro,
    config: ProptestConfig {
      cases: numValidComputeTestCases,
      max_global_rejects: numValidComputeTestCases * computeRejectRatio,
      verbose: verbosity,
      ..ProptestConfig::default()
    },
    api_input: generators::option_strategy_default(generators::SNG_Data_Model_Message_strategy_default())
  }

  // Custom strategy: bias toward Critical to stress-test the drop path
  testComputeCB_macro! {
    prop_testComputeCB_Critical_biased,
    config: ProptestConfig {
      cases: numValidComputeTestCases,
      max_global_rejects: numValidComputeTestCases * computeRejectRatio,
      verbose: verbosity,
      ..ProptestConfig::default()
    },
    api_input: generators::option_strategy_bias(
      5,  // bias toward Some (5:1 vs None)
      generators::SNG_Data_Model_Message_strategy_cust(
        any::<i32>(),
        generators::SNG_Data_Model_SecurityLevel_strategy_cust(
          1,  // Public
          1,  // Restricted
          5   // Critical (heavily biased)
        )
      )
    )
  }

  // Custom strategy: only Some inputs (no None), all security levels
  testComputeCB_macro! {
    prop_testComputeCB_always_some,
    config: ProptestConfig {
      cases: numValidComputeTestCases,
      max_global_rejects: numValidComputeTestCases * computeRejectRatio,
      verbose: verbosity,
      ..ProptestConfig::default()
    },
    api_input: generators::SNG_Data_Model_Message_strategy_default().prop_map(Some)
  }
}
