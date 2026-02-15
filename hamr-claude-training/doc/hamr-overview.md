# HAMR Overview

HAMR is a model-based development for developing high-assurance safety- and security-critical systems.  HAMR is part of the Sireum framework (https://sireum.org). Developers define the architecture of their systems in a subset of SysMLv2 that is designed to support concepts from AADL.  Overall themes of HAMR:
  - provide developer-friendly specifications, testing, and formal verification that is integrated across models and code
  - provide a variety of forms of reporting and assurance evidence that can be used by independent auditors to confirm that the deployed system meets its specifications
  - support development of system application logic in multiple programming languages including Rust, C, and the Slang safety-critical subset of Scala developed as part of the Sireum framework from Kansas State University
  - support deployment on multiple platforms and operating systems including the JVM and the seL4 verified microkernel using the Microkit libraries

The developer edits SysMLv2 models in a customization of the VSCodium tool called CodeIVE (which we assume is already installed by the developer when installing the Sireum framework). CodeIVE uses the SysIDE VSCode extension to support editing of SysMLv2 models.  HAMR provides functions for checking model well-formedness, model analysis, code generation configuration, and code generation.   These functions are exposed in CodeIVE command palette, through CLI interfaces, and through MCP interfaces.  The developer codes, tests, and debugs application logic in any text editor, but most likely the CodeIVE.

HAMR emphasizes component-based architectures.  Systems are built from components with ports.  Connections between ports of different components indicate uni-directional communication between those component via the connected ports.

Components represent AADL Thread components are leaf nodes in the architecture.  These represent sporadic or periodic real-time tasks.  These follow a read - compute - write style (see @technical-approach.md for details).  The code in these tasks is organized into entry points that are invoked by the underlying scheduling framework.  

# Workflow Concepts

## Modeling 

Developers first design the architecture of their systems using SysMLv2.  Models must be constructed from a specific subset of SysMLv2 the uses concepts from AADL.  This is subset is referred to as "HAMR SysML".  HAMR SysML AADL concepts are captured using SysMLv2 definitions in  accompanying library packages (`hamr.aadl.library` and `aadl.library`).   When using the CodeIVE for editing models the SysMLv2 extension checks basic well-formedness of SysMLv2 according to the standardized rules of SysMLv2.  The developer runs the HAMR Type Checking function to additionally check that models lie within the AADL-concept subset supported by HAMR.  This function also provides other forms of type checking and structure checking.  When with a developer or Claude edits models it will typically want to run HAMR Type Checking to confirm that it has produced models that lie within the HAMR subset.  In addition to be available in the CodeIVE command palette, it is available via a `sireum hamr` command-line option as well as an MCP interface.

Models are developed incrementally, gradually adding information about data types, component attributes, etc.  Basic HAMR SysML patterns for components, component integration, and data types are illustrated in @hamr-sysml-patterns.

The developer may add formal specifications in the form of behavior contracts to Thread components using the GUMBO contract language.


## Configuring Code Generation Options

When invoking HAMR code generation, arguments are passed to indicate the location of generated files and other options.  When using CLI or MCP, these arguments can be supplied directly.  When invoking in the CodeIVE, configuration options are defined as comments as the top of the main model file.

Here is an example of such a comment.
```
//@ HAMR: --platform Microkit --output-dir ../../hamr
```
We refer to these as code generation "configuration specifications".  When code generation is run, it uses this information to configure its actions.  In particular, from the configuration specification comment, we can see that this is a configuration indicating that code generation should produce code for the seL4 Microkit platform, and that the resulting code should go in the `hamr` folder indicated by the given relative path.  Within the given `output-dir` folder, by default, HAMR places code in a subfolder with a name associated with the target platform.  So for the example above, generate code will go in the `../../hamr/microkit` folder.

## Running Code Generation 

When HAMR code generation first runs, it will place code files, makefiles, and other build artifacts in the configured `output-dir` organized in a hierarchy of folders (see `doc/folder-structure`).  This is typically in a folder named `hamr`.  The folder stucture varies based on the programming language and target platform used in HAMR code generation.  

Some of these files are always auto-generated by HAMR code generation each time it is run.  These files are marked with the following comment at the top of the file
```
// Do not edit this file as it will be overwritten if codegen is rerun
```
Such files generally implement system infrastructure that can be completely determine from the model.

Other files are designed to be edited by the user.  These typically hold the application logic for components marked as AADL threads in the models.  These files are marked with the following comment at the top of the file
```
// This file will not be overwritten if codegen is rerun
```
In the initial code generation, these files will contain boiler plate code and templates/skeletons for methods that the developer completes to code application logic.  There are some parts of these files that contain formal specifications (contracts), variable declarations, and other information that HAMR generates from model information.  These sections are marked with special comment delimiters.  
```
// BEGIN MARKER STATE VAR INIT
  (code here should not be modified by the developer -- HAMR auto-generates it and auto-updates it)
// END MARKER STATE VAR INIT
```
As models are updated, e.g., with new formal specifications, as code generation is rerun, HAMR will not overwrite the entire file.  It will however, attempt to automatically weave updated specifications into the file in the regions marked with the special comments.

## Thread Component Implementation

The developer's primary concern is the code generated for the model components marked as AADL Threads.  These represent real-time tasks whose execution is initiated (dispatched) either by time-outs (for periodic threads) or by messages arriving on component ports (sporadic threads).   
The code in these tasks is organized into entry points that are invoked by the underlying scheduling framework.  The initial code generation provides skeletons for these entry points, and the developer implements the application logic of the component by filling in code in these skeletons.  The code implementing the entry points follows a read - compute - write style (see `technical-approach.md` for details).  

The primary entry points in a component are:
  - Initialize entry point - the developer uses this to initialize the local state of the component as well as initial values of output ports.  Output data ports MUST be initialized.  Initialization of output event and event data ports is optional.  No input ports are allowed to be used in the Initialize entry point.  The Initialize entry point is dispatched once; this dispatch occurs during the HAMR run-time initialization phase.
  - Compute entry point - this takes different forms depending on the declared dispatch mode for the Thread components.  For Periodic components, there is a single method called `timetriggered` that is invoked by the scheduler at points determined by the thread's declared period.  For Sporadic components, there is an event handler associated with each event port and event data port of the thread.

HAMR auto-generates APIs for communicating over the ports of the Thread component.  When the developer codes the application logic in the component, `get_` methods are called to read values from input ports, `put_` methods are called to put values on output ports.

### seL4 Microkit Specific Information

For HAMR development on the seL4 Microkit target, components will primarily be implemented in Rust.  

Verus (https://verus-lang.github.io/verus/guide/) is used to verify that Rust code implementing the component application logic (the entry point methods above) conforms to Verus contracts.  Verus contracts for entry point methods are automatically generated by HAMR from model-level GUMBO contracts.

The PropTest framework (https://altsysrq.github.io/proptest-book/) is used to support property-based testing for each Thread component.  HAMR will generate GUMBOX contracts (executable versions of the GUMBO contracts written as Rust boolean functions) to support conventional unit testing as well as automated property based testing.

Developers and Claude should use the testing and verification capabilities above to determine the correctness of Rust component implementations (i.e., to determine if the Rust entry point implementations conform to GUMBO model-level contracts).


## Component (Unit) Testing 

HAMR generates infrastructure code for libraries that help the developer with unit testing of each component.  For testing Initialize entry points, these libraries include support running the entry point method and for getting values from the component output ports.   For testing Compute entry points, the libraries include support for putting values on input ports, running the entry point method and for getting values from the component output ports. 

## Task Scheduling 

HAMR adopts a simple static cyclic scheduling approach.   HAMR generates an initial candidate schedule and the developer modifies that schedule to obtain a particular linear ordering of components within the schedule's major frame.

## Component Integration and System Builds 

The developer DOES NOT play a significant role in performing component integration or constructing the system build.  Instead, HAMR auto-generates build files to carry out the build.  Because of the very regular nature of component interfaces (defined in terms of ports), integration of components is highly automated based on component port connections specified in the model.

## Deployment Platform

HAMR code generation options enable systems to be deployed to the JVM, Unix-like OSes (Linux, MacOS) using Unix processes, and the seL4 Microkernel as supported by the Microkit framework.  

For seL4 Microkit, HAMR generates 
  - kernel configuration information (in the Microkit XML system description file).
  - bridge code (glue code) that adapts HAMR thread application code to the computing and communication primities available on seL4 Microkit
  - other infrastructure necessary to deploy HAMR-assembled seL4-based systems on the Qemu simulator or on development boards

