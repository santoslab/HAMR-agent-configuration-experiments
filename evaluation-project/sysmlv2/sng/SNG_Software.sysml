/*============================================================
    SNG   S o f t w a r e

    Defines the software components (threads) used in the
    Simple Network Guard system.

    Core pipeline components:
      - Gate: drops TopSecret messages, passes others
      - Filter: clamps Secret message payloads to [0,100],
                passes Public messages unchanged

    Test harness components:
      - TestSender: generates test messages
      - TestReceiver: receives and logs processed messages
  ============================================================*/

package SNG_Software {
    private import HAMR::*;
    private import SNG_Data_Model::*;

    //=== Common thread style ===
    part def Periodic_Rust_Thread :> Thread {
        attribute :>> Period = 1000 [ms];
        attribute :>> Dispatch_Protocol = Supported_Dispatch_Protocols::Periodic;
        attribute :>> Implementation_Language = Implementation_Languages::Rust;
    }

    //===============================================================================
    //
    //  G a t e   Thread
    //
    //  Implements message drop/pass policies:
    //    - TopSecret messages are dropped (Req_TS)
    //    - Secret and Public messages are passed through (Req_S_1, Req_P)
    //
    //===============================================================================
    part def Gate :> Periodic_Rust_Thread {
        //-- Interface --
        port input : EventDataPort { in :>> type : Message; }
        port output : EventDataPort { out :>> type : Message; }

        //-- Behavior Constraints --
        //  GUMBO integration constraint: messages that pass through
        //  the Gate are never TopSecret.
        language "GUMBO" /*{
            integration
                guarantee No_TopSecret_Output:
                    output.security_level != SNG_Data_Model::SecurityLevel.TopSecret;
        }*/

        //  Compute contracts for Gate (Req_TS, Req_S_1, Req_P) will be
        //  added once GUMBO semantics for EventDataPort option types
        //  (message present vs. absent) are clarified.
    }

    //===============================================================================
    //
    //  F i l t e r   Thread
    //
    //  Implements payload sanitization:
    //    - Public messages pass unchanged (Req_P)
    //    - Secret message payloads are clamped to [0,100] (Req_S_2)
    //
    //===============================================================================
    part def Filter :> Periodic_Rust_Thread {
        //-- Interface --
        port input : EventDataPort { in :>> type : Message; }
        port output : EventDataPort { out :>> type : Message; }

        //-- Behavior Constraints --
        //  GUMBO integration constraint: the Filter only receives
        //  non-TopSecret messages (guaranteed by Gate upstream).
        language "GUMBO" /*{
            integration
                assume No_TopSecret_Input:
                    input.security_level != SNG_Data_Model::SecurityLevel.TopSecret;
        }*/

        //  Compute contracts for Filter (Req_S_2, Req_P) will be
        //  added once GUMBO semantics for EventDataPort option types
        //  (message present vs. absent) are clarified.
    }

    //===============================================================================
    //
    //  T e s t S e n d e r   Thread
    //
    //  Generates test messages with varying security levels and payloads
    //  to exercise the guard pipeline.
    //
    //===============================================================================
    part def TestSender :> Periodic_Rust_Thread {
        //-- Interface --
        port output : EventDataPort { out :>> type : Message; }
    }

    //===============================================================================
    //
    //  T e s t R e c e i v e r   Thread
    //
    //  Receives and logs messages that have passed through the guard pipeline.
    //
    //===============================================================================
    part def TestReceiver :> Periodic_Rust_Thread {
        //-- Interface --
        port input : EventDataPort { in :>> type : Message; }
    }
}
