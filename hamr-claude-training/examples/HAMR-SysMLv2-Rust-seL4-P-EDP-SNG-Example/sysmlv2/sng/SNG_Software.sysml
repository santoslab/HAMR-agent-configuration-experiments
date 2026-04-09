/*============================================================
    SNG   S o f t w a r e

    Defines the software components (threads) used in the
    Simple Network Guard system.

    Core pipeline components:
      - Gate: drops Critical messages, passes others
      - Filter: clamps Restricted message payloads to [0,100],
                passes Public messages unchanged

    Test harness components:
      - TestSender: generates test messages
      - TestReceiver: receives and logs processed messages
  ============================================================*/

package SNG_Software {
    private import HAMR::*;
    private import SNG_Data_Model::*;
    private import GumboLib::*;

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
    //    - Critical messages are dropped (Req_C)
    //    - Restricted and Public messages are passed through (Req_R_1, Req_P)
    //
    //===============================================================================
    part def Gate :> Periodic_Rust_Thread {
        //-- Interface --
        port input : EventDataPort { in :>> type : Message; }
        port output : EventDataPort { out :>> type : Message; }

        //-- Behavior Constraints --
        //  Gate uses guarantee-only clauses with "implies" to illustrate
        //  how implies can give the same semantics as case assume/guarantee pairs.
        //  HasEvent/NoEvent guard port access; short-circuit "and" prevents
        //  erroneous access of an empty port.
        language "GUMBO" /*{
            integration
                guarantee No_Critical_Output:
                    GumboLib::GUMBO__Library::allowedSecurityLevel(output);

            compute
                guarantee Req_C_Drop_Critical:
                    HasEvent(input) and (input.security_level == SNG_Data_Model::SecurityLevel.Critical)
                    implies NoSend(output);

                guarantee Req_R1_Pass_Restricted:
                    HasEvent(input) and (input.security_level == SNG_Data_Model::SecurityLevel.Restricted)
                    implies HasEvent(output) and GumboLib::GUMBO__Library::equalMessage(input, output);

                guarantee Req_P_Pass_Public:
                    HasEvent(input) and (input.security_level == SNG_Data_Model::SecurityLevel.Public)
                    implies HasEvent(output) and GumboLib::GUMBO__Library::equalMessage(input, output);

                guarantee No_Input_No_Output:
                    (not HasEvent(input)) implies NoSend(output);
        }*/
    }

    //===============================================================================
    //
    //  F i l t e r   Thread
    //
    //  Implements payload sanitization:
    //    - Public messages pass unchanged (Req_P)
    //    - Restricted message payloads are clamped to [0,100] (Req_R_2)
    //
    //===============================================================================
    part def Filter :> Periodic_Rust_Thread {
        //-- Interface --
        port input : EventDataPort { in :>> type : Message; }
        port output : EventDataPort { out :>> type : Message; }

        //-- Behavior Constraints --
        //  Filter uses compute_cases with case assume/guarantee pairs
        //  to contrast with Gate's guarantee-only style.
        //  Integration assume uses library function to express the
        //  pipeline invariant established by Gate upstream.
        language "GUMBO" /*{
            integration
                assume No_Critical_Input:
                    GumboLib::GUMBO__Library::allowedSecurityLevel(input);

            compute
              compute_cases
                case Req_P_Public_Pass:
                    assume HasEvent(input) and (input.security_level == SNG_Data_Model::SecurityLevel.Public);
                    guarantee HasEvent(output) and GumboLib::GUMBO__Library::equalMessage(input, output);

                case Req_R2_Restricted_Clamp:
                    assume HasEvent(input) and (input.security_level == SNG_Data_Model::SecurityLevel.Restricted);
                    guarantee HasEvent(output)
                        and GumboLib::GUMBO__Library::equalSecurityLevel(input, output)
                        and GumboLib::GUMBO__Library::clampedPayload(output)
                        and (GumboLib::GUMBO__Library::clampedPayload(input) implies GumboLib::GUMBO__Library::equalPayload(input, output));

                case No_Input:
                    assume (not HasEvent(input));
                    guarantee NoSend(output);
        }*/
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
