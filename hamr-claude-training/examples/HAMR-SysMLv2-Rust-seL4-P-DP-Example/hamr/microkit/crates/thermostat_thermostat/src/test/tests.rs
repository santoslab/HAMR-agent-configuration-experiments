// This file will not be overwritten if codegen is rerun

//----------------------------------------------------------------
//  This file illustrates three types of HAMR unit (thread application code)
//  testing:
//    - manual tests 
//    - manual GUMBOX tests
//    - automated randomized GUMBOX tests (property-based testing)
//
//  See explanations for the mechanics and purpose of each type of test
//  in the header sections below.
//----------------------------------------------------------------


//================================================================
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
//
//  M a n u a l l y - w r i t t e n    U n i t    T e s t s  
//
//  These examples illustrate how to use the basic APIs auto-generated
//  by HAMR to support unit testing for a HAMR thread component.
//  The test APIs are found in src/test/util/test_apis.rs and are 
//  re-generated each time HAMR code generation is run (e.g., after
//  changes are made to model structures or model contracts).
//
//  In manually written tests, code is written to 
//   - construct a set of component inputs, 
//  i.e., a "test vector" that provides a value
//  for each input port (and optionally, each GUMBO-specified component
//  variable (GSV))
//   - put the test vector values into the component's inputs via `put_`
//     API calls
//   - invoke a component entry point (e.g., the compute entry point)
//     to run component application code (which will leave values in
//     output ports and update GSVs) 
//   - get values of component output ports and GSVs via `get_` APIs.
//   - compare output values with expected values via `assert!(..)`
//
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
//================================================================


mod tests {
  // NOTE: need to run tests sequentially to prevent race conditions
  //       on the app and the testing apis which are static
  use serial_test::serial;

  use crate::test::util::*;
  use data::*;
  use data::Isolette_Data_Model::*; // Add for easier reference of data types

  // auto-generated example test for initialize entry point
  #[test]
  #[serial]
  fn test_initialization() {
    // invoke initialize entry point
    crate::thermostat_thermostat_initialize();

    // use auto-generated test APIs to retrieve values of 
    // output ports and GUMBO-defined local state as illustrated below

    // let heat_control: On_Off = test_apis::get_heat_control();
    // let post_lastCmd : On_Off = test_apis::get_lastCmd();

    // ..compare outputs to expected results..
    // assert!(..)    
  }

  #[test]
  #[serial]
  fn test_initialization_REQ_THERM_1() {
    // invoke initialize entry point
    crate::thermostat_thermostat_initialize();

    // use auto-generated test APIs to retrieve values of 
    // output ports and GUMBO-defined local state 
    let heat_control: On_Off = test_apis::get_heat_control(); // output port
    let post_lastCmd : On_Off = test_apis::get_lastCmd(); // GUMBO defined local state (post-state value)

    // ..compare outputs to expected results..
    // REQ_THERM_1 
    //  - The Heat Control command shall be initially Off
    assert!(heat_control == On_Off::Off);
    assert!(post_lastCmd == On_Off::Off);
  }

  // auto-generated example test for `compute` entry point
  #[test]
  #[serial]
  fn test_compute() {
    // initialize entry point should always be called to initialize 
    // output ports and local state
    crate::thermostat_thermostat_initialize();

    // implement input of "test vector"
    // ..set values for input data ports
    test_apis::put_current_temp(Isolette_Data_Model::Temp::default());
    test_apis::put_desired_temp(Isolette_Data_Model::Set_Points::default());
    // ..[optional] set values for GUMBO-declared local state variables
    test_apis::put_lastCmd(Isolette_Data_Model::On_Off::default());

    // invoke initialize entry point
    crate::thermostat_thermostat_timeTriggered();

    // use auto-generated test APIs to retrieve values of 
    // output ports and GUMBO-defined local state as illustrated below

    // let heat_control: On_Off = test_apis::get_heat_control();
    // let post_lastCmd : On_Off = test_apis::get_lastCmd();

    // ..compare outputs to expected results..
    // assert!(..)   

  }

  //========================================================================
  //  REQ-THERM-2: If Current Temperature is less than the Lower Desired Temperature,
  //  the Heat Control shall be set to On.
  //========================================================================

     /*
       Inputs:
         current_temp:  95f 
         lower_desired_temp: 98f
         upper_desired_temp: *irrelevant to requirement* (use 100f)

       Expected Outputs:
         heat_control: On
         last_cmd (post): On
    */
  #[test]
  #[serial]
  fn test_compute_REQ_THERM_2() {
    // [InvokeEntryPoint]: invoke the entry point test method
    crate::thermostat_thermostat_initialize();

    // generate values for the incoming data ports
    let current_temp = Temp { degrees: 95 };
    let lower_desired_temp = Temp { degrees: 98 };
    let upper_desired_temp = Temp { degrees: 100 };
    let desired_temp = Set_Points {lower: lower_desired_temp, upper: upper_desired_temp};
 
    // [PutInPorts]: put values on the input ports
    test_apis::put_current_temp(current_temp);
    test_apis::put_desired_temp(desired_temp);

    // [InvokeEntryPoint]: Invoke the entry point
    crate::thermostat_thermostat_timeTriggered();

    // get result values from output ports
    let api_heat_control = test_apis::get_heat_control();
    let lastCmd = test_apis::get_lastCmd();

    assert!(api_heat_control == On_Off::Onn);
    assert!(lastCmd == On_Off::Onn);
  }

