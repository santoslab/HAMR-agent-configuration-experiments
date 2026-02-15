/*============================================================
    Isolette   S o f t w a r e 

    Defines the software components (threads) used in the 
    Isolette system.  

    Define the software elements in a separate package enables
    us to more easily reuse them and assemble them in different
    ways in different systems as we deploy to different platforms.

  ============================================================*/
 
package Isolette_Software {
  private import HAMR::*;

  //===============================================================================
  // 
  //  S t y l e s  (optional)
  // 
  //  Define common styles/patterns of software to be used.
  //  In a larger development context, these might be managed resources of a 
  //  particular organization's development process.
  //===============================================================================
  part def Periodic_Thread :> Thread {
    attribute :>> Period = 1000 [ms];  // default period for Isolette threads
    attribute :>> Dispatch_Protocol = Supported_Dispatch_Protocols::Periodic;
  }

  part def Periodic_Rust_Thread :> Periodic_Thread {
     attribute :>> Implementation_Language = Implementation_Languages::Rust;
  }

  //===============================================================================
  // 
  //  T h e r m o s t a t   Thread  
  // 
  //===============================================================================
  part def Thermostat :> Periodic_Rust_Thread {
    //-- I n t e r f a c e  --
    in port current_temp : DataPort { :>> type : Isolette_Data_Model::Temp; }
    in port desired_temp : DataPort { :>> type : Isolette_Data_Model::Set_Points; }
    out port heat_control : DataPort { :>> type : Isolette_Data_Model::On_Off; }
    
    //-- B e h a v i o r    C o n s t r a i n t s --
    language "GUMBO" /*{
      state
         lastCmd: Isolette_Data_Model::On_Off;
            
      functions
      //  -- desired bounds on temperature values in the thermostat
      def Temp_Lower_Bound(): Base_Types::Integer_32 := 95 [i32];
      def Temp_Upper_Bound(): Base_Types::Integer_32 := 104 [s32];
          
	  //-- == I n t e g r a t i o n     C o n s t r a i n t s	
	  integration
  	    assume ASSM_CT_Range: 
  	    //-- Literals used in spec below
  	      Temp_Lower_Bound() <= current_temp.degrees & current_temp.degrees <= Temp_Upper_Bound();


  		//--  == I n i t i a l i z e    E n t r y    P o i n t    Behavior Constraints  ==
        initialize
	      guarantee  //-- note: anonymous guarantee clause
		    initlastCmd: lastCmd == Isolette_Data_Model::On_Off.Off;
		  guarantee REQ_THERM_1 "The Heat Control command shall be Off initially.":
		    heat_control == Isolette_Data_Model::On_Off.Off;
		
		//--  ====== C o m p u t e    E n t r y    P o i n t   Behavior Constraints =====      
		  compute
        // -- we assume that Operator Interface enforces the contraint below on incoming 
        //    set points (desired temperature range)
        assume ASSM_LDT_LE_UDT: desired_temp.lower.degrees <= desired_temp.upper.degrees;
			
		    //-- the lastCmd state variable is always equal to the value of the heat_control output port
		    guarantee lastCmd "Set lastCmd to value of output Cmd port":
			  lastCmd == heat_control;
			
		    compute_cases
			  case REQ_THERM_2 "If Current Temperature is less than
                                |the Lower Desired Temperature, the Heat Control shall be set to On.":
			
			    assume (current_temp.degrees < desired_temp.lower.degrees);
			    guarantee heat_control == Isolette_Data_Model::On_Off.Onn;
			
    		  case REQ_THERM_3 "If the Current Temperature is greater than
                                |the Upper Desired Temperature, the Heat Control shall be set to Off.":
			    assume (current_temp.degrees > desired_temp.upper.degrees);
			    guarantee heat_control == Isolette_Data_Model::On_Off.Off;
			
			  case REQ_THERM_4 "If the Current Temperature is greater than or equal 
                                |to the Lower Desired Temperature
                                |and less than or equal to the Upper Desired Temperature, the value of
                                |the Heat Control shall not be changed.":
			    assume (current_temp.degrees >= desired_temp.lower.degrees
				    	& current_temp.degrees <= desired_temp.upper.degrees); 
			    guarantee heat_control == In(lastCmd);
        }*/
    }

    //===============================================================================
    // 
    //  O p e r a t o r   I n t e r f a c e  Thread  
    // 
    //===============================================================================
    part def OperatorInterface :> Periodic_Rust_Thread {
      //-- I n t e r f a c e  --
      out port desired_temp : DataPort { :>> type : Isolette_Data_Model::Set_Points; }
    }
}