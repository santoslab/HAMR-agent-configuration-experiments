// This file will not be overwritten if codegen is rerun

//================================================================
//  Filter Component Tests
//
//  The Filter implements payload sanitization:
//    Req_P:    Public messages pass through unchanged
//    Req_R_2a: Restricted payload > 100 is clamped to 100
//    Req_R_2b: Restricted payload < 0 is clamped to 0
//    Req_R_2c: Restricted payload in [0,100] passes unchanged
//
//  Integration assumption (from upstream Gate):
//    Critical messages never arrive at the Filter
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
    crate::msg_filter_msg_filter_initialize();

    // After initialization, output port should have no value
    let output = test_apis::get_output();
    assert!(output.is_none());
  }

  //========================================================================
  //  Helper: initialize, set input, run compute, return output
  //========================================================================
  fn run_filter(input: Option<SNG_Data_Model::Message>) -> Option<SNG_Data_Model::Message>
  {
    crate::msg_filter_msg_filter_initialize();
    test_apis::put_input(input);
    crate::msg_filter_msg_filter_timeTriggered();
    test_apis::get_output()
  }

  //========================================================================
  //  Req_P: Public messages pass through unchanged
  //========================================================================

  #[test]
  #[serial]
  fn test_Req_P_public_unchanged() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Public,
      payload: 42,
    };
    let output = run_filter(Some(msg));
    assert!(output == Some(msg), "Public message should pass through unchanged");
  }

  #[test]
  #[serial]
  fn test_Req_P_public_zero_payload() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Public,
      payload: 0,
    };
    let output = run_filter(Some(msg));
    assert!(output == Some(msg), "Public message with zero payload should pass unchanged");
  }

  #[test]
  #[serial]
  fn test_Req_P_public_negative_payload() {
    // Public messages are NOT clamped -- only Restricted messages are
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Public,
      payload: -500,
    };
    let output = run_filter(Some(msg));
    assert!(output == Some(msg), "Public message should pass unchanged even with negative payload");
  }

  #[test]
  #[serial]
  fn test_Req_P_public_large_payload() {
    // Public messages are NOT clamped -- only Restricted messages are
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Public,
      payload: 99999,
    };
    let output = run_filter(Some(msg));
    assert!(output == Some(msg), "Public message should pass unchanged even with large payload");
  }

  #[test]
  #[serial]
  fn test_Req_P_public_min_payload() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Public,
      payload: i32::MIN,
    };
    let output = run_filter(Some(msg));
    assert!(output == Some(msg), "Public message should pass unchanged with i32::MIN payload");
  }

  #[test]
  #[serial]
  fn test_Req_P_public_max_payload() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Public,
      payload: i32::MAX,
    };
    let output = run_filter(Some(msg));
    assert!(output == Some(msg), "Public message should pass unchanged with i32::MAX payload");
  }

  //========================================================================
  //  Req_R_2a: Restricted payload > 100 clamped to 100
  //========================================================================

  #[test]
  #[serial]
  fn test_Req_R2a_restricted_payload_above_100() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: 150,
    };
    let output = run_filter(Some(msg));
    let expected = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: 100,
    };
    assert!(output == Some(expected),
      "Restricted message with payload 150 should be clamped to 100");
  }

  #[test]
  #[serial]
  fn test_Req_R2a_restricted_payload_101_boundary() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: 101,
    };
    let output = run_filter(Some(msg));
    let expected = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: 100,
    };
    assert!(output == Some(expected),
      "Restricted message with payload 101 (just above boundary) should be clamped to 100");
  }

  #[test]
  #[serial]
  fn test_Req_R2a_restricted_payload_max_i32() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: i32::MAX,
    };
    let output = run_filter(Some(msg));
    let expected = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: 100,
    };
    assert!(output == Some(expected),
      "Restricted message with i32::MAX payload should be clamped to 100");
  }

  //========================================================================
  //  Req_R_2b: Restricted payload < 0 clamped to 0
  //========================================================================

  #[test]
  #[serial]
  fn test_Req_R2b_restricted_payload_below_0() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: -10,
    };
    let output = run_filter(Some(msg));
    let expected = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: 0,
    };
    assert!(output == Some(expected),
      "Restricted message with payload -10 should be clamped to 0");
  }

  #[test]
  #[serial]
  fn test_Req_R2b_restricted_payload_minus_1_boundary() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: -1,
    };
    let output = run_filter(Some(msg));
    let expected = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: 0,
    };
    assert!(output == Some(expected),
      "Restricted message with payload -1 (just below boundary) should be clamped to 0");
  }

  #[test]
  #[serial]
  fn test_Req_R2b_restricted_payload_min_i32() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: i32::MIN,
    };
    let output = run_filter(Some(msg));
    let expected = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: 0,
    };
    assert!(output == Some(expected),
      "Restricted message with i32::MIN payload should be clamped to 0");
  }

  //========================================================================
  //  Req_R_2c: Restricted payload in [0,100] passes unchanged
  //========================================================================

  #[test]
  #[serial]
  fn test_Req_R2c_restricted_payload_in_range() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: 50,
    };
    let output = run_filter(Some(msg));
    assert!(output == Some(msg),
      "Restricted message with payload 50 (in range) should pass unchanged");
  }

  #[test]
  #[serial]
  fn test_Req_R2c_restricted_payload_lower_boundary_0() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: 0,
    };
    let output = run_filter(Some(msg));
    assert!(output == Some(msg),
      "Restricted message with payload 0 (lower boundary) should pass unchanged");
  }

  #[test]
  #[serial]
  fn test_Req_R2c_restricted_payload_upper_boundary_100() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: 100,
    };
    let output = run_filter(Some(msg));
    assert!(output == Some(msg),
      "Restricted message with payload 100 (upper boundary) should pass unchanged");
  }

  #[test]
  #[serial]
  fn test_Req_R2c_restricted_payload_1() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: 1,
    };
    let output = run_filter(Some(msg));
    assert!(output == Some(msg),
      "Restricted message with payload 1 (just above lower boundary) should pass unchanged");
  }

  #[test]
  #[serial]
  fn test_Req_R2c_restricted_payload_99() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: 99,
    };
    let output = run_filter(Some(msg));
    assert!(output == Some(msg),
      "Restricted message with payload 99 (just below upper boundary) should pass unchanged");
  }

  //========================================================================
  //  No input: output should be None
  //========================================================================

  #[test]
  #[serial]
  fn test_no_input() {
    let output = run_filter(None);
    assert!(output.is_none(), "No input should produce no output");
  }

  //========================================================================
  //  Security level preservation: output always has the same security level
  //========================================================================

  #[test]
  #[serial]
  fn test_security_level_preserved_public() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Public,
      payload: 42,
    };
    let output = run_filter(Some(msg)).unwrap();
    assert!(output.security_level == SNG_Data_Model::SecurityLevel::Public,
      "Output security level should be preserved for Public messages");
  }

  #[test]
  #[serial]
  fn test_security_level_preserved_restricted() {
    let msg = SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: 999,
    };
    let output = run_filter(Some(msg)).unwrap();
    assert!(output.security_level == SNG_Data_Model::SecurityLevel::Restricted,
      "Output security level should be preserved for Restricted messages");
  }

  //========================================================================
  //  Comprehensive boundary test: all clamping boundaries in one test
  //========================================================================

  #[test]
  #[serial]
  fn test_clamping_boundaries_comprehensive() {
    // Test values around the clamping boundaries for Restricted messages
    let test_cases: &[(i32, i32)] = &[
      // (input_payload, expected_output_payload)
      (i32::MIN, 0),   // far below -> clamp to 0
      (-1,       0),   // just below -> clamp to 0
      (0,        0),   // lower boundary -> unchanged
      (1,        1),   // just above lower -> unchanged
      (50,       50),  // middle of range -> unchanged
      (99,       99),  // just below upper -> unchanged
      (100,      100), // upper boundary -> unchanged
      (101,      100), // just above upper -> clamp to 100
      (i32::MAX, 100), // far above -> clamp to 100
    ];

    for &(input_payload, expected_payload) in test_cases {
      let msg = SNG_Data_Model::Message {
        security_level: SNG_Data_Model::SecurityLevel::Restricted,
        payload: input_payload,
      };
      let output = run_filter(Some(msg));
      let expected = SNG_Data_Model::Message {
        security_level: SNG_Data_Model::SecurityLevel::Restricted,
        payload: expected_payload,
      };
      assert!(output == Some(expected),
        "Restricted payload {}: expected output payload {}, got {:?}",
        input_payload, expected_payload, output);
    }
  }
}

