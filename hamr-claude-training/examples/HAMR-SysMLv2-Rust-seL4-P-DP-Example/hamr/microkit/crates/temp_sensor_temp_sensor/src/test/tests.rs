// This file will not be overwritten if codegen is rerun

//============================================================================
//  T e m p    S e n s o r  --   ((Testing))
//
//  The only specified behavior for this component is that 
//  temperature readings fall within a specified range.  Constants
//  defining the upper and lower bounds for the range are declared in 
//  the application code.
//
//  The implementation of the sensor component provides a simple simulation
//  that reports temperature values.  Thus, the testing provided here
//  ensures that the simulated temperature values fall within the specified 
//  range.
//============================================================================

mod tests {
  // NOTE: need to run tests sequentially to prevent race conditions
  //       on the app and the testing apis which are static
  use serial_test::serial;

  use crate::test::util::*;
  use data::*;
  use data::Isolette_Data_Model::*;

  // Import app module to access declared const for temp bounds
  // (rename module to something shorter for convenience).
  use crate::component::temp_sensor_temp_sensor_app as app; 

  #[test]
  #[serial]
  fn test_initialization() {
    crate::temp_sensor_temp_sensor_initialize();
}

  // Tire-kicking test for compute entry point (timeTriggered method)
  // ...just run initialize followed by time-triggered to exercise execution.
  // This simple approach works in this case because the component has no input ports
  // (so no need to supply values), and we choose not to examine output ports.
  #[test]
  #[serial]
  fn test_compute() {
    crate::temp_sensor_temp_sensor_initialize();
    crate::temp_sensor_temp_sensor_timeTriggered();
  }

  //========================================================================
  //  REQ_TS_1: the Current Temperature provided by the temperature sensor 
  //    lies within the range of 96 and 103 inclusive.
  //========================================================================

     /*
       Inputs:
         (none)

       Expected Outputs:
         current_temp: (in range)
    */

  // Helper function to test if sensor output temperature is
  // is in the expected range (utilize constants declared in app component).
  fn current_temp_in_range(ct: Temp) -> bool {
       app::sensed_temp_lower_bound <= ct.degrees 
    && ct.degrees <= app::sensed_temp_upper_bound
  }

  #[test]
  #[serial]
  fn test_compute_REQ_TS_1() {
    // [InvokeEntryPoint]: invoke the initialize entry point 
    //   to initialize the state of the component
    crate::temp_sensor_temp_sensor_initialize();

    // [InvokeEntryPoint]: invoke the compute entry point 
    crate::temp_sensor_temp_sensor_timeTriggered();

     // get result values from output ports
    let api_current_temp = test_apis::get_current_temp();
    
    assert!(current_temp_in_range(api_current_temp));
  }

  #[test]
  #[serial]
  fn test_compute_REQ_TS_1_repeated() {
    // [InvokeEntryPoint]: invoke the initialize entry point 
    //   to initialize the state of the component
    crate::temp_sensor_temp_sensor_initialize();

    // test sensor simulation by repeatedly invoking the compute entry point and 
    // checking the visible post-state each time.

    for _ in 0..20 {
       // [InvokeEntryPoint]: invoke the compute entry point 
       crate::temp_sensor_temp_sensor_timeTriggered();
       // get result values from output ports
       let api_current_temp = test_apis::get_current_temp();
       assert!(current_temp_in_range(api_current_temp));
    }
  }

}

mod GUMBOX_tests {
  use serial_test::serial;
  use proptest::prelude::*;

  use crate::test::util::*;
  use crate::testInitializeCB_macro;
  use crate::testComputeCB_macro;

  // number of valid (i.e., non-rejected) test cases that must be executed for the compute method.
  const numValidComputeTestCases: u32 = 100;

  // how many total test cases (valid + rejected) that may be attempted.
  //   0 means all inputs must satisfy the precondition (if present),
  //   5 means at most 5 rejected inputs are allowed per valid test case
  const computeRejectRatio: u32 = 5;

  const verbosity: u32 = 2;

  testInitializeCB_macro! {
    prop_testInitializeCB_macro, // test name
    config: ProptestConfig { // proptest configuration, built by overriding fields from default config
      cases: numValidComputeTestCases,
      max_global_rejects: numValidComputeTestCases * computeRejectRatio,
      verbose: verbosity,
      ..ProptestConfig::default()
    }
  }

  testComputeCB_macro! {
    prop_testComputeCB_macro, // test name
    config: ProptestConfig { // proptest configuration, built by overriding fields from default config
      cases: numValidComputeTestCases,
      max_global_rejects: numValidComputeTestCases * computeRejectRatio,
      verbose: verbosity,
      ..ProptestConfig::default()
    }
  }
}