  #[test]
  #[serial]
  fn test_compute_REQ_THERM_2_container() { // Alternate version: Illustrate "container"-based APIs
    // [InvokeEntryPoint]: invoke the entry point test method
    crate::thermostat_thermostat_initialize();

    // Inputs can be "bundled" into a container and put "all at once".
    // There are two versions Pre-State-Container: one without inputs
    // for GUMBO state variables (GSV) and one that includes them.  Below
    // illustrates the version without.   In this case, GSVs retain
    // whatever value they have at the time of the Compute entry point
    // (timeTriggered method) invocation.
    let preStateContainer = test_apis::PreStateContainer {
      api_current_temp : Temp { degrees: 95},
      api_desired_temp : Set_Points {
        lower: Temp { degrees:  98 },
        upper: Temp { degrees: 100 }
      }    
    };

    // [PutInPorts]: put values on the input ports
    test_apis::put_concrete_inputs_container(preStateContainer);

    // [InvokeEntryPoint]: Invoke the entry point
    crate::thermostat_thermostat_timeTriggered();

    // get result values from output ports
    let api_heat_control = test_apis::get_heat_control();
    let lastCmd = test_apis::get_lastCmd();

    assert!(api_heat_control == On_Off::Onn);
    assert!(lastCmd == On_Off::Onn);
  }

  //. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .
  //  Illustrate use of helper functions for repeated patterns
  //. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

  fn test_compute_THERM_helper(
    // container 
    inputs: test_apis::PreStateContainer,
    // expected result
    expected_ouput: On_Off)
  {
    // [InvokeEntryPoint]: invoke the entry point test method
    crate::thermostat_thermostat_initialize();

    // [PutInPorts]: put values on the input ports 
    test_apis::put_concrete_inputs_container(inputs);

    // [InvokeEntryPoint]: Invoke the entry point
    crate::thermostat_thermostat_timeTriggered();

    // get result values from output ports
    let api_heat_control = test_apis::get_heat_control();
    let lastCmd = test_apis::get_lastCmd();

    assert!(api_heat_control == expected_ouput);
    assert!(lastCmd == expected_ouput);
  }
  
  #[test]
  #[serial]
  fn test_compute_REQ_THERM_3_container() { // Alternate version: Illustrate "container"-based APIs
    // [InvokeEntryPoint]: invoke the entry point test method
    crate::thermostat_thermostat_initialize();

    let preStateContainer = test_apis::PreStateContainer {
      api_current_temp : Temp { degrees: 101},
      api_desired_temp : Set_Points {
        lower: Temp { degrees:  98 },
        upper: Temp { degrees: 100 }
      }    
    };

    // [PutInPorts]: put values on the input ports
    test_apis::put_concrete_inputs_container(preStateContainer);

    // [InvokeEntryPoint]: Invoke the entry point
    crate::thermostat_thermostat_timeTriggered();

    // get result values from output ports
    let api_heat_control = test_apis::get_heat_control();
    let lastCmd = test_apis::get_lastCmd();

    assert!(api_heat_control == On_Off::Off);
    assert!(lastCmd == On_Off::Off);
  }

  #[test]
  #[serial]
  fn test_compute_REQ_THERM_2_helper() { // Alternate version: Illustrate "container"-based helpers
     test_compute_THERM_helper(
     test_apis::PreStateContainer {
        api_current_temp : Temp { degrees: 95},
        api_desired_temp : Set_Points {
          lower: Temp { degrees:  98 },
          upper: Temp { degrees: 100 }
      }
    },
    On_Off::Onn
   );
  }


  // EXERCISES:
  //  - construct a test for REQ-THERM-3, making direct use of APIs
  //    without a container
  //  - construct an alternate test for REQ-THERM-3, using a container structure 
  //    and the method `put_concrete_inputs_container`
  //  - construct an alternate test for REQ-THERM-3 that uses the
  //    helper method `test_compute_THERM_helper`


   //========================================================================
 //  REQ-THERM-4: If the Current Temperature is greater than or equal
 //  to the Lower Desired Temperature and less than or equal to the
 //  Upper Desired Temperature, the value of the Heat Control shall not be changed.":
 //========================================================================

  // Test design notes:
  // the "shall not be changed" requires reasoning about the internal
  // state of the component (saved heat control value) or about the value
  // of the heat control output on the previous activation.  To test the requirement above,
  // we create a sequence of activations and assertions that capture the requirement
  // in terms of values on previous activations.  An alternative approach is to
  // modify the internal state of the component directly -- that approach is
  // illustrated in the REQ-THERM-4-alt tests.

  //. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .
  //  Illustrate "inheriting" values of last_cmd from previous iterations
  //. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

