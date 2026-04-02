# HAMR SysMLv2 and GUMBO Quick Reference

## Table of Contents

1. [Package Declarations and Imports](#1-package-declarations-and-imports)
2. [HAMR Code Generation Annotations](#2-hamr-code-generation-annotations)
3. [Component Definitions (Part Defs)](#3-component-definitions-part-defs)
4. [Port Declarations](#4-port-declarations)
5. [Thread Properties](#5-thread-properties)
6. [Subcomponents (Part Instantiation)](#6-subcomponents-part-instantiation)
7. [Connections](#7-connections)
8. [Processor and Deployment](#8-processor-and-deployment)
9. [Data Type Definitions](#9-data-type-definitions)
10. [GUMBO Specification Blocks](#10-gumbo-specification-blocks)
11. [GUMBO Library Functions](#11-gumbo-library-functions)
12. [GUMBO State Declarations](#12-gumbo-state-declarations)
13. [GUMBO Subclause Functions](#13-gumbo-subclause-functions)
14. [GUMBO Integration Constraints](#14-gumbo-integration-constraints)
15. [GUMBO Initialize Contracts](#15-gumbo-initialize-contracts)
16. [GUMBO Compute Contracts](#16-gumbo-compute-contracts)
17. [GUMBO Handler Contracts (Sporadic Threads)](#17-gumbo-handler-contracts-sporadic-threads)
18. [GUMBO Operators and Expressions](#18-gumbo-operators-and-expressions)
19. [GUMBO Quantified Expressions (Arrays)](#19-gumbo-quantified-expressions-arrays)
20. [AADL Domain Library Hierarchy](#20-aadl-domain-library-hierarchy)
21. [Base Types Reference](#21-base-types-reference)

---

## 1. Package Declarations and Imports

### Basic Package

```sysml
package MyPackage {
  // declarations
}
```

### Library Package (for reusable AADL/HAMR definitions)

```sysml
library package HAMR {
  // library-scoped definitions
}
```

### Imports

```sysml
// Import everything from a package (wildcard)
private import HAMR::*;
private import Isolette_Data_Model::*;

// Public import (re-exports to consumers of this package)
public import HAMR::*;

// Multiple imports
private import Regulate::*;
private import Monitor::*;
private import HAMR::*;
```

### Naming Convention

- Package names match file names (e.g., `Regulate.sysml` contains `package Regulate`)
- Qualified references use `::` (e.g., `Isolette_Data_Model::TempWstatus_i`)
- Enum values use `.` (e.g., `Isolette_Data_Model::Status.Init_Status`)

---

## 2. HAMR Code Generation Annotations

HAMR annotations are special comments at the top of the file, prefixed with `//@ HAMR:`.

```sysml
//@ HAMR: --platform JVM --slang-output-dir hamr/slang --package-name isolette
//@ HAMR: --platform JVM --runtime-monitoring --slang-output-dir ../hamr/slang --package-name isolette
//@ HAMR: --platform Microkit --sel4-output-dir ../hamr/microkit
//@ HAMR: --platform Linux --package-name rts --run-transpiler --max-string-size 250 --output-dir hamr
//@ HAMR: --platform seL4 --package-name rts --run-transpiler --max-string-size 250 --output-dir hamr
```

### Common Options

| Option | Description |
|--------|-------------|
| `--platform JVM` | Target JVM/Slang platform |
| `--platform Microkit` | Target seL4 Microkit |
| `--platform Linux` | Target Linux |
| `--platform seL4` | Target seL4 (CAmkES) |
| `--slang-output-dir <path>` | Output for Slang code |
| `--sel4-output-dir <path>` | Output for seL4 code |
| `--package-name <name>` | Top-level package name |
| `--runtime-monitoring` | Enable runtime monitoring |
| `--run-transpiler` | Run C transpiler |
| `--max-string-size <n>` | Max string size for transpiled code |
| `--sourcepath <paths>` | Colon-separated library paths |
| `--output-dir <path>` | General output directory |

Multiple HAMR annotations can appear, each targeting a different platform.

---

## 3. Component Definitions (Part Defs)

All AADL component categories are modeled as `part def` specializing from the HAMR/AADL domain library.

### System

```sysml
part def Isolette_Single_Sensor :> System {
  // subcomponents, ports, connections
}
```

### Process

```sysml
part def Manage_Regulator_Interface_Process :> Process {
  part mri: Manage_Regulator_Interface_i;   // thread subcomponent

  // ports, connections
  attribute :>> Domain = 7;                  // scheduling domain
}
```

### Thread

```sysml
part def Manage_Regulator_Interface_i :> Thread {
  attribute :>> Dispatch_Protocol = Supported_Dispatch_Protocols::Periodic;
  attribute :>> Period = 1000 [ms];
  attribute :>> Implementation_Language = Implementation_Languages::Rust;

  // ports
  port current_tempWstatus : DataPort { in :>> type : Isolette_Data_Model::TempWstatus_i; }
  port regulator_status : DataPort { out :>> type : Isolette_Data_Model::Status; }

  // GUMBO contracts go here
  language "GUMBO" /*{ ... }*/
}
```

### Processor

```sysml
part def Isolette_Processor :> Processor {
  attribute :>> Frame_Period = 1000 [ms];
  attribute :>> Clock_Period = 1 [ms];
}
```

### Abstract (environment components)

```sysml
part def Nurse :> Abstract {
  port operator_visual_information : DataPort { in :>> type : Interface_Interaction; }
  port operator_commands : DataPort { out :>> type : Interface_Interaction; }
}
```

### Helper Thread Definitions (shorthand patterns)

```sysml
// Reusable base thread defs to reduce boilerplate
part def Periodic_Thread :> Thread {
  attribute :>> Period = 1[s];
  attribute :>> Dispatch_Protocol = Periodic;
}

part def Sporadic_Thread :> Thread {
  attribute :>> Dispatch_Protocol = Sporadic;
}

part def Periodic_Rust_Thread :> Thread {
  attribute :>> Period = 1[s];
  attribute :>> Dispatch_Protocol = Supported_Dispatch_Protocols::Periodic;
  attribute :>> Implementation_Language = Implementation_Languages::Rust;
}

// Usage: inherit from helper
part def producer_t_p :> Periodic_Thread {
  port write_port: DataPort { out :>> type : struct_i; }
}
```

---

## 4. Port Declarations

### DataPort (continuous data, sampled)

```sysml
// Input data port
port current_temp : DataPort { in :>> type : Temperature; }

// Output data port
port heat_control : DataPort { out :>> type : Isolette_Data_Model::On_Off; }

// Bidirectional data port
port infant_interaction : DataPort { inout :>> type : Air_Interaction; }

// Using Base_Types directly
port channel1 : DataPort { in :>> type : Base_Types::Boolean; }
```

### EventPort (event-only, no data payload)

```sysml
out port tempChanged : EventPort;
in port read_port : EventPort;
```

### EventDataPort (event with data payload)

```sysml
// Input event data port
port fanAck : EventDataPort { in :>> type : FanAck; }
port setPoint : EventDataPort { in :>> type : SetPoint; }

// Output event data port
port fanCmd : EventDataPort { out :>> type : FanCmd; }
port setPoint : EventDataPort { out :>> type : SetPoint; }
```

### EventDataPort with Queue Size

```sysml
port read_port : EventDataPort {
  in :>> type : struct_i;
  attribute :>> Queue_Size = 2;
}

port read_port : EventDataPort {
  in :>> type : struct_i;
  attribute :>> Queue_Size = 5;
}
```

---

## 5. Thread Properties

### Dispatch Protocol

```sysml
// Periodic dispatch (time-triggered)
attribute :>> Dispatch_Protocol = Supported_Dispatch_Protocols::Periodic;

// Sporadic dispatch (event-triggered)
attribute :>> Dispatch_Protocol = Supported_Dispatch_Protocols::Sporadic;

// Shorthand (with appropriate imports)
attribute :>> Dispatch_Protocol = Periodic;
attribute :>> Dispatch_Protocol = Sporadic;
```

### Period

```sysml
attribute :>> Period = 1000 [ms];   // 1 second
attribute :>> Period = 1 [s];       // 1 second (alternative unit)
```

### Implementation Language

```sysml
attribute :>> Implementation_Language = Implementation_Languages::Rust;
attribute :>> Implementation_Language = Implementation_Languages::Slang;
attribute :>> Implementation_Language = Implementation_Languages::C;
```

---

## 6. Subcomponents (Part Instantiation)

```sysml
part def Isolette_Single_Sensor :> System {
  // Typed subcomponents using qualified names
  part thermostat : Thermostat::Thermostat_Single_Sensor_System_i;
  part operator_interface : Operator_Interface::Operator_Interface_System_i;
  part temperature_sensor : Devices::Temperature_Sensor_System_i;
  part heat_source : Devices::Heat_Source_System_i;
  part isolette_processor : Isolette_Processor;   // local type

  // Within same package
  part rt : Regulate::Regulate_Temperature_i;
  part mt : Monitor::Monitor_Temperature_i;
}
```

---

## 7. Connections

### Basic PortConnection

```sysml
connection <name> : PortConnection
  connect <source> to <target>;
```

### Multi-line Format

```sysml
connection oioc : PortConnection
  connect operator_commands to operator_interface.operator_commands;

connection ct : PortConnection
  connect temperature_sensor.current_tempWstatus to thermostat.current_tempWstatus;
```

### Single-line Format

```sysml
connection udtw: PortConnection connect upper_desired_tempWstatus to mri.upper_desired_tempWstatus;
```

### Connection Patterns

```sysml
// External port to internal subcomponent port
connection a2ts : PortConnection
  connect air_temperature to temperature_sensor.air;

// Internal subcomponent port to external port
connection rdt : PortConnection
  connect mri.displayed_temp to displayed_temp;

// Between internal subcomponents
connection mudt : PortConnection
  connect mri.upper_desired_temp to mhs.upper_desired_temp;

// Fan-out (one source to multiple targets) -- use separate connections
connection rcth : PortConnection connect current_tempWstatus to mhs.current_tempWstatus;
connection rctm : PortConnection connect current_tempWstatus to mrm.current_tempWstatus;
```

---

## 8. Processor and Deployment

### Processor Definition

```sysml
part def Isolette_Processor :> Processor {
  attribute :>> Frame_Period = 1000 [ms];
  attribute :>> Clock_Period = 1 [ms];
}
```

### Processor Binding (Allocation)

```sysml
allocation pb0: Deployment_Properties::Actual_Processor_Binding
  allocate thermostat to isolette_processor;

allocation pb1: Deployment_Properties::Actual_Processor_Binding
  allocate operator_interface to isolette_processor;
```

### Process Domain Assignment

```sysml
part def Manage_Regulator_Interface_Process :> Process {
  // ...
  attribute :>> Domain = 7;
}
```

### Scheduling Mode

```sysml
// MCS (Mixed-Criticality Scheduling)
attribute :>> Scheduling = MCS;

// Domain Scheduling
attribute :>> Scheduling = Domain_Scheduling;
```

---

## 9. Data Type Definitions

### Struct (Record Type)

```sysml
part def TempWstatus_i :> Data {
  part degrees : Base_Types::Integer_32;
  attribute status : ValueStatus;              // enum-typed field
}

part def Temperature :> Data {
  part degrees : Base_Types::Float_32;
  attribute unit : TempUnit;
}

part def SetPoint :> Data {
  part low : Temperature;                      // nested struct field
  part high : Temperature;
}

// Using the Struct specialization (explicit struct marker)
part def struct_i :> Struct {
  part size : Base_Types::Integer_32;
  part elements : ArrayOfInts;
}

part def MyStruct_i :> Struct {
  part fieldInt64 : Base_Types::Integer_64;
  part fieldStr   : Base_Types::String;
  attribute fieldEnum : MyEnum;                // enum field uses `attribute`
  part fieldRec   : MyStruct2_i;               // nested struct
  part fieldArray : MyArrayOneDim;             // array field
}
```

### Enum

```sysml
enum def On_Off {
  enum Onn;
  enum Off;
}

enum def Status {
  enum Init_Status;
  enum On_Status;
  enum Failed_Status;
}

enum def FanCmd {
  enum On;
  enum Off;
}

// Compact enum (alternative style)
enum def MyEnum { On; Off; }
```

### Referencing Enum Values

```sysml
Isolette_Data_Model::On_Off.Off
Isolette_Data_Model::Status.Init_Status
Isolette_Data_Model::Regulator_Mode.Normal_Regulator_Mode
FanCmd.On                                     // within same package
```

### Array

```sysml
part def MyArrayInt32 :> Array {
  part :>> Base_Type : Base_Types::Integer_32; // element type
  attribute :>> Dimensions = 10;               // array size
  attribute :>> Array_Size_Kind = Array_Size_Kinds::Fixed;
  attribute :>> Data_Size = 40[byte];          // total byte size
}

// Array of structs
part def MyArrayStruct :> Array {
  part :>> Base_Type : MyStruct2_i;
  attribute :>> Dimensions = 10;
  attribute :>> Array_Size_Kind = Array_Size_Kinds::Fixed;
  attribute :>> Data_Size = 40[byte];
}

// Shorthand (with appropriate imports: Fixed is an alias)
part def MyArrayOneDim :> Array {
  part :>> Base_Type : Base_Types::Integer_32;
  attribute :>> Dimensions = 10;
  attribute :>> Array_Size_Kind = Fixed;
  attribute :>> Data_Size = 40 [byte];
}
```

### Empty Data Definition

```sysml
part def RawEthernetMessage :> Data {
  // opaque / uninterpreted data
}
```

---

## 10. GUMBO Specification Blocks

GUMBO contracts are embedded in `Thread` or `Data` part defs using:

```sysml
language "GUMBO" /*{
  // GUMBO specification here
}*/
```

The content is inside a block comment `/* ... */` so standard SysMLv2 parsers ignore it, but HAMR parses and processes it.

### Overall Structure (Thread)

```sysml
part def MyThread :> Thread {
  // ... ports, properties ...

  language "GUMBO" /*{
    state
      // state variable declarations

    functions
      // helper function definitions

    integration
      // port-level assume/guarantee constraints

    initialize
      // modifies clause (optional)
      // guarantee clauses for initialization

    compute
      // modifies clause (optional)
      // assume clauses
      // guarantee clauses
      // compute_cases
      // handle clauses (sporadic threads)
  }*/
}
```

### Data Invariants

```sysml
part def Temperature :> Data {
  part degrees : Base_Types::Float_32;

  language "GUMBO" /*{
    invariants
      inv AbsZero:
        degrees >= GUMBO_Periodic_Definitions::GUMBO__Library::absoluteZero();
  }*/
}

part def TempWstatus_i :> Data {
  part degrees : Base_Types::Integer_32;
  attribute status : ValueStatus;

  language "GUMBO" /*{
    invariants
      inv MaxMinEarthTemperatures "Temps should fall within the max/min temperatures
                                  |recorded on planet Earth" :
        -128 [s32] <= degrees and degrees <= 134 [s32];
  }*/
}

part def SetPoint :> Data {
  part low : Temperature;
  part high : Temperature;

  language "GUMBO" /*{
    invariants
      inv SetPoint_Data_Invariant:
        (low.degrees >= 50.0 [f32]) &
        (high.degrees <= 110.0 [f32]) &
        (low.degrees <= high.degrees);
  }*/
}
```

---

## 11. GUMBO Library Functions

GUMBO library functions are defined in a dedicated package and can be called from any GUMBO block.

### Defining Library Functions

```sysml
package GUMBO_Periodic_Definitions {
  language "GUMBO" /*{
    library
      functions
        def absoluteZero(): Base_Types::Float_32 := -459.67 [f32];
  }*/
}
```

```sysml
package GumboLib {
  language "GUMBO" /*{
    library
      functions
        def normalLibraryFunction(a: MyArrayInt32): Base_Types::Boolean :=
          (0 .. size(a) - 2) -> forAll {i; a#(i) <= a#(i + 1)};

        // Spec function (abstract, no body -- implemented externally)
        @spec def librarySpecFunction_Assume(a: MyArrayInt32): Base_Types::Boolean;
        @spec def librarySpecFunction_Guarantee(a: MyArrayInt32): Base_Types::Boolean;
  }*/
}
```

### Complex Library Example (Isolette)

```sysml
package GUMBO_Library {
  language "GUMBO" /*{
    library
      functions
        def LowerAlarmTemp_lower(): Base_Types::Integer_32 := 96[s32];
        def LowerAlarmTemp_upper(): Base_Types::Integer_32 := 101[s32];

        def Allowed_LowerAlarmTemp(lower: Base_Types::Integer_32): Base_Types::Boolean :=
          LowerAlarmTemp_lower() <= lower & lower <= LowerAlarmTemp_upper();

        def Allowed_LowerAlarmTempWStatus(
              lower: Isolette_Data_Model::TempWstatus_i): Base_Types::Boolean :=
          (isValidTempWstatus(lower) implies
                Allowed_LowerAlarmTemp(lower.degrees));

        def isValidTempWstatus(
              value: Isolette_Data_Model::TempWstatus_i): Base_Types::Boolean :=
          value.status == Isolette_Data_Model::ValueStatus.Valid;
  }*/
}
```

### Calling Library Functions

```sysml
// Fully qualified: PackageName::GUMBO__Library::functionName(args)
GUMBO_Library::GUMBO__Library::Allowed_LowerAlarmTemp(lower_alarm_temp.degrees)
GUMBO_Periodic_Definitions::GUMBO__Library::absoluteZero()
TempControlPeriodic::GUMBO__Library::inRange(currentTemp)
```

Note: The `GUMBO__Library` (double underscore) is the auto-generated container name for library functions.

### Inline Library (within a component package)

```sysml
package TempControlPeriodic {
  private import HAMR::*;

  language "GUMBO" /*{
    library
      functions
        def inRange(temp: Temperature): Base_Types::Boolean :=
          temp.degrees >= -40.0 [f32] and temp.degrees <= 122.0 [f32];
  }*/

  // ... component definitions follow ...
}
```

---

## 12. GUMBO State Declarations

State variables represent persistent internal component state that influences behavior across dispatches.

```sysml
language "GUMBO" /*{
  state
    lastRegulatorMode: Isolette_Data_Model::Regulator_Mode;
}*/
```

```sysml
state
  lastCmd: Isolette_Data_Model::On_Off;
```

```sysml
state
  currentSetPoint: SetPoint;
  currentFanState: FanCmd;
  latestTemp: Temperature;
```

```sysml
state
  myArrayInt32_StateVar: MyArrayInt32;
  myArrayStruct_StateVar: MyArrayStruct;
  myStructArray_StateVar: MyStructArray_i;
```

---

## 13. GUMBO Subclause Functions

Functions defined within a thread's GUMBO block (not in a library).

### Regular Functions

```sysml
functions
  def ROUND(num: Base_Types::Integer_32): Base_Types::Integer_32 := num;

  def timeout_condition_satisfied(): Base_Types::Boolean := T;

  def defaultTempDegrees(): Base_Types::Float_32 := 72 [f32];
```

### Functions Operating on Arrays

```sysml
functions
  def myArrayInt32_FunctionReturn(v: MyArrayInt32): MyArrayInt32 := v;

  def myArrayInt32_FunctionParam(v: MyArrayInt32): Base_Types::Boolean :=
    (0 .. size(v) - 1) -> exists {i; v#(i) == 0[i32] };

  def myStructArray_i_FunctionParam(v: MyStructArray_i): Base_Types::Boolean :=
    (0 .. size(v.fieldArray) - 1) -> exists {i;
      v.fieldArray#(i).fieldSInt32 == 0[i32] };
```

### Spec Functions (Abstract -- no body)

```sysml
functions
  // For assume contexts
  @spec def subclauseSpecFunction_Assume(a: MyArrayInt32): Base_Types::Boolean;

  // For guarantee contexts
  def subclauseSpecFunction_Guarantee(a: MyArrayInt32): Base_Types::Boolean;
```

### Calling a Library Function from a Subclause Function

```sysml
functions
  def Allowed_UpperAlarmTempWStatus(
        upper: Isolette_Data_Model::TempWstatus_i): Base_Types::Boolean :=
    GUMBO_Library::GUMBO__Library::Allowed_UpperAlarmTempWStatus(upper);
```

---

## 14. GUMBO Integration Constraints

Integration constraints specify assumptions and guarantees on ports that hold between dispatches -- they constrain the values flowing through connections.

### Assume (on input ports -- receiver side)

```sysml
integration
  assume currentTempRange:
    (currentTemp.degrees >= -40.0 [f32]) & (currentTemp.degrees <= 122.0 [f32]);

  assume Allowed_LowerAlarmTemp:
    GUMBO_Library::GUMBO__Library::Allowed_LowerAlarmTempWStatus(
      lower_alarm_tempWstatus);

  assume Allowed_UpperAlarmTemp:
    GUMBO_Library::GUMBO__Library::Allowed_UpperAlarmTempWStatus(
      upper_alarm_tempWstatus);
```

### Guarantee (on output ports -- sender side)

```sysml
integration
  guarantee Sensor_Temperature_Range:
    TempControlPeriodic::GUMBO__Library::inRange(currentTemp);

  guarantee Allowed_LowerAlarmTempWstatus
    "Table_A_12_LowerAlarmTemp: Range [96..101]" :
    (GUMBO_Library::GUMBO__Library::isValidTempWstatus(lower_alarm_tempWstatus) implies
          (96[i32] <= lower_alarm_tempWstatus.degrees
          & lower_alarm_tempWstatus.degrees <= 101[i32]));
```

### Integration with Array Quantifiers

```sysml
integration
  guarantee integrationArrayInt32_DataPort:
    p_myArrayInt32_DataPort#(0) == 1[s32] &
    GumboLib::GUMBO__Library::librarySpecFunction_Guarantee(
      p_myArrayInt32_DataPort) &
    (0 .. size(p_myArrayInt32_DataPort) - 2) -> forAll {i;
      p_myArrayInt32_DataPort#(i) <= p_myArrayInt32_DataPort#(i + 1) };

  assume integrationStructArray_EventDataPort
    "Example of optional descriptor" :
    (0 .. size(c_myStructArray_EventDataPort.fieldArray) - 2) -> forAll {i;
      c_myStructArray_EventDataPort.fieldArray#(i).fieldSInt32
        <= c_myStructArray_EventDataPort.fieldArray#(i + 1).fieldSInt32 };
```

---

## 15. GUMBO Initialize Contracts

Initialize contracts specify what must hold after the component's initialize entrypoint executes.

### Basic Guarantee

```sysml
initialize
  guarantee
    RegulatorStatusIsInitiallyInit:
      regulator_status == Isolette_Data_Model::Status.Init_Status;
```

### Multiple Guarantees

```sysml
initialize
  guarantee
    initlastCmd: lastCmd == Isolette_Data_Model::On_Off.Off;
  guarantee REQ_MHS_1 "If the Regulator Mode is INIT, the Heat Control
                       |shall be set to Off." :
    heat_control == Isolette_Data_Model::On_Off.Off;
```

### With Modifies Clause

```sysml
initialize
  modifies (latestFanCmd);
  guarantee initLatestFanCmd "Initialize state variable":
    latestFanCmd == FanCmd.Off;
  guarantee initFanCmd "Initial fan command":
    fanCmd == FanCmd.Off;
```

```sysml
initialize
  modifies currentSetPoint, currentFanState, latestTemp;

  guarantee defaultSetPoint:
    (currentSetPoint.low.degrees == 70 [f32])
    and (currentSetPoint.high.degrees == 80 [f32]);
  guarantee defaultFanStates:
    currentFanState == FanCmd.Off;
  guarantee defaultLatestTemp:
    latestTemp.degrees == 72.0[f32];
```

### Compound Guarantee

```sysml
initialize
  guarantee REQ_MA_1 "..." :
    alarm_control == Isolette_Data_Model::On_Off.Off &
    lastCmd == Isolette_Data_Model::On_Off.Off;
```

### Output Port Initialization

```sysml
initialize
  guarantee initOutputDataPort
    "The Initialize Entry Point must initialize all component
    |local state and all output data ports." :
    output == false;
```

---

## 16. GUMBO Compute Contracts

Compute contracts specify the behavior of the compute entrypoint.

### Assume Clauses

```sysml
compute
  assume lower_is_not_higher_than_upper:
    lower_desired_tempWstatus.degrees <= upper_desired_tempWstatus.degrees;

  assume Figure_A_7 "This is not explicitly stated in the requirements..." :
    upper_alarm_temp.degrees - lower_alarm_temp.degrees >= 1 [s32];
```

### Guarantee Clauses (General)

```sysml
compute
  guarantee lastCmd "Set lastCmd to value of output Cmd port":
    lastCmd == heat_control;

  guarantee orOutput:
    actuate == (channel1 | channel2);

  guarantee coincidenceOutput "description..." :
    actuate == ((channel1 & channel2) |
                (channel1 & channel3) |
                (channel1 & channel4) |
                (channel2 & channel3) |
                (channel2 & channel4) |
                (channel3 & channel4));
```

### Modifies Clause in Compute

```sysml
compute
  modifies (latestFanCmd);
  // or
  modifies currentSetPoint, currentFanState, latestTemp;
```

### Compute Cases

Compute cases provide case-based specifications with per-case assumes and guarantees.

```sysml
compute
  compute_cases

    case REQ_MRI_1 "If the Regulator Mode is INIT,
                    |the Regulator Status shall be set to Init." :
      assume regulator_mode == Isolette_Data_Model::Regulator_Mode.Init_Regulator_Mode;
      guarantee regulator_status == Isolette_Data_Model::Status.Init_Status;

    case REQ_MRI_2 "If the Regulator Mode is NORMAL,
                    |the Regulator Status shall be set to On" :
      assume regulator_mode == Isolette_Data_Model::Regulator_Mode.Normal_Regulator_Mode;
      guarantee regulator_status == Isolette_Data_Model::Status.On_Status;

    case REQ_MRI_3 "If the Regulator Mode is FAILED,
                    |the Regulator Status shall be set to Failed." :
      assume regulator_mode == Isolette_Data_Model::Regulator_Mode.Failed_Regulator_Mode;
      guarantee regulator_status == Isolette_Data_Model::Status.Failed_Status;
```

### Mixed General + Case-Based Guarantees

```sysml
compute
  assume lower_is_lower_temp:
    lower_desired_temp.degrees <= upper_desired_temp.degrees;

  // General guarantee (always holds)
  guarantee lastCmd "Set lastCmd to value of output Cmd port":
    lastCmd == heat_control;

  // Case-specific guarantees
  compute_cases
    case REQ_MHS_1 "..." :
      assume regulator_mode ==
        Isolette_Data_Model::Regulator_Mode.Init_Regulator_Mode;
      guarantee heat_control == Isolette_Data_Model::On_Off.Off;

    case REQ_MHS_4 "If ... in range, the value shall not be changed." :
      assume (regulator_mode ==
          Isolette_Data_Model::Regulator_Mode.Normal_Regulator_Mode)
        & (current_tempWstatus.degrees >= lower_desired_temp.degrees
        & current_tempWstatus.degrees <= upper_desired_temp.degrees);
      guarantee heat_control == In(lastCmd);
```

### Guarantee with Implication

```sysml
guarantee
  (not interface_failure.flag) implies
    (lower_alarm_temp.degrees == lower_alarm_tempWstatus.degrees
    &
    upper_alarm_temp.degrees == upper_alarm_tempWstatus.degrees);
```

### Using the Implication Operator

```sysml
guarantee altCurrentTempLTSetPoint "If current temperature is less than
                                   |the current low set point, then the
                                   |fan state shall be Off" :
  (currentTemp.degrees < setPoint.low.degrees implies
        (latestFanCmd == FanCmd.Off and fanCmd == FanCmd.Off));

guarantee REQ_MRI_8 "..." :
  (not interface_failure.flag implies
        ((lower_desired_temp.degrees == lower_desired_tempWstatus.degrees)
        & (upper_desired_temp.degrees == upper_desired_tempWstatus.degrees)));
```

### Unspecified Behavior

```sysml
case REQ_MRI_5 "If the Regulator Mode is not NORMAL,
                |the value of the Display Temperature is UNSPECIFIED." :
  guarantee true;

case REQ_MRI_9 "..." :
  guarantee true;
```

---

## 17. GUMBO Handler Contracts (Sporadic Threads)

For sporadic (event-driven) threads, handler contracts specify behavior per event handler.

```sysml
compute
  modifies currentSetPoint, currentFanState, latestTemp;

  // General guarantees (hold for all handlers)
  guarantee TC_Req_01 "..." :
    (latestTemp.degrees < currentSetPoint.low.degrees implies
          currentFanState == FanCmd.Off);

  guarantee mustSendFanCmd "..." :
    (In(currentFanState) != currentFanState)
      implies MustSend(fanCmd, currentFanState) and
    (currentFanState == In(currentFanState))
      implies NoSend(fanCmd);

  // Per-handler contracts
  handle setPoint:
    modifies (currentSetPoint);
    guarantee setPointChanged:
      currentSetPoint == setPoint;
    guarantee latestTempNotModified:
      (latestTemp == In(latestTemp));

  handle tempChanged:
    modifies (latestTemp);
    guarantee tempChanged:
      latestTemp == currentTemp;
    guarantee setPointNotModified:
      currentSetPoint == In(currentSetPoint);

  handle fanAck:
    guarantee setPointNotModified:
      currentSetPoint == In(currentSetPoint);
    guarantee lastTempNotModified:
      latestTemp == In(latestTemp);
    guarantee currentFanState:
      currentFanState == In(currentFanState);
    guarantee noEventsSent:
      NoSend(fanCmd);
```

---

## 18. GUMBO Operators and Expressions

### The In() Operator (Pre-State Values)

`In(stateVar)` refers to the value of a state variable at the beginning of the current dispatch (pre-state). It can only be used on state variables, not on ports.

```sysml
// State variable unchanged
guarantee heat_control == In(lastCmd);

// Comparing pre and post state
guarantee
  latestFanCmd == In(latestFanCmd) & fanCmd == latestFanCmd;

// In() with array indexing
guarantee noChange:
  In(myArrayInt32_StateVar)#(0) == myArrayInt32_StateVar#(0);

// In() with struct field access on arrays
guarantee:
  In(myArrayStruct_StateVar)#(0).fieldSInt32
    == myArrayStruct_StateVar#(0).fieldSInt32;

// In() passed to functions
guarantee:
  myArrayInt32_FunctionParam(In(myArrayInt32_StateVar));
```

### GUMBO on Event Data Ports

Event data ports can be empty (no message) or contain a message on any given cycle. GUMBO
provides three operators for reasoning about event data port state:

| Operator | Meaning | Slang Translation |
|----------|---------|-------------------|
| `HasEvent(port)` | Port contains a message | `port.nonEmpty` |
| `MustSend(port)` | A message must be sent on the port | `port.nonEmpty` |
| `MustSend(port, value)` | A message with the given value must be sent | `port == Some(value)` |
| `NoSend(port)` | No message must be sent on the port | `port.isEmpty` |

#### Integration Constraints on Event Data Ports

Integration constraints on event data ports apply only when a message is present:

```sysml
// Producer: guarantee output payload is in range when a message is sent
part def Prod :> Thread {
    port output : EventDataPort { out :>> type : Message; }

    language "GUMBO" /*{
        integration
            guarantee Payload_Range:
                (0 [i32] <= output.payload) & (output.payload <= 90 [i32]);
    }*/
}

// Consumer: assume input payload is in range when a message is received
part def Cons :> Thread {
    port input : EventDataPort { in :>> type : Message; }

    language "GUMBO" /*{
        integration
            assume Payload_Range:
                (0 [i32] <= input.payload) & (input.payload <= 100 [i32]);
    }*/
}
```

The Producer's integration **guarantee** on output becomes the Consumer's integration
**assume** on input — this is compositional reasoning across the pipeline.

#### State and Initialize with Event Data Ports

```sysml
part def Cons :> Thread {
    port input : EventDataPort { in :>> type : Message; }

    language "GUMBO" /*{
        state
            payload_sum: Base_Types::Integer_32;

        functions
            def Init_Payload_Sum(): Base_Types::Integer_32 := 0 [i32];

        integration
            assume Payload_Range:
                (0 [i32] <= input.payload) & (input.payload <= 100 [i32]);

        initialize
            guarantee initSum: payload_sum == Init_Payload_Sum();
    }*/
}
```

#### Compute Guarantees with HasEvent / NoSend

Compute contracts use `HasEvent` to check if a message is present on an event data port,
then `NoSend` or field-level guarantees to specify output behavior.

**Important:** Use `and` (conditional/short-circuit) rather than `&` (logical) when combining
`HasEvent` with field access. With `&`, both sides are evaluated — so `HasEvent(input) & input.payload > 0`
would attempt to access `input.payload` even when there is no message, causing a verification error.
With `and`, the field access is only evaluated when `HasEvent(input)` is true.

```sysml
compute
    // No input → no output
    guarantee No_Input_No_Output:
        (not HasEvent(input)) implies NoSend(output);

    // Critical input → drop (no output)
    guarantee Critical_Dropped:
        (HasEvent(input) and input.security_level == SecurityLevel.Critical)
        implies NoSend(output);

    // Non-critical input → forward unchanged
    guarantee Non_Critical_Forwarded:
        (HasEvent(input) and input.security_level != SecurityLevel.Critical)
        implies (HasEvent(output) and output == input);

    // Restricted payload > 100 → clamp to 100
    guarantee Restricted_Clamped_High:
        (HasEvent(input) and input.security_level == SecurityLevel.Restricted
            and input.payload > 100)
        implies (HasEvent(output)
                 and output.security_level == input.security_level
                 and output.payload == 100);
```

### HasEvent with Array Quantifiers

```sysml
HasEvent(c_myArrayInt32_EventDataPort) implies
  (0 .. size(c_myArrayInt32_EventDataPort) - 2) -> forAll { in i;
    c_myArrayInt32_EventDataPort#(i)
      <= c_myArrayInt32_EventDataPort#(i + 1) };
```

### Logical Operators

| Operator | Meaning |
|----------|---------|
| `&` | Logical AND (evaluates both sides) |
| `and` | Conditional AND (short-circuit: skips right side if left is false) |
| `\|` | Logical OR (evaluates both sides) |
| `or` | Conditional OR (short-circuit: skips right side if left is true) |
| `not` | Logical NOT |
| `implies` | Conditional implication (short-circuit: skips consequent if antecedent is false) |

**Note:** `&`/`|` are defined in KerML's `BooleanFunctions` as non-conditional (evaluate both sides).
`and`/`or`/`implies` are defined in KerML's `ControlFunctions` as conditional (second operand is an `expr` — short-circuit).


### Comparison and Arithmetic

| Operator | Meaning |
|----------|---------|
| `==` | Equality |
| `!=` | Inequality |
| `<`, `<=`, `>`, `>=` | Relational |
| `+`, `-`, `*`, `/` | Arithmetic |

### Type-Annotated Literals

```sysml
-128 [s32]        // signed 32-bit integer literal
134 [s32]         // signed 32-bit integer literal
96 [i32]          // integer 32 literal (alternative)
72 [f32]          // float 32 literal
-459.67 [f32]     // negative float 32 literal
50.0 [f32]        // float 32 literal
1 [s32]           // signed 32 literal
0 [i32]           // integer 32 literal
```

### Boolean Literals

```sysml
true
false
T     // shorthand for true
F     // shorthand for false
```

### Field Access

```sysml
// Simple field access
current_tempWstatus.degrees
current_tempWstatus.status
interface_failure.flag

// Nested field access
setPoint.low.degrees
setPoint.high.degrees

// Array element field access
v.fieldArray#(i).fieldSInt32
```

---

## 19. GUMBO Quantified Expressions (Arrays)

### Array Element Access

```sysml
// Index-based access with #()
p_myArrayInt32_DataPort#(0)
myArrayInt32_StateVar#(i)
v.fieldArray#(i).fieldSInt32
```

### Array Size

```sysml
size(p_myArrayInt32_DataPort)
size(v)
size(v.fieldArray)
```

### ForAll (Universal Quantifier)

```sysml
// All adjacent elements are sorted
(0 .. size(p_myArrayInt32_DataPort) - 2) -> forAll {i;
  p_myArrayInt32_DataPort#(i) <= p_myArrayInt32_DataPort#(i + 1) };

// With `in` keyword (alternative form)
(0 .. size(myArrayInt32_StateVar) - 2) -> forAll { in i;
  myArrayInt32_StateVar#(i) <= myArrayInt32_StateVar#(i + 1) };

// Over struct array fields
(0 .. size(c_myArrayStruct_DataPort) - 2) -> forAll { in i;
  c_myArrayStruct_DataPort#(i).fieldSInt32
    <= c_myArrayStruct_DataPort#(i + 1).fieldSInt32 };

// Over nested array in struct
(0 .. size(c_myStructArray_DataPort.fieldArray) - 2) -> forAll { in i;
  c_myStructArray_DataPort.fieldArray#(i).fieldSInt32
    <= c_myStructArray_DataPort.fieldArray#(i + 1).fieldSInt32 };
```

### Exists (Existential Quantifier)

```sysml
(0 .. size(c_myArrayInt32_DataPort) - 1) -> exists { in i;
  c_myArrayInt32_DataPort#(i) == 0[i32] };

(0 .. size(v) - 1) -> exists {i; v#(i) == 0[i32] };
```

### Combining In() with Array Quantifiers

```sysml
// Compare pre-state array with post-state
guarantee isSorted_MyArrayInt32_StateVar_Guarantee:
  (0 .. size(myArrayInt32_StateVar) - 2) -> forAll { in i;
    In(myArrayInt32_StateVar)#(i)
      <= myArrayInt32_StateVar#(i + 1) };

// In() on struct containing array
guarantee:
  (0 .. size(myStructArray_StateVar.fieldArray) - 2) -> forAll { in i;
    In(myStructArray_StateVar).fieldArray#(i).fieldSInt32
      <= myStructArray_StateVar.fieldArray#(i + 1).fieldSInt32 };

// In() passed to function returning array
guarantee:
  (0 .. size(myArrayInt32_FunctionReturn(myArrayInt32_StateVar)) - 2)
    -> forAll { in i;
      myArrayInt32_FunctionReturn(In(myArrayInt32_StateVar))#(i) <=
        myArrayInt32_FunctionReturn(myArrayInt32_StateVar)#(i + 1) };
```

---

## 20. AADL Domain Library Hierarchy

### Import Chain

```
HAMR
  +-- HAMR_Time_Units
  +-- HAMR_Microkit
  +-- HAMR_AADL
  |     +-- System :> AADL::System
  |     +-- Processor :> AADL::Processor
  |     +-- Process :> AADL::Process
  |     +-- Thread :> AADL::Thread
  |     +-- Struct :> Data
  |     +-- Array :> Data
  +-- AADL
  |     +-- Component (abstract)
  |     +-- System, Process, Thread, Processor, ... :> Component
  |     +-- Data :> Component
  |     +-- DataPort, EventPort, EventDataPort (port defs)
  |     +-- PortConnection (interface def)
  +-- Base_Types
        +-- Boolean, Integer_8/16/32/64, Float_32/64, etc. :> Data
```

### What `private import HAMR::*` Gives You

Through transitive imports, a single `private import HAMR::*` brings in:

- `System`, `Process`, `Thread`, `Processor` (from HAMR_AADL)
- `Data`, `Struct`, `Array` (from HAMR_AADL)
- `DataPort`, `EventPort`, `EventDataPort` (from AADL)
- `PortConnection` (from AADL)
- `Base_Types::Boolean`, `Base_Types::Integer_32`, `Base_Types::Float_32`, etc.
- `Supported_Dispatch_Protocols::Periodic`, `Supported_Dispatch_Protocols::Sporadic`
- `Deployment_Properties::Actual_Processor_Binding`
- `Periodic`, `Sporadic` shorthand attributes
- `Implementation_Languages::Rust`, `Implementation_Languages::Slang`, `Implementation_Languages::C`
- `MCS`, `Domain_Scheduling` scheduling modes
- `Array_Size_Kinds::Fixed`, `Array_Size_Kinds::Bound`

---

## 21. Base Types Reference

All defined in `Base_Types` package, extending `AADL::Data`.

| Type | Size | Description |
|------|------|-------------|
| `Base_Types::Boolean` | -- | Boolean value |
| `Base_Types::Integer` | -- | Generic integer |
| `Base_Types::Integer_8` | 1 byte | Signed 8-bit integer |
| `Base_Types::Integer_16` | 2 bytes | Signed 16-bit integer |
| `Base_Types::Integer_32` | 4 bytes | Signed 32-bit integer |
| `Base_Types::Integer_64` | 8 bytes | Signed 64-bit integer |
| `Base_Types::Unsigned_8` | 1 byte | Unsigned 8-bit integer |
| `Base_Types::Unsigned_16` | 2 bytes | Unsigned 16-bit integer |
| `Base_Types::Unsigned_32` | 4 bytes | Unsigned 32-bit integer |
| `Base_Types::Unsigned_64` | 8 bytes | Unsigned 64-bit integer |
| `Base_Types::Float` | -- | Generic float |
| `Base_Types::Float_32` | 4 bytes | 32-bit IEEE 754 float |
| `Base_Types::Float_64` | 8 bytes | 64-bit IEEE 754 float |
| `Base_Types::Character` | -- | Single character |
| `Base_Types::String` | -- | String value |
| `Base_Types::Natural` | -- | Non-negative integer |

### GUMBO Literal Suffixes

| Suffix | Type |
|--------|------|
| `[s32]` | Signed 32-bit integer |
| `[i32]` | Integer 32 (alias) |
| `[f32]` | Float 32 |
| `[ms]` | Milliseconds (for Period, etc.) |
| `[s]` | Seconds |
| `[byte]` | Bytes (for Data_Size) |