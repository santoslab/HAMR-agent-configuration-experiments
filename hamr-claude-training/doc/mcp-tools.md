# HAMR MCP Tools Reference

HAMR exposes its functionality through MCP (Model Context Protocol) tools via the Sireum MCP server. This document describes the MCP tools most relevant for HAMR development with SysMLv2 models targeting Rust on seL4 Microkit.

## MCP Server Configuration

The Sireum MCP server is configured in `.mcp.json`:
```json
{
  "mcpServers": {
    "sireum": {
      "type": "stdio",
      "command": "${SIREUM_HOME}/bin/sireum-mcp.bat",
      "args": []
    }
  }
}
```

The `SIREUM_HOME` environment variable must point to the Sireum installation directory.

---

## sireum_hamr_sysml_tipe -- SysMLv2 Type Checker

**Purpose:** Validates that SysMLv2 models conform to the HAMR SysML subset (AADL concepts expressed in SysMLv2). Run this after creating or modifying `.sysml` model files to confirm well-formedness.

**When to use:** After every model edit to confirm the model is valid HAMR SysML.

### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `args` | string[] | No | Positional arguments; typically the path to the main `.sysml` file |
| `sourcepath` | string[] | No | Source paths containing `.sysml` files. Must include both the project model folder and the `aadl-lib` folder containing the HAMR/AADL library definitions |
| `exclude` | string | No | Sourcepath exclusion as URI segment |
| `parseable-messages` | boolean | No | Print parseable file messages (default: false) |

### Sourcepath Convention

HAMR SysML projects have a standard sourcepath layout. Given a project with:
```
sysmlv2/
  aadl-lib/           # HAMR and AADL library definitions (shared across projects)
    aadl.library/     # Standard AADL property sets as SysMLv2
    hamr.aadl.library/ # HAMR-specific SysMLv2 definitions
  my-model/           # Project-specific model files
    MySystem.sysml
```

The `sourcepath` should include paths to all directories containing `.sysml` files that the type checker needs to resolve. For the example above, the sourcepath would typically include both `sysmlv2/aadl-lib` (recursively) and `sysmlv2/my-model`.

### Example Usage

Type check the ProdCons example:
```
Tool: sireum_hamr_sysml_tipe
args: ["<project-root>/sysmlv2/prod-cons/ProdCons.sysml"]
sourcepath: [
  "<project-root>/sysmlv2/aadl-lib",
  "<project-root>/sysmlv2/prod-cons"
]
```

---

## sireum_hamr_sysml_codegen -- SysMLv2 Code Generator

**Purpose:** Generates implementation code (Rust, C bridge code, build files, Microkit configuration, test infrastructure) from SysMLv2 models. This is the primary code generation tool for HAMR projects.

**When to use:** After the model passes type checking, to generate or regenerate the implementation scaffolding and infrastructure.

### Key Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `args` | string[] | No | Positional arguments; typically the path to the main `.sysml` file |
| `sourcepath` | string[] | No | Source paths of `.sysml` files (same convention as type checker) |
| `platform` | enum | No | Target platform. For seL4 work, use `"Microkit"`. Options: `JVM`, `Linux`, `Cygwin`, `MacOS`, `seL4`, `seL4_Only`, `seL4_TB`, `Microkit`, `ros2` |
| `output-dir` | string | No | Default output directory for generated code. Typically `<project-root>/hamr`. Generated code goes into a platform-specific subfolder (e.g., `hamr/microkit/` for Microkit) |
| `system-name` | string | No | Fully qualified name of the system to instantiate |
| `line` | integer | No | Line number containing the system to instantiate in the sysml file (default: 0) |
| `workspace-root-dir` | string | No | Root directory containing the architectural model project |

### Additional Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `verbose` | boolean | false | Enable verbose output |
| `bit-width` | enum (8,16,32,64) | - | Default bit-width for unbounded integer types |
| `max-array-size` | integer | 100 | Default sequence size |
| `max-string-size` | integer | 100 | Size for statically allocated strings |
| `devices-as-thread` | boolean | false | Treat AADL devices as threads |
| `exclude-component-impl` | boolean | false | Exclude Slang component implementations |
| `runtime-monitoring` | boolean | false | Enable runtime monitoring |
| `run-transpiler` | boolean | false | Run Transpiler during codegen |
| `no-proyek-ive` | boolean | false | Do not run Proyek IVE |
| `no-embed-art` | boolean | false | Do not embed ART project files |
| `strict-aadl-mode` | boolean | false | Generate strictly AADL-compliant code |
| `package-name` | string | "base" | Base package name for Slang project |
| `sbt-mill` | boolean | false | Generate SBT and Mill projects |