  #[test]
  #[serial]
  fn test_compute_REQ_THERM_4() {
    // [InvokeEntryPoint]: invoke the entry point test method
    crate::thermostat_thermostat_initialize();

    // Activation 1: Cause heat control to be On

    /*
      Inputs:
        current_tempWstatus  95f  
        lower_desired_temp: (use 98f)
        upper_desired_temp: (use 100f)
    
      Expected Outputs:
        heat_control: On
        last_cmd: On
    */

    // generate values for the incoming data ports
    let current_temp = Temp { degrees: 95 };
    let lower_desired_temp = Temp { degrees: 98 };
    let upper_desired_temp = Temp { degrees: 100 };
    let desired_temp = Set_Points {lower: lower_desired_temp, upper: upper_desired_temp};
 
    // [PutInPorts]: put values on the input ports
    test_apis::put_current_temp(current_temp);
    test_apis::put_desired_temp(desired_temp);

    // [InvokeEntryPoint]: Invoke the entry point
    crate::thermostat_thermostat_timeTriggered();

    // get result values from output ports
    let api_heat_control = test_apis::get_heat_control();
    let lastCmd = test_apis::get_lastCmd();

    assert!(api_heat_control == On_Off::Onn);
    assert!(lastCmd == On_Off::Onn);

    // Activation 1-a: Current temp lies within set points (closed interval)
    //   in this case current temp is equal to lower bound,
    //   so heat control should still be on
    /*
       Inputs:
         current_tempWstatus  98f  
         lower_desired_temp: (use 98f)
         upper_desired_temp: (use 100f)
         regulator_mode: Normal

       Expected Outputs:
         heat_control: On
         last_cmd: On
    */

    // generate values for the incoming data ports
    let current_temp = Temp { degrees: 98 };
    let lower_desired_temp = Temp { degrees: 98 };
    let upper_desired_temp = Temp { degrees: 100 };
    let desired_temp = Set_Points {lower: lower_desired_temp, upper: upper_desired_temp};
 
    // [PutInPorts]: put values on the input ports
    test_apis::put_current_temp(current_temp);
    test_apis::put_desired_temp(desired_temp);

    // [InvokeEntryPoint]: Invoke the entry point
    crate::thermostat_thermostat_timeTriggered();

    // get result values from output ports
    let api_heat_control = test_apis::get_heat_control();
    let lastCmd = test_apis::get_lastCmd();

    assert!(api_heat_control == On_Off::Onn);
    assert!(lastCmd == On_Off::Onn);

    // Activation 1-b: Current temp lies within set points (closed interval)
    //   in this case current temp is between lower bound and upper bound,
    //   so heat control should still be on
    /*
       Inputs:
         current_tempWstatus  99f  
         lower_desired_temp: (use 98f)
         upper_desired_temp: (use 100f)
         regulator_mode: Normal

       Expected Outputs:
         heat_control: On
         last_cmd: On
   */

   // generate values for the incoming data ports
    let current_temp = Temp { degrees: 99 };
    let lower_desired_temp = Temp { degrees: 98 };
    let upper_desired_temp = Temp { degrees: 100 };
    let desired_temp = Set_Points {lower: lower_desired_temp, upper: upper_desired_temp};
 
    // [PutInPorts]: put values on the input ports
    test_apis::put_current_temp(current_temp);
    test_apis::put_desired_temp(desired_temp);

    // [InvokeEntryPoint]: Invoke the entry point
    crate::thermostat_thermostat_timeTriggered();

    // get result values from output ports
    let api_heat_control = test_apis::get_heat_control();
    let lastCmd = test_apis::get_lastCmd();

    assert!(api_heat_control == On_Off::Onn);
    assert!(lastCmd == On_Off::Onn);

    // Activation 1-c: Current temp lies within set points (closed interval)
    //   in this case current temp is equal to upper bound,
    //   so heat control should still be on

    /*
       Inputs:
         current_tempWstatus  100f  
         lower_desired_temp: (use 98f)
         upper_desired_temp: (use 100f)
         regulator_mode: Normal

       Expected Outputs:
         heat_control: On
         last_cmd: On
   */

    // generate values for the incoming data ports
    let current_temp = Temp { degrees: 100 };
    let lower_desired_temp = Temp { degrees: 98 };
    let upper_desired_temp = Temp { degrees: 100 };
    let desired_temp = Set_Points {lower: lower_desired_temp, upper: upper_desired_temp};
 
    // [PutInPorts]: put values on the input ports
    test_apis::put_current_temp(current_temp);
    test_apis::put_desired_temp(desired_temp);

    // [InvokeEntryPoint]: Invoke the entry point
    crate::thermostat_thermostat_timeTriggered();

    // get result values from output ports
    let api_heat_control = test_apis::get_heat_control();
    let lastCmd = test_apis::get_lastCmd();

    assert!(api_heat_control == On_Off::Onn);
    assert!(lastCmd == On_Off::Onn);

    // Activation 2:   Cause the heat control to be off
    //   Current temp lies above set points
    //   in this case current temp is greater than upper bound,
    //   so heat control should be off
    /*
       Inputs:
         current_tempWstatus  101f  
         lower_desired_temp: (use 98f)
         upper_desired_temp: (use 100f)
         regulator_mode: Normal

       Expected Outputs:
         heat_control: Off
         last_cmd: Off
    */

    // generate values for the incoming data ports
    let current_temp = Temp { degrees: 101 };
    let lower_desired_temp = Temp { degrees: 98 };
    let upper_desired_temp = Temp { degrees: 100 };
    let desired_temp = Set_Points {lower: lower_desired_temp, upper: upper_desired_temp};
 
    // [PutInPorts]: put values on the input ports
    test_apis::put_current_temp(current_temp);
    test_apis::put_desired_temp(desired_temp);

    // [InvokeEntryPoint]: Invoke the entry point
    crate::thermostat_thermostat_timeTriggered();

    // get result values from output ports
    let api_heat_control = test_apis::get_heat_control();
    let lastCmd = test_apis::get_lastCmd();

    assert!(api_heat_control == On_Off::Off);
    assert!(lastCmd == On_Off::Off);

    // Activation 2-a:
    //   Current temp lies within set points (closed interval)
    //   in this case current temp is equal to upper bound,
    //   so heat control should still be off

    /*
       Inputs:
         current_tempWstatus  100f  
         lower_desired_temp: (use 98f)
         upper_desired_temp: (use 100f)
         regulator_mode: Normal

       Expected Outputs:
         heat_control: Off
         last_cmd: Off
    */

    // generate values for the incoming data ports
    let current_temp = Temp { degrees: 100 };
    let lower_desired_temp = Temp { degrees: 98 };
    let upper_desired_temp = Temp { degrees: 100 };
    let desired_temp = Set_Points {lower: lower_desired_temp, upper: upper_desired_temp};
 
    // [PutInPorts]: put values on the input ports
    test_apis::put_current_temp(current_temp);
    test_apis::put_desired_temp(desired_temp);

    // [InvokeEntryPoint]: Invoke the entry point
    crate::thermostat_thermostat_timeTriggered();

    // get result values from output ports
    let api_heat_control = test_apis::get_heat_control();
    let lastCmd = test_apis::get_lastCmd();

    assert!(api_heat_control == On_Off::Off);
    assert!(lastCmd == On_Off::Off);

    // Activation 2-b:
    //   Current temp lies within set points (closed interval)
    //   in this case current temp is between lower and upper bound,
    //   so heat control should still be off

    /*
       Inputs:
         current_tempWstatus  99f  
         lower_desired_temp: (use 98f)
         upper_desired_temp: (use 100f)
         regulator_mode: Normal

       Expected Outputs:
         heat_control: Off
         last_cmd: Off
    */

    // generate values for the incoming data ports
    let current_temp = Temp { degrees: 99 };
    let lower_desired_temp = Temp { degrees: 98 };
    let upper_desired_temp = Temp { degrees: 100 };
    let desired_temp = Set_Points {lower: lower_desired_temp, upper: upper_desired_temp};
 
    // [PutInPorts]: put values on the input ports
    test_apis::put_current_temp(current_temp);
    test_apis::put_desired_temp(desired_temp);

    // [InvokeEntryPoint]: Invoke the entry point
    crate::thermostat_thermostat_timeTriggered();

    // get result values from output ports
    let api_heat_control = test_apis::get_heat_control();
    let lastCmd = test_apis::get_lastCmd();

    assert!(api_heat_control == On_Off::Off);
    assert!(lastCmd == On_Off::Off);

    // Activation 2-c:
    //   Current temp lies within set points (closed interval)
    //   in this case current temp is equal to lower bound,
    //   so heat control should still be off
    /*
       Inputs:
         current_tempWstatus  98f 
         lower_desired_temp: (use 98f)
         upper_desired_temp: (use 100f)
         regulator_mode: Normal

       Expected Outputs:
         heat_control: Off
         last_cmd: Off
    */

    // generate values for the incoming data ports
    let current_temp = Temp { degrees: 98 };
    let lower_desired_temp = Temp { degrees: 98 };
    let upper_desired_temp = Temp { degrees: 100 };
    let desired_temp = Set_Points {lower: lower_desired_temp, upper: upper_desired_temp};
 
    // [PutInPorts]: put values on the input ports
    test_apis::put_current_temp(current_temp);
    test_apis::put_desired_temp(desired_temp);

    // [InvokeEntryPoint]: Invoke the entry point
    crate::thermostat_thermostat_timeTriggered();

    // get result values from output ports
    let api_heat_control = test_apis::get_heat_control();
    let lastCmd = test_apis::get_lastCmd();

    assert!(api_heat_control == On_Off::Off);
    assert!(lastCmd == On_Off::Off);

  }

