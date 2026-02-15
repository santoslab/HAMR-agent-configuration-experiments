// This file will not be overwritten if codegen is rerun

//============================================================================
//  O p e r a t o r     I n t e r f a c e   --   ((Testing))
//
//  The only specified behavior for this component is that 
//  set points values fall within a specified range, and that the upper
//  set point value is less than the lower set point value.
//
//  The implementation of the operator interface component provides a simple simulation
//  that reports set points.  Thus, the testing provided here
//  ensures that the simulated set point values conform to the constraints 
//  listed above.
//============================================================================

mod tests {
  // NOTE: need to run tests sequentially to prevent race conditions
  //       on the app and the testing apis which are static
  use serial_test::serial;

  use crate::test::util::*;
  use data::*;
  use data::Isolette_Data_Model::*; // Add to simplify reference to data types

  #[test]
  #[serial]
  fn test_initialization() {
    crate::operator_interface_operator_interface_initialize();
}

  #[test]
  #[serial]
  fn test_compute() {
    crate::operator_interface_operator_interface_initialize();
    crate::operator_interface_operator_interface_timeTriggered();
  }

  //---------------------------------------------------
  //  Test REQ_OP_1, REQ_OP_2, REQ_OP_3 in a single test
  //  via repeated dispatches of the compute entry point  
  //---------------------------------------------------

  // Helper function to test if sensor output temperature is
  // is in the expected range (utilize constants declared in app component).
  fn desired_temp_in_range(sp: Set_Points) -> bool {
       sp.lower.degrees <= sp.upper.degrees
    && 97 <= sp.lower.degrees 
    && sp.upper.degrees <= 102
  }

  #[test]
  #[serial]
  fn test_compute_REQ_OP_INTERFACE_repeated() {
    // [InvokeEntryPoint]: invoke the initialize entry point 
    //   to initialize the state of the component
    crate::operator_interface_operator_interface_initialize();
 
    // test sensor simulation by repeatedly invoking the compute entry point and 
    // checking the visible post-state each time.

    for _ in 0..20 {
       // [InvokeEntryPoint]: invoke the compute entry point 
       crate::operator_interface_operator_interface_timeTriggered();
       // get result values from output ports
       let api_desired_temp = test_apis::get_desired_temp();
       assert!(desired_temp_in_range(api_desired_temp));
    }
  }

}
