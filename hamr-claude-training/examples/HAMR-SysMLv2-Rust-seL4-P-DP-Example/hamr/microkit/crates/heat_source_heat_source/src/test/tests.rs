// This file will not be overwritten if codegen is rerun

//============================================================================
//  H e a t   S o u r c e   --   ((Testing))
//
//  Since this thread component has very simple (no functionality except to maintain
//  a very simple heater device simulation) and no GUMBO-specified behavior, 
//  the testing that we provide for this component is very simple:
//   - we test whether or not the simulation of the heater device 
//     aligns with heat control commands sent to the component, e.g., 
//     receipt of a On command causes the simulated heater device to be placed
//     in an on state.
//
//  There are only manual tests and no GUMBOX (contract-based) tests because 
//  there are no GUMBO contracts for this component.
//
//  Note: this code illustrates how to access non-GUMBO declared state 
//  (the simulated heater device state) in test code.
//============================================================================


mod tests {
  // NOTE: need to run tests sequentially to prevent race conditions
  //       on the app and the testing apis which are static
  use serial_test::serial;

  use crate::test::util::*;
  use data::*;
  use data::Isolette_Data_Model::*;

  // ------------- helper methods -------------

  // Define helper method to access the `heater_state` non-GUMBO state variable 
  // used in heater simulation.
  //
  // For GUMBO-declared `state` variables, HAMR auto-generates a getter method.
  // For non-GUMBO-declared `state` variables, we need to define getters manually.
  //
  // Note: This illustrates a general approach for how to do the following in testing code:
  //  (a) access the singleton "app" value holding the state for a 
  //   HAMR Rust component, 
  //  (b) access a non-GUMBO-declared state variable (a field in the app struct).
  //
  pub fn get_heater_state() -> On_Off
  {
     unsafe {
       match &crate::app {
         Some(inner) => inner.heater_state,
         None => panic!("The app is None")
       }
    }
  }

  // ------------ manual tests ---------------

  //========================================================================
  //  REQ-HS-1: 
  //     The heat source is initially in the OFF state
  //========================================================================

  #[test]
  #[serial]
  fn test_initialization_REQ_HS_1() {
    crate::heat_source_heat_source_initialize();

    // After initialization, the state of the simulated heater should be OFF
    let heater_state: On_Off = get_heater_state();
    assert!(heater_state == On_Off::Off);
  }

  //========================================================================
  //  REQ-HS-2: 
  //    When commanded to be ON, the heat source shall be active 
  //     (be in the On state)
  //========================================================================

  #[test]
  #[serial]
  fn test_compute_heater_REQ_HS_2() {
    crate::heat_source_heat_source_initialize();

    // populate incoming data ports
    test_apis::put_heat_control(On_Off::Onn);

    // invoke compute entry point
    crate::heat_source_heat_source_timeTriggered();

    // After compute entry point execution, the state of the simulated heater should be match the input (Onn)
    let heater_state: On_Off = get_heater_state();
    assert!(heater_state == On_Off::Onn);
  }

  //========================================================================
  //  REQ-HS-3: 
  //    When commanded to be OFF, the heat source shall not be active 
  //    (be in the Off state)
  //========================================================================

  #[test]
  #[serial]
  fn test_compute_heater_REQ_HS_3() {
    crate::heat_source_heat_source_initialize();

    // populate incoming data ports
    test_apis::put_heat_control(On_Off::Off);

    // invoke compute entry point
    crate::heat_source_heat_source_timeTriggered();

    // After compute entry point execution, the state of the simulated heater should be match the input (Off)
    let heater_state: On_Off = get_heater_state();
    assert!(heater_state == On_Off::Off);
  }

}