  //. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .
  //  Illustrate explicit setting of last_cmd 
  //. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

  // Activation 1-a: Current temp lies within set points (closed interval)
  //   in this case current temp is equal to lower bound,
  //   so heat control should still be on
  /*
     Inputs:
      current_tempWstatus  98
      lower_desired_temp: (use 98)
      upper_desired_temp: (use 100)
      
    ** force previous heat_control lastCmd to ON **

     Expected Outputs:
      heat_control: On
      last_cmd: On
  */

  #[test]
  #[serial]
  fn test_compute_REQ_THERM_4_alt_1_a() {
    // [InvokeEntryPoint]: invoke the entry point test method
    crate::thermostat_thermostat_initialize();

    // generate values for the incoming data ports
    let current_temp = Temp { degrees: 98 };
    let lower_desired_temp = Temp { degrees: 98 };
    let upper_desired_temp = Temp { degrees: 100 };
    let desired_temp = Set_Points {lower: lower_desired_temp, upper: upper_desired_temp};
 
    // [PutInPorts]: put values on the input ports
    test_apis::put_current_temp(current_temp);
    test_apis::put_desired_temp(desired_temp);

    // set component internal state (last_cmd) to Onn
    test_apis::put_lastCmd(On_Off::Onn);

    // [InvokeEntryPoint]: Invoke the entry point
    crate::thermostat_thermostat_timeTriggered();

    // get result values from output ports
    let api_heat_control = test_apis::get_heat_control();
    let lastCmd = test_apis::get_lastCmd();

    assert!(api_heat_control == On_Off::Onn);
    assert!(lastCmd == On_Off::Onn);
  }

