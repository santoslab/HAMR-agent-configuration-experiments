//@ HAMR: --platform Microkit --output-dir ../../hamr

/*============================================================
    SNG   S y s t e m

    Top-level system integrating the SNG Guard pipeline
    (Gate and Filter) with test infrastructure (TestSender
    and TestReceiver) for end-to-end simulation and testing.

    The Guard pipeline stages (Gate, Filter) represent the
    deployable network guard. The test components exercise
    it by sending messages in and observing messages out.

    Note: HAMR does not currently support nested System
    definitions, so all processes are at the same system level.
    The logical separation between guard and test harness is
    maintained through naming conventions, comments, and
    package organization (guard threads in SNG_Software,
    test threads in SNG_Software).
  ============================================================*/

package SNG {
    private import HAMR::*;
    private import SNG_Software::*;
    private import SNG_Data_Model::*;

    //===============================================================================
    //
    //  SNG System
    //
    //  Data flow pipeline:
    //    TestSender -> Gate -> Filter -> TestReceiver
    //
    //===============================================================================
    part def SNG_System :> System {
        //-- Guard pipeline (deployable) --
        part gate : Gate_Process;
        part msg_filter : Filter_Process;

        //-- Test infrastructure --
        part test_sender : TestSender_Process;
        part test_receiver : TestReceiver_Process;

        //-- Hardware --
        part sng_processor : Platform::SNG_Processor;

        //-- Connections (pipeline) --
        connection ts_to_gate : PortConnection connect
            test_sender.output to gate.input;
        connection gate_to_filter : PortConnection connect
            gate.output to msg_filter.input;
        connection filter_to_tr : PortConnection connect
            msg_filter.output to test_receiver.input;

        //-- Allocations --
        allocation pb0 : Deployment_Properties::Actual_Processor_Binding
            allocate test_sender to sng_processor;
        allocation pb1 : Deployment_Properties::Actual_Processor_Binding
            allocate gate to sng_processor;
        allocation pb2 : Deployment_Properties::Actual_Processor_Binding
            allocate msg_filter to sng_processor;
        allocation pb3 : Deployment_Properties::Actual_Processor_Binding
            allocate test_receiver to sng_processor;
    }

    //===============================================================================
    //
    //  G a t e   Process  (partition -- guard pipeline stage 1)
    //
    //===============================================================================
    part def Gate_Process :> Process {
        //-- Interface --
        port input : EventDataPort { in :>> type : Message; }
        port output : EventDataPort { out :>> type : Message; }

        //-- Properties --
        attribute :>> Domain = SNG_Properties::Domain_Gate;

        //-- Internals --
        part gate : Gate;
        connection ci : PortConnection connect input to gate.input;
        connection co : PortConnection connect gate.output to output;
    }

    //===============================================================================
    //
    //  F i l t e r   Process  (partition -- guard pipeline stage 2)
    //
    //===============================================================================
    part def Filter_Process :> Process {
        //-- Interface --
        port input : EventDataPort { in :>> type : Message; }
        port output : EventDataPort { out :>> type : Message; }

        //-- Properties --
        attribute :>> Domain = SNG_Properties::Domain_Filter;

        //-- Internals --
        part msg_filter : Filter;
        connection ci : PortConnection connect input to msg_filter.input;
        connection co : PortConnection connect msg_filter.output to output;
    }

    //===============================================================================
    //
    //  T e s t S e n d e r   Process  (partition -- test harness)
    //
    //===============================================================================
    part def TestSender_Process :> Process {
        //-- Interface --
        port output : EventDataPort { out :>> type : Message; }

        //-- Properties --
        attribute :>> Domain = SNG_Properties::Domain_TestSender;

        //-- Internals --
        part test_sender : TestSender;
        connection c : PortConnection connect test_sender.output to output;
    }

    //===============================================================================
    //
    //  T e s t R e c e i v e r   Process  (partition -- test harness)
    //
    //===============================================================================
    part def TestReceiver_Process :> Process {
        //-- Interface --
        port input : EventDataPort { in :>> type : Message; }

        //-- Properties --
        attribute :>> Domain = SNG_Properties::Domain_TestReceiver;

        //-- Internals --
        part test_receiver : TestReceiver;
        connection c : PortConnection connect input to test_receiver.input;
    }
}