### Output Directory Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `output-dir` | string | Default output directory |
| `output-c-dir` | string | Output directory for C artifacts |
| `slang-output-dir` | string | Output directory for Slang project files |
| `sel4-output-dir` | string | Output directory for CAmkES/Microkit project files |
| `sel4-aux-code-dirs` | string[] | Directories containing C files to include in CAmkES/Microkit build |
| `aux-code-dirs` | string[] | Auxiliary C source code directories |

### ROS2-Specific Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `ros2-dir` | string | Path to ROS2 installation |
| `ros2-output-workspace-dir` | string | Path to ROS2 workspace |
| `ros2-nodes-language` | enum | `Python` or `Cpp` |
| `ros2-launch-language` | enum | `Python` or `Xml` |

### Configuration via Model Comments

Instead of passing all parameters via the MCP tool, HAMR supports configuration comments at the top of the main `.sysml` file:
```
//@ HAMR: --platform Microkit --output-dir ../../hamr
```
When present, code generation uses these options. MCP tool parameters can supplement or override these.

### Example Usage

Generate Microkit code for ProdCons:
```
Tool: sireum_hamr_sysml_codegen
args: ["<project-root>/sysmlv2/prod-cons/ProdCons.sysml"]
sourcepath: [
  "<project-root>/sysmlv2/aadl-lib",
  "<project-root>/sysmlv2/prod-cons"
]
platform: "Microkit"
output-dir: "<project-root>/hamr"
```

### What Gets Generated

Code generation produces the following in the output directory under a platform-specific subfolder (e.g., `hamr/microkit/`):

- **`components/`** -- C bridge code for each thread component (auto-generated, do not edit)
- **`crates/`** -- Rust crates for each thread component:
  - `<component>/src/component/<component>_app.rs` -- Developer-editable application logic with entry point skeletons
  - `<component>/src/bridge/` -- Auto-generated API, GUMBOX contracts, and extern C bindings
  - `<component>/src/test/` -- Auto-generated test infrastructure (PropTest, unit test support)
- **`crates/data/`** -- Auto-generated Rust data type definitions from model data types
- **`types/`** -- Shared type headers
- **`Makefile`**, **`system.mk`** -- Build infrastructure
- **`microkit.system`** -- Microkit system description XML
- **`microkit.schedule.xml`** -- Static cyclic schedule

Files marked `// Do not edit this file as it will be overwritten if codegen is rerun` are fully auto-generated. Files marked `// This file will not be overwritten if codegen is rerun` are for developer editing. Within developer-editable files, sections between `BEGIN MARKER` / `END MARKER` comments are auto-managed by HAMR and should not be manually modified.

---

## sireum_hamr_sysml_config -- Code Generation Configuration

**Purpose:** Opens an interactive form for configuring HAMR code generation options. Primarily useful in the CodeIVE GUI environment.

**When to use:** When setting up or modifying code generation configuration interactively.

### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `args` | string[] | No | Positional arguments; typically the path to the main `.sysml` file |
| `theme` | enum | No | Form color theme: `"dark"` or `"light"` |

### Note

For Claude-driven workflows, it is typically more practical to either:
1. Edit the `//@ HAMR:` configuration comment in the `.sysml` file directly, or
2. Pass configuration parameters directly to `sireum_hamr_sysml_codegen`

---

## sireum_hamr_sysml_logika -- HAMR SysMLv2 Logika Verifier

**Purpose:** Runs the Logika formal verifier on SysMLv2 models to check GUMBO integration constraints. This operates at the model level, verifying that GUMBO contracts (integration constraints, initialize guarantees, compute contracts) specified in SysMLv2 models are well-formed and consistent.

**When to use:** After adding or modifying GUMBO contracts in SysMLv2 models to verify that integration constraints are satisfiable and consistent.