  // Activation 1-b: Current temp lies within set points (closed interval)
  //   in this case current temp is between lower bound and upper bound,
  //   so heat control should still be on
  /*
     Inputs:
       current_tempWstatus  99f  
       lower_desired_temp: (use 98f)
       upper_desired_temp: (use 100f)
       regulator_mode: Normal

       ** force previous heat_control lastCmd to ON **

     Expected Outputs:
       heat_control: On
       last_cmd: On
   */

  #[test]
  #[serial]
  fn test_compute_REQ_THERM_4_alt_1_b() {
    // [InvokeEntryPoint]: invoke the entry point test method
    crate::thermostat_thermostat_initialize();

    // generate values for the incoming data ports
    let current_temp = Temp { degrees: 99 };
    let lower_desired_temp = Temp { degrees: 98 };
    let upper_desired_temp = Temp { degrees: 100 };
    let desired_temp = Set_Points {lower: lower_desired_temp, upper: upper_desired_temp};
 
    // [PutInPorts]: put values on the input ports
    test_apis::put_current_temp(current_temp);
    test_apis::put_desired_temp(desired_temp);

    // set component internal state (last_cmd) to Onn
    test_apis::put_lastCmd(On_Off::Onn);

    // [InvokeEntryPoint]: Invoke the entry point
    crate::thermostat_thermostat_timeTriggered();

    // get result values from output ports
    let api_heat_control = test_apis::get_heat_control();
    let lastCmd = test_apis::get_lastCmd();

    assert!(api_heat_control == On_Off::Onn);
    assert!(lastCmd == On_Off::Onn);
  }

  // Activation 1-c: Current temp lies within set points (closed interval)
  //   in this case current temp is equal to upper bound,
  //   so heat control should still be on

  /*
     Inputs:
       current_tempWstatus  100f  
       lower_desired_temp: (use 98f)
       upper_desired_temp: (use 100f)
       regulator_mode: Normal

       ** force previous heat_control lastCmd to ON **

     Expected Outputs:
      heat_control: On
      last_cmd: On
  */

  #[test]
  #[serial]
  fn test_compute_REQ_THERM_4_alt_1_c() {
    // [InvokeEntryPoint]: invoke the entry point test method
    crate::thermostat_thermostat_initialize();

    // generate values for the incoming data ports
    let current_temp = Temp { degrees: 100 };
    let lower_desired_temp = Temp { degrees: 98 };
    let upper_desired_temp = Temp { degrees: 100 };
    let desired_temp = Set_Points {lower: lower_desired_temp, upper: upper_desired_temp};
 
    // [PutInPorts]: put values on the input ports
    test_apis::put_current_temp(current_temp);
    test_apis::put_desired_temp(desired_temp);

    // set component internal state (last_cmd) to Onn
    test_apis::put_lastCmd(On_Off::Onn);

    // [InvokeEntryPoint]: Invoke the entry point
    crate::thermostat_thermostat_timeTriggered();

    // get result values from output ports
    let api_heat_control = test_apis::get_heat_control();
    let lastCmd = test_apis::get_lastCmd();

    assert!(api_heat_control == On_Off::Onn);
    assert!(lastCmd == On_Off::Onn);
  }

  //. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .
  //  Illustrate use of helper functions for repeated patterns
  //. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

  fn test_compute_THERM_helper_wGSV(
    // container with GUMBO local state
    inputs: test_apis::PreStateContainer_wGSV,
    // expected result
    expected_ouput: On_Off)
  {
    // [InvokeEntryPoint]: invoke the entry point test method
    crate::thermostat_thermostat_initialize();

    // [PutInPorts]: put values on the input ports and set GUMBO state variables
    test_apis::put_concrete_inputs_container_wGSV(inputs);

    // [InvokeEntryPoint]: Invoke the entry point
    crate::thermostat_thermostat_timeTriggered();

    // get result values from output ports
    let api_heat_control = test_apis::get_heat_control();
    let lastCmd = test_apis::get_lastCmd();

    assert!(api_heat_control == expected_ouput);
    assert!(lastCmd == expected_ouput);
  }
  
