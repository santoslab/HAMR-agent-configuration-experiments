// This file will not be overwritten if codegen is rerun

mod tests {
  // NOTE: need to run tests sequentially to prevent race conditions
  //       on the app and the testing apis which are static
  use serial_test::serial;

  use crate::test::util::*;
  use data::*;

  #[test]
  #[serial]
  fn test_initialization() {
    crate::heat_source_heat_source_initialize();
}

  #[test]
  #[serial]
  fn test_compute() {
    crate::heat_source_heat_source_initialize();

    // populate incoming data ports
    test_apis::put_heat_control(Isolette_Data_Model::On_Off::default());

    crate::heat_source_heat_source_timeTriggered();
  }
}
