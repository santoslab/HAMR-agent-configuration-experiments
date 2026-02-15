# Technical Approach of HAMR Execution Semantics

Many of AADL's thread execution concepts are based on long-established
task patterns and principles for achieving analyzeable real-time
systems (such as found in Burns and Wellings "Analysable Real-Time Systems").  
Burns and Wellings described these principles for Ada, but we implement similar notions in Slang and Rust.  Following these principles, at each activation of a thread, 
the application code of the thread (it's Compute entry point) 
will abstractly compute a function from its input port
values and local variables to output port values while possibly
updating its local variables.

In the general case, an AADL Thread may explictly call the 
AADL run-time services (RTS) to receive new inputs at any point in its execution.
Yet, this would break the atomicity of a dispatch
execution. We explictly forbid this in our formalization as this would
introduce unsoundess in many AADL analyses and contract languages such as GUMBO.
Furthermore, this capability is barely used in practice.

In AADL terminology, dispatching a thread refers to the thread
becoming ready for execution from a OS scheduler perspective.  The
thread Dispatch_Protocol property selects among several
strategies for determining when a thread should be dispatched.  In
HAMR, we consider only *Periodic*, which dispatches a thread
when a certain time interval is passed, and Sporadic, which
dispatches a thread upon arrival of messages to input ports specified
as *dispatch triggers*.  

In descriptions of HAMR's semantics, the state of each port is further decomposed into the
Infrastructure Port State (IPS) and the Application Port State
(APS). The IPS represents the communication infrastructure's
perspective of the port; this is hidden from the thread application code.
The APS represents the thread application code's perspective of the port.

HAMR follows AADL and supports an input-compute-output model of communication and execution that is based on threads and port communication. To ensure determinism, the inputs received from other components are frozen at a specified point, by default the dispatch of a thread. As a result the computation performed by a thread is not affected by the arrival of new input unless the thread explicitly requests for input. Similarly, the output is made available to other components at a specified point in time. 

The distinction between IPS and APS is used to represent AADL's
notions of port freezing and port variable.  Typically, when a thread is
dispatched, the component infrastructure uses the RTS to move one or more values from the IPS of
input ports into the input APS.  Then the component application code
is called and the APS values then remain ``frozen'' as the code
executes.  This provides the application a consistent view of inputs
even though input IPS may be concurrently updated by communication
infrastructure behind the scenes.  The application code writes to the
output APS throughout execution.  Our intended design for this is that
when the application code completes, the component infrastructure will
call the Send Output RTS to move output values from the
output APS to the IPS, thus releasing the output values all at once to
the communication infrastructure for propagation to consumers. This
release of output values is the key desired behavior. There are
multiple possible implementations that achieve this behavior.  At the
component's external interface, this execution pattern follows the
(Read Inputs; Compute; Write Outputs) structure championed by
various real-time system methods.

For input event data ports, the IPS typically would be a queue into
which the middleware would insert arriving values following overflow
policies specified for the port.  For input data ports, the IPS
typically would be a memory block large enough to hold a single value.
For output ports, the IPS represents pending value(s) to be propagated
by the communication infrastructure to connected consumer ports.

The AADL standard indicates that a thread's application code is
organized into entry points (e.g., subprograms that are invoked from
the AADL run-time).  For example, the Initialize Entry Point
is called during the system's initialization phase,
the Compute Entry Point is called during the
system's ``normal'' compute phase.  Other entry points are defined for
handling faults, performing mode changes, etc.  We plan to address the
higher-level coordination semantics for these phases in follow-on
work.  Multiple organizations of entry points are allowed.  Here, we
address a single and for each thread.