  // Activation 2-a:
  //   Current temp lies within set points (closed interval)
  //   in this case current temp is equal to upper bound,
  //   so heat control should still be off

  /*
     Inputs:
       current_tempWstatus  100f  (validity: don't care (use Valid))
       lower_desired_temp: (use 98f)
       upper_desired_temp: (use 100f)
       regulator_mode: Normal

     ** force previous heat_control lastCmd to OFF **

     Expected Outputs:
        heat_control: Off
        last_cmd: Off
  */

  #[test]
  #[serial]
  fn test_compute_REQ_THERM_4_alt_2_a() {
    test_compute_THERM_helper_wGSV(
      test_apis::PreStateContainer_wGSV {
        In_lastCmd: On_Off::Off,  
        api_current_temp : Temp { degrees: 100},
        api_desired_temp : Set_Points {
          lower: Temp { degrees:  98 },
          upper: Temp { degrees: 100 }
        }
      },    
      On_Off::Off);  // expected
  }

  // Activation 2-b:
  //   Current temp lies within set points (closed interval)
  //   in this case current temp is between lower and upper bound,
  //   so heat control should still be off

  /*
     Inputs:
       current_tempWstatus  99f  (validity: don't care (use Valid))
       lower_desired_temp: (use 98f)
       upper_desired_temp: (use 100f)
       regulator_mode: Normal

    ** force previous heat_control lastCmd to OFF **

     Expected Outputs:
       heat_control: Off
  */

  #[test]
  #[serial]
  fn test_compute_REQ_THERM_4_alt_2_b() {
    test_compute_THERM_helper_wGSV(
      test_apis::PreStateContainer_wGSV {
        In_lastCmd: On_Off::Off,  
        api_current_temp : Temp { degrees: 99},
        api_desired_temp : Set_Points {
          lower: Temp { degrees:  98 },
          upper: Temp { degrees: 100 }
        }
      },    
      On_Off::Off); // expected
  }

  // Activation 2-c:
  //   Current temp lies within set points (closed interval)
  //   in this case current temp is equal to lower bound,
  //   so heat control should still be off
  /*
     Inputs:
       current_tempWstatus  98f 
       lower_desired_temp: (use 98f)
       upper_desired_temp: (use 100f)
       regulator_mode: Normal

    ** force previous heat_control lastCmd to OFF **

     Expected Outputs:
       heat_control: Off
  */

  #[test]
  #[serial]
  fn test_compute_REQ_THERM_4_alt_2_c() {
    test_compute_THERM_helper_wGSV(
      test_apis::PreStateContainer_wGSV {
        In_lastCmd: On_Off::Off,  
        api_current_temp : Temp { degrees: 98},
        api_desired_temp : Set_Points {
          lower: Temp { degrees:  98 },
          upper: Temp { degrees: 100 }
        }
      },    
      On_Off::Off); // expected
  }

}

//================================================================
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
//
//  M a n u a l l y - w r i t t e n    G U M B O X  (contract-based)  T e s t s
//
//  These examples illustrate how to use the GUMBOX APIs auto-generated
//  by HAMR to support contract-based unit testing for a HAMR thread component.
//  The test APIs are found in src/test/util/cb_apis.rs and are 
//  re-generated each time HAMR code generation is run (e.g., after
//  changes are made to model structures or model contracts).
//
//  When HAMR code generation is run, if a thread component has GUMBO
//  contracts declared for it, in addition to generating "logical"
//  Verus contracts for the application entry point methods, e.g., 
//  in `component/thermostat_process_thermostat_app.rs` to be verified
//  by Verus SMT-based verification, HAMR will generate "executable" versions
//  of the contracts as Rust boolean functions that can be called to 
//  support unit testing and run-time monitoring.  These boolean functions
//  can be found in the `bridge/<component_instance_name>_GUMBOX.rs` file
//  for a component.
//
//  There are two styles of unit testing supported:
//   - manual GUMBOX tests - in which the developer directly constructs
//     an input test vector, 
//   - automated property-based testing - in which HAMR generates infrastructure
//     to automatically generate random test vectors for testing the component.
//
//  In manual GUMBOX tests, code is written to 
//   - construct a set of component inputs, 
//  i.e., a "test vector" that provides a value
//  for each input port (and optionally, each GUMBO-specified component
//  variable (GSV))
//   - call the HAMR-generated `cb_apis::testComputeCB` method.
//     This method
//       - checks that the supplied test vector satisfies the pre-condition, 
//       - puts the values of the test vector into the component input ports
//       - invokes the compute entry point
//       - gets the values in the output ports of the test vector
//       - checks that the compute entry point post-condition on the 
//         output port values and input test vector values 
//         (recall that the post-condition, in addition to establishing 
//          constraints on output values, may also specify how output values 
//          are related to input values)
//       - returns a HarnessResult value, which has three alteratives
//             RejectedPrecondition - the input test vector does not satisfy 
//                 the precondition
//             FailedPostcondition(TestCaseError),
//               - the input test vector satisfies the precondition, but the 
//                 result of running the entry point DOES NOT satisfy the post condition
//             Passed
//               - both the pre- and post-condition are satisfied
//     In manual GUMBOX tests, one usually asserts that the HarnessResult 
//       is Passed
// 
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
//================================================================

