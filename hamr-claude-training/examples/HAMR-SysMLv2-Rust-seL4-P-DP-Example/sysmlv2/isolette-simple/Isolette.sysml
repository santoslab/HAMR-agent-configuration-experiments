//@ HAMR: --platform Microkit --output-dir ../../hamr

package Isolette {

  private import HAMR::*;
  private import Isolette_Software::*;

  //===============================================================================
  // 
  //  I s o l e t t e   S y s t e m 
  // 
  //  Top-level architecture definition integrating software and hardware and 
  //  specifying the partitioning and the system.
  //
  //===============================================================================
  part def Isolette_System :> System {
    //-- I n t e r n a l s --
    // ..indicate the processes (partitions) of the system
    part temp_sensor: TempSensor_Process;
    part operator_interface: OperatorInterface_Process;
    part thermostat: Thermostat_Process;
    part heat_source: HeatSource_Process;
    part isolette_processor : Platform::Processor_1;

    // connections (information flows) between partitions
    connection ct : PortConnection connect temp_sensor.current_temp to thermostat.current_temp;
    connection hc : PortConnection connect thermostat.heat_control to heat_source.heat_control;
    connection sp : PortConnection connect operator_interface.desired_temp to thermostat.desired_temp;

    //-- A l l o c a t i o n s  of software/partitions to hardware
    allocation pb0: Deployment_Properties::Actual_Processor_Binding 
      allocate temp_sensor to isolette_processor;
    allocation pb1: Deployment_Properties::Actual_Processor_Binding 
      allocate operator_interface to isolette_processor;
    allocation pb2: Deployment_Properties::Actual_Processor_Binding 
      allocate thermostat to isolette_processor;
    allocation pb3: Deployment_Properties::Actual_Processor_Binding 
      allocate heat_source to isolette_processor;
  }

  //===============================================================================
  // 
  //  T e m p S e n s o r   Process  (partition)
  // 
  //===============================================================================
  part def TempSensor_Process :> Process {
     //-- I n t e r f a c e  --
    port current_temp : DataPort { out :>> type : Isolette_Data_Model::Temp; }

    //-- P r o p e r t i e s  --
    attribute :>> Domain = Isolette_Properties::Domain_TS;

    //-- I n t e r n a l s --
    part temp_sensor: Devices::TempSensor;        
    connection ct : PortConnection connect temp_sensor.current_temp to current_temp;
  }

  //===============================================================================
  // 
  //  H e a t S o u r c e   Process  (partition)
  // 
  //===============================================================================
  part def HeatSource_Process :> Process {
    //-- I n t e r f a c e  --
    port heat_control : DataPort { in :>> type : Isolette_Data_Model::On_Off; }

    //-- P r o p e r t i e s  --
    attribute :>> Domain = Isolette_Properties::Domain_HS;

    //-- I n t e r n a l s --
    part heat_source: Devices::HeatSource; 
    connection hc : PortConnection connect heat_control to heat_source.heat_control;
  }

  //===============================================================================
  // 
  //  O p e r a t o r   I n t e r f a c e    Process  (partition)
  // 
  //===============================================================================
  part def OperatorInterface_Process :> Process {
    //-- I n t e r f a c e  --
    port desired_temp : DataPort { out :>> type : Isolette_Data_Model::Set_Points; }

    //-- P r o p e r t i e s  --
    attribute :>> Domain = Isolette_Properties::Domain_OI;

    //-- I n t e r n a l s --
    part operator_interface: OperatorInterface; 
    connection sp : PortConnection connect operator_interface.desired_temp to desired_temp;
  }

  //===============================================================================
  // 
  //  T h e r m o s t a t    Process  (partition)
  // 
  //===============================================================================
  part def Thermostat_Process :> Process {
    //-- I n t e r f a c e  --
    port current_temp : DataPort { in :>> type : Isolette_Data_Model::Temp; }
    port desired_temp : DataPort { in :>> type : Isolette_Data_Model::Set_Points; }
    port heat_control : DataPort { out :>> type : Isolette_Data_Model::On_Off; }

    attribute :>> Domain = Isolette_Properties::Domain_THERM;

    //-- I n t e r n a l s --
    part thermostat: Thermostat;
    connection ct : PortConnection connect current_temp to thermostat.current_temp;
    connection hc : PortConnection connect thermostat.heat_control to heat_control;
    connection sp : PortConnection connect desired_temp to thermostat.desired_temp;
  }
}