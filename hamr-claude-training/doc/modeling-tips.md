# SysMLv2 Modeling Tips for HAMR

Practical tips and known issues for creating SysMLv2 models that work with the HAMR toolchain.

## SysMLv2 Keyword Conflicts

Certain identifiers conflict with SysMLv2 reserved keywords. Use alternatives:

| Keyword | Alternative |
|---------|-------------|
| `filter` | `msg_filter` |

If the HAMR type checker reports unexpected parse errors on an identifier, check whether the name conflicts with a SysMLv2 keyword.

## Architecture Constraints

### Flat System Architecture Required

HAMR does not currently support nested `System` definitions (a `System` containing another `System`). Nested systems cause internal errors in the HAMR type checker.

**Use a flat architecture** with a single top-level `System` containing all `Process` instances directly:

```sysml
// CORRECT: flat architecture
part def MySystem :> System {
    part proc_a : ProcessA;
    part proc_b : ProcessB;
    // all processes at the same level
}

// INCORRECT: nested systems (causes type checker errors)
part def MySystem :> System {
    part subsystem : MySubsystem;  // System containing System -- will fail
}
```

Maintain logical separation through naming conventions, comments, and package organization instead.

### One Thread Per Process (seL4 Microkit)

When targeting seL4 Microkit, HAMR only allows one `Thread` instance per `Process`. Each `Process` corresponds to a seL4 Microkit protection domain.

```sysml
part def MyProcess :> Process {
    port input : EventDataPort { in :>> type : MyData; }
    port output : EventDataPort { out :>> type : MyData; }

    attribute :>> Domain = MyProperties::Domain_MyComponent;

    part my_thread : MyThread;
    connection ci : PortConnection connect input to my_thread.input;
    connection co : PortConnection connect my_thread.output to output;
}
```

## GUMBO Contract Blocks

### GUMBO Blocks Are Parsed, Not Comments

The `language "GUMBO" /*{...}*/` syntax looks like a comment, but the HAMR type checker **parses** the content inside. The GUMBO syntax must be valid:

```sysml
// This IS parsed by the type checker -- syntax must be correct
language "GUMBO" /*{
    integration
        guarantee No_TopSecret_Output:
            output.security_level != SNG_Data_Model::SecurityLevel.TopSecret;
}*/
```

### GUMBO Limitations with EventDataPorts

GUMBO does not currently support `None`/`Some`/`Option` syntax for EventDataPort compute contracts. For specifying behavior on EventDataPort values, use **integration constraints** on port fields instead of compute contracts:

```sysml
// WORKS: integration constraint on port fields
language "GUMBO" /*{
    integration
        guarantee Constraint_Name:
            output.field_name != SomeValue;
}*/

// NOT SUPPORTED: Option/Some/None in compute contracts
```

### GUMBO Field Access Syntax

In GUMBO expressions, access enum values using dot notation with the full package-qualified path:

```
PackageName::EnumType.EnumValue
```

Example: `SNG_Data_Model::SecurityLevel.TopSecret`

## Data Type Definitions

### Field Keywords Matter

When defining struct-style data types (`part def ... :> Data`), the keyword used for fields depends on the field's type:

- **`attribute`** for enum-typed fields
- **`part`** for Data-typed fields (including base types like `Integer_32`)

```sysml
part def Message :> Data {
    attribute security_level : SecurityLevel;      // enum field -> attribute
    part payload : Base_Types::Integer_32;         // Data field -> part
}

enum def SecurityLevel {
    enum Public;
    enum Secret;
    enum TopSecret;
}
```

## Model File Organization

Organize model files by concern. A typical project structure:

| File | Purpose |
|------|---------|
| `<Project>_Data_Model.sysml` | Data types (enums, structs) |
| `<Project>_Software.sysml` | Thread definitions with ports and GUMBO contracts |
| `<Project>.sysml` | Main system file (processes, connections, allocations, HAMR config) |
| `Platform.sysml` | Processor definition |
| `<Project>_Properties.sysml` | Scheduling domain constants and shared properties |

The main system file should include the HAMR configuration comment at the top:

```sysml
//@ HAMR: --platform Microkit --output-dir ../../hamr
```

## Common Thread Definition Pattern

Define a reusable base thread definition to avoid repeating common attributes:

```sysml
part def Periodic_Rust_Thread :> Thread {
    attribute :>> Period = 1000 [ms];
    attribute :>> Dispatch_Protocol = Supported_Dispatch_Protocols::Periodic;
    attribute :>> Implementation_Language = Implementation_Languages::Rust;
}

part def MyThread :> Periodic_Rust_Thread {
    port input : EventDataPort { in :>> type : MyData; }
    port output : EventDataPort { out :>> type : MyData; }

    language "GUMBO" /*{
        integration
            guarantee Output_Constraint:
                output.some_field != SomeBadValue;
    }*/
}
```

## Scheduling Domains

Scheduling domain IDs start at **2** (domains 0 and 1 are reserved by Microkit). Order domain assignments to match the intended pipeline flow:

```sysml
package MyProperties {
    private import HAMR::*;

    attribute Domain_Sender : CASE_Scheduling::Domain = 2;
    attribute Domain_Processor : CASE_Scheduling::Domain = 3;
    attribute Domain_Receiver : CASE_Scheduling::Domain = 4;
}
```

## Process and Connection Patterns

### Process Definition

Each process wraps a single thread, exposing the thread's ports at the process level and connecting them internally:

```sysml
part def MyProcess :> Process {
    port input : EventDataPort { in :>> type : MyData; }
    port output : EventDataPort { out :>> type : MyData; }

    attribute :>> Domain = MyProperties::Domain_MyComponent;

    part my_thread : MyThread;
    connection ci : PortConnection connect input to my_thread.input;
    connection co : PortConnection connect my_thread.output to output;
}
```

### System Definition

The system connects processes in a pipeline, allocates them to a processor:

```sysml
part def MySystem :> System {
    part sender : SenderProcess;
    part processor : ProcessorProcess;
    part receiver : ReceiverProcess;
    part hw : Platform::MyProcessor;

    connection c1 : PortConnection connect sender.output to processor.input;
    connection c2 : PortConnection connect processor.output to receiver.input;

    allocation pb0 : Deployment_Properties::Actual_Processor_Binding
        allocate sender to hw;
    allocation pb1 : Deployment_Properties::Actual_Processor_Binding
        allocate processor to hw;
    allocation pb2 : Deployment_Properties::Actual_Processor_Binding
        allocate receiver to hw;
}
```

## Processor Definition

```sysml
package Platform {
    private import HAMR::*;

    part def MyProcessor :> Processor {
        attribute :>> Frame_Period = 1 [s];
        attribute :>> Clock_Period = 1 [s];
    }
}
```

## MCP Workflow Tip

Always **type-check before code generation**. If the model has errors, code generation may produce incorrect or incomplete output.

```
1. Edit .sysml files
2. Run sireum_hamr_sysml_tipe (type check)
3. Fix any errors
4. Run sireum_hamr_sysml_codegen (code generation)
```

The `sourcepath` for both tools must include the `aadl-lib/` directory and the project model directory.