mod GUMBOX_manual_tests {
  use serial_test::serial;
 
  use crate::test::util::*;
  use data::Isolette_Data_Model::*;

   //========================================================================
  //  REQ-THERM-2: If Current Temperature is less than the Lower Desired Temperature,
  //  the Heat Control shall be set to On.
  //========================================================================

     /*
       Inputs:
         current_temp:  95f 
         lower_desired_temp: 98f
         upper_desired_temp: *irrelevant to requirement* (use 100f)
      */

  #[test]
  #[serial]
  fn test_compute_GUMBOX_manual_REQ_THERM_2() {
    // generate values for the incoming data ports
    let current_temp = Temp { degrees: 95 };
    let lower_desired_temp = Temp { degrees: 98 };
    let upper_desired_temp = Temp { degrees: 100 };
    let desired_temp = Set_Points {lower: lower_desired_temp, upper: upper_desired_temp};

    let harness_result = 
       cb_apis::testComputeCB(current_temp, desired_temp);

    // ToDo: Jason: should we use #[derive(PartialEq)] for  HarnessResult 
    // assert!(harness_result == cb_apis::HarnessResult::Passed);
    assert!(matches!(harness_result, cb_apis::HarnessResult::Passed));
  }

  // There are also "container-based" variants (both without and with GUMBO
  // state variables) of the contract-based testing APIs as illustrated below.
  #[test]
  #[serial]
  fn test_compute_GUMBOX_manual_REQ_THERM_2_container() { // Alternate version: Illustrate "container"-based APIs
    // [InvokeEntryPoint]: invoke the entry point test method
    crate::thermostat_thermostat_initialize();

    // Inputs can be "bundled" into a container. 
    let preStateContainer = test_apis::PreStateContainer {
      api_current_temp : Temp { degrees: 95},
      api_desired_temp : Set_Points {
        lower: Temp { degrees:  98 },
        upper: Temp { degrees: 100 }
      }    
    };

    let harness_result = 
       cb_apis::testComputeCB_container(preStateContainer);
    assert!(matches!(harness_result, cb_apis::HarnessResult::Passed));
  }

  // Developers will occasionally supply a test vector that doesn't satsify 
  // the pre-condition.  This is represents a failed development process step:
  // the developer has not followed the methodology to design 
  // an appropriate test input OR the pre-condition has not been specified 
  // appropriately.
  //
  // The test below represents this type of error.

  /*
  #[test]
  #[serial]
  fn test_compute_GUMBOX_manual_REQ_THERM_2_failed_pre() {
    // generate values for the incoming data ports
    let current_temp = Temp { degrees: 95 };
    let lower_desired_temp = Temp { degrees: 102 }; // wrong: lower value shoud be LEQ to upper
    let upper_desired_temp = Temp { degrees: 100 };
    let desired_temp = Set_Points {lower: lower_desired_temp, upper: upper_desired_temp};

    let harness_result = 
       cb_apis::testComputeCB(current_temp, desired_temp);

    // HarnessResult::RejectedPrecondition is returned to harness_result, 
    // which causes the assertion below to fail.
    assert!(matches!(harness_result, cb_apis::HarnessResult::Passed));
  }
  */

 // EXERCISES:
 //  - construct a GUMBOX manual test for REQ-THERM-3, making direct use of APIs
 //    without a container
 //  - construct an alternate GUMBOX manual test for REQ-THERM-3, 
 //    using a container structure 
  
 
 //========================================================================
 //  REQ-THERM-4: If the Current Temperature is greater than or equal
 //  to the Lower Desired Temperature and less than or equal to the
 //  Upper Desired Temperature, the value of the Heat Control shall not be changed.":
 //========================================================================

  // Test design notes:
  // The _CB based tests re-initialize the internal state of the component
  // (including the GUMBO state variables) during each call.  Therefore, 
  // we cannot use them to implement the testing approach in which we 
  // "inherit" the values of GSV from previous invocations.  Instead, 
  // we following the approach in which we explicitly force the GSV pre-state
  // variable values to specific values on each test invocation.
  //

  // Activation 1-a: Current temp lies within set points (closed interval)
  //   in this case current temp is equal to lower bound,
  //   so heat control should still be on
  /*
     Inputs:
      current_tempWstatus  98
      lower_desired_temp: (use 98)
      upper_desired_temp: (use 100)
      
    ** force previous heat_control lastCmd to ON **

     Expected Outputs:
      heat_control: On
      last_cmd: On
  */