### Key Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `args` | string[] | - | Positional arguments; paths to `.sysml` files to verify |
| `sourcepath` | string[] | - | Source paths of SysML v2 `.sysml` files (same convention as type checker) |
| `exclude` | string | - | Sourcepath exclusion as URI segment |
| `timeout` | integer | 2 | Timeout in seconds for validity checking |
| `rlimit` | integer | 2000000 | SMT2 solver resource limit |
| `line` | integer | 0 | Focus verification to a specific line number |
| `par` | integer | 100 | Parallelization percentage of CPU cores to use |

### Solver Configuration

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `solver-valid` | string | `"cvc4,--full-saturate-quant; z3; cvc5,--full-saturate-quant"` | SMT2 configurations for validity queries |
| `solver-sat` | string | `"z3"` | SMT2 configurations for satisfiability queries |
| `sat` | boolean | false | Enable assumption satisfiability checking |

### Verification Control

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `interprocedural` | boolean | false | Enable inter-procedural verification |
| `interprocedural-contracts` | boolean | false | Use contracts in inter-procedural verification |
| `loop-bound` | integer | 3 | Loop bound for inter-procedural verification |
| `recursive-bound` | integer | 3 | Recursive call-chain bound |
| `skip-methods` | string | - | Skip methods with specified fully-qualified names |
| `skip-types` | string | - | Skip specified traits, classes, and objects |

### Splitting Options (for managing verification complexity)

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `split-all` | boolean | false | Split all verification conditions |
| `split-contract` | boolean | false | Split on contract cases |
| `split-if` | boolean | false | Split on if-conditional expressions |
| `split-match` | boolean | false | Split on match expressions |

### Debugging and Diagnostics

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `log-vc` | boolean | false | Display all verification conditions |
| `log-vc-dir` | string | - | Write verification conditions to a directory |
| `log-pc` | boolean | false | Display path conditions before each statement |
| `log-detailed-info` | boolean | false | Display detailed feedback |
| `elide-encoding` | boolean | false | Strip SMT2 encoding from feedback |
| `stats` | boolean | false | Collect verification statistics |

### Example Usage

Verify GUMBO integration constraints for ProdCons:
```
Tool: sireum_hamr_sysml_logika
args: ["<project-root>/sysmlv2/prod-cons/ProdCons.sysml"]
sourcepath: [
  "<project-root>/sysmlv2/aadl-lib",
  "<project-root>/sysmlv2/prod-cons"
]
```

---

## Typical HAMR Development Workflow Using MCP Tools

1. **Create/Edit SysMLv2 model** -- Write `.sysml` files defining the system architecture
2. **Type check** -- Run `sireum_hamr_sysml_tipe` to validate the model
3. **Generate code** -- Run `sireum_hamr_sysml_codegen` with platform `Microkit` to generate Rust scaffolding
4. **Implement application logic** -- Edit `*_app.rs` files in each component's `component/` directory
5. **Run tests** -- Use the generated test infrastructure (PropTest) to test component implementations
6. **Iterate** -- Modify models or code and repeat from step 2

Optional:
1. **Add GUMBO contracts in SysMLv2 models** -- Edit `.sysml` files to include behavior specifications written in the GUMBO contract language
2. **Check SysMLv2 GUMBO Integrations** -- Run `sireum_hamr_sysml_logika` to verify that GUMBO integration constraints in the SysMLv2 models are satisfied

## Other Available MCP Tools

The Sireum MCP server exposes additional tools beyond those documented above. Some that may be useful in HAMR projects include:

- **`sireum_hamr_sysml_translator`** -- Translate SysMLv2 models between representations
- **`sireum_logika_verifier`** -- Logika formal verifier for Slang (Scala) source files
- **`sireum_logika_config`** -- Configure Logika verifier settings
- **`sireum_proyek_compile`** -- Compile a Sireum/Slang project
- **`sireum_proyek_test`** -- Run project tests
- **`sireum_proyek_run`** -- Run a Sireum project
- **`sireum_slang_tipe`** -- Type check Slang code
- **`sireum_tools_slangcheck_generator`** -- Generate SlangCheck test harnesses
- **`sireum_tools_slangcheck_runner`** -- Run SlangCheck tests
- **`sireum_tools_slangcheck_tester`** -- SlangCheck testing