//================================================================
//  Manual GUMBOX (contract-based) Tests
//
//  These use cb_apis::testComputeCB to check that the GUMBO
//  integration precondition (No_Critical_Input) is respected
//  and the component behavior is consistent.
//================================================================

mod GUMBOX_manual_tests {
  use serial_test::serial;

  use crate::test::util::*;
  use data::*;

  //-- Req_P: Public messages --

  #[test]
  #[serial]
  fn test_GUMBOX_Req_P_public() {
    let input = Some(SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Public,
      payload: 42,
    });
    let result = cb_apis::testComputeCB(input);
    assert!(matches!(result, cb_apis::HarnessResult::Passed));
  }

  //-- Req_R_2: Restricted message clamping --

  #[test]
  #[serial]
  fn test_GUMBOX_Req_R2a_clamp_above() {
    let input = Some(SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: 150,
    });
    let result = cb_apis::testComputeCB(input);
    assert!(matches!(result, cb_apis::HarnessResult::Passed));
  }

  #[test]
  #[serial]
  fn test_GUMBOX_Req_R2b_clamp_below() {
    let input = Some(SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: -10,
    });
    let result = cb_apis::testComputeCB(input);
    assert!(matches!(result, cb_apis::HarnessResult::Passed));
  }

  #[test]
  #[serial]
  fn test_GUMBOX_Req_R2c_in_range() {
    let input = Some(SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Restricted,
      payload: 50,
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

  //-- Critical input should be rejected by precondition --
  //   (The GUMBO integration assume No_Critical_Input
  //    means Critical inputs violate the precondition)

  #[test]
  #[serial]
  fn test_GUMBOX_critical_rejected_precondition() {
    let input = Some(SNG_Data_Model::Message {
      security_level: SNG_Data_Model::SecurityLevel::Critical,
      payload: 42,
    });
    let result = cb_apis::testComputeCB(input);
    assert!(matches!(result, cb_apis::HarnessResult::RejectedPrecondition),
      "Critical input should be rejected by the GUMBO precondition (No_Critical_Input)");
  }

  //-- Boundary payloads with valid security levels --

  #[test]
  #[serial]
  fn test_GUMBOX_boundary_payloads() {
    let payloads = [i32::MIN, -1, 0, 1, 50, 99, 100, 101, i32::MAX];
    let levels = [
      SNG_Data_Model::SecurityLevel::Public,
      SNG_Data_Model::SecurityLevel::Restricted,
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
//
//  The Filter has a GUMBO precondition (No_Critical_Input)
//  so Critical inputs are automatically rejected by the harness.
//  Custom strategies are used to avoid excessive rejections.
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

  // Default strategy: generates all security levels including Critical
  // (Critical inputs are rejected by the GUMBO precondition)
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

  // Custom strategy: only non-Critical messages to avoid rejections
  testComputeCB_macro! {
    prop_testComputeCB_no_rejections,
    config: ProptestConfig {
      cases: numValidComputeTestCases,
      max_global_rejects: 0,  // zero rejections allowed
      verbose: verbosity,
      ..ProptestConfig::default()
    },
    api_input: generators::option_strategy_default(
      generators::SNG_Data_Model_Message_strategy_cust(
        any::<i32>(),
        generators::SNG_Data_Model_SecurityLevel_strategy_cust(
          1,  // Public
          1,  // Restricted
          0   // Critical excluded
        )
      )
    )
  }

  // Custom strategy: Restricted messages only, with payload focused on
  // the clamping boundaries [-5, 105] to stress-test boundary behavior
  testComputeCB_macro! {
    prop_testComputeCB_restricted_boundary,
    config: ProptestConfig {
      cases: 200,
      max_global_rejects: 0,
      verbose: verbosity,
      ..ProptestConfig::default()
    },
    api_input: generators::SNG_Data_Model_Message_strategy_cust(
      (-5i32..=105i32),
      generators::SNG_Data_Model_SecurityLevel_strategy_cust(
        0,  // no Public
        1,  // Restricted only
        0   // no Critical
      )
    ).prop_map(Some)
  }

  // Custom strategy: always Some input, only valid (non-Critical) messages
  testComputeCB_macro! {
    prop_testComputeCB_always_some_valid,
    config: ProptestConfig {
      cases: numValidComputeTestCases,
      max_global_rejects: 0,
      verbose: verbosity,
      ..ProptestConfig::default()
    },
    api_input: generators::SNG_Data_Model_Message_strategy_cust(
      any::<i32>(),
      generators::SNG_Data_Model_SecurityLevel_strategy_cust(
        1,  // Public
        1,  // Restricted
        0   // no Critical
      )
    ).prop_map(Some)
  }
}