  #[test]
  #[serial]
  fn test_compute_GUMBOX_manual_1_a() {
    // [InvokeEntryPoint]: invoke the entry point test method
    crate::thermostat_thermostat_initialize();

    // generate values for the incoming data ports
    let current_temp = Temp { degrees: 98 };
    let lower_desired_temp = Temp { degrees: 98 };
    let upper_desired_temp = Temp { degrees: 100 };
    let desired_temp = Set_Points {lower: lower_desired_temp, upper: upper_desired_temp};
    let last_cmd = On_Off::Onn;

    let harness_result = 
       cb_apis::testComputeCBwGSV(last_cmd, current_temp, desired_temp);

    assert!(matches!(harness_result, cb_apis::HarnessResult::Passed));
  }

  // Exercise
  //
  //  Complete the remaining manual GUMBOX tests for REQ-THERM-4
  //  corresponding to original manual tests given previously.

}


//================================================================
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
//
//  A u t o m a t e d    G U M B O X  (contract-based)  T e s t s
//  (property-based testing)

//  These examples illustrate how to use the GUMBOX APIs auto-generated
//  by HAMR to support automated property-based contract-based unit testing 
//  for a HAMR thread component.
// 
//  The test APIs are found in src/test/util/cb_apis.rs and are 
//  re-generated each time HAMR code generation is run (e.g., after
//  changes are made to model structures or model contracts).
//
//  The auto-generated file includes macros for 
//  automated property-based testing - in which HAMR uses the
//  Rust prop test libraries (https://altsysrq.github.io/proptest-book/)
//  to automatically generate random test vectors for testing the component.
//
//  In automated GUMBOX tests, code is written to 
//   - configure the control of random value generation and 
//     test running.  This includes aspects such as 
//     indicating the desired number of tests.
//   - specifying the random value generators to be used for 
//     each input value (input port and GUMBO-specified variable).
//     HAMR generates default generators for each data type used 
//     in inputs in the `test/util/generators.rs` file.
//
//  The auto-generated artifacts for the framework can be used immediately
//  by developers for automated testing that complements contract-based 
//  verification with Verus.
//
//  However, The overall objective in this assurance approarch is to 
//  configure the input generation framework in a manner that yields
//  desired coverage of both entry point code and entry point contracts.
//
//  Thus effective use of the framework that goes beyond simple "tire kicking"
//  requires that developers know how to...
//   .. obtain and read coverage information for both code and contracts
//   .. configurate the generation frameworo.
//  
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
//================================================================

mod GUMBOX_tests {
  use serial_test::serial;

  // import the proptest libraries used for random value generation 
  //  and broader property-based testing concepts
  use proptest::prelude::*;

  // import HAMR-generated utilities for applying property-based testing 
  // for testing component entry points with "properties" phrased as GUMBOX contracts.
  use crate::test::util::*;
  use crate::testInitializeCB_macro;
  use crate::testComputeCB_macro;
  use crate::testComputeCBwGSV_macro;

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

  // testComputeCB_macro! {
  //   prop_testComputeCB_macro, // test name
  //   config: ProptestConfig { // proptest configuration, built by overriding fields from default config
  //     cases: numValidComputeTestCases,
  //     max_global_rejects: numValidComputeTestCases * computeRejectRatio,
  //     verbose: verbosity,
  //     ..ProptestConfig::default()
  //   },
  //   // strategies for generating each component input
  //   api_current_temp: generators::Isolette_Data_Model_Temp_strategy_default(),
  //   api_desired_temp: generators::Isolette_Data_Model_Set_Points_strategy_default()
  // }

  // testComputeCBwGSV_macro! {
  //   prop_testComputeCBwGSV_macro, // test name
  //   config: ProptestConfig { // proptest configuration, built by overriding fields from default config
  //     cases: numValidComputeTestCases,
  //     max_global_rejects: numValidComputeTestCases * computeRejectRatio,
  //     verbose: verbosity,
  //     ..ProptestConfig::default()
  //   },
  //   // strategies for generating each component input
  //   In_lastCmd: generators::Isolette_Data_Model_On_Off_strategy_default(),
  //   api_current_temp: generators::Isolette_Data_Model_Temp_strategy_default(),
  //   api_desired_temp: generators::Isolette_Data_Model_Set_Points_strategy_default()
  // }

  testComputeCBwGSV_macro! {
    prop_testComputeCBwGSV_REQ2thru4, // test name
    config: ProptestConfig { // proptest configuration, built by overriding fields from default config
      cases: numValidComputeTestCases,
      max_global_rejects: numValidComputeTestCases * computeRejectRatio,
      verbose: verbosity,
      ..ProptestConfig::default()
    },
    // Replace the default strategies with strategies for custom ranges
    In_lastCmd: generators::Isolette_Data_Model_On_Off_strategy_default(),
    api_current_temp: generators::Isolette_Data_Model_Temp_strategy_cust(94..=105),
    api_desired_temp: generators::Isolette_Data_Model_Set_Points_strategy_cust(
      generators::Isolette_Data_Model_Temp_strategy_cust(94..=103),
      generators::Isolette_Data_Model_Temp_strategy_cust(97..=105)
    )
  }
}
