# GUMBO BNF Reference (SysMLv2 Version)

Derived from the ANTLR grammar at `sireum/hamr-sysml-parser GUMBO.g4`,
filtered to productions actually used in GUMBO practice, and augmented
with built-in predicates and operators that are realized as uninterpreted
functions in KerML (so they do not appear in the grammar file).

Conventions:
- `?` = optional, `*` = zero-or-more, `+` = one-or-more
- `|` separates alternatives
- `'keyword'` = literal terminal
- UPPER_CASE = lexical tokens
- Semicolons are terminators (required after each clause body expression).

Sources consulted: SNG evaluation project, isolette-simple, ProdCons,
ardupilot-basic (INSPECTA-Open-Platform), isolette (INSPECTA-models).

---

## 1. Embedding in SysMLv2

GUMBO blocks appear inside `part def` (thread or data type) or `package`
declarations using the SysMLv2 `language` annotation:

```
language "GUMBO" /*{ <GumboContent> }*/
```

The delimiters MUST be `/*{` and `}*/` — not `{` and `}`.

A GUMBO block at package level is a **library** (GumboLibrary).
A GUMBO block inside a `part def :> Thread` is a **subclause** (GumboSubclause).
A GUMBO block inside a `part def :> Data` is a **data invariant**.

---

## 2. Top-Level Structure

```bnf
GumboContent       ::= GumboLibrary | GumboSubclause | DataInvariants

(* Package-level GUMBO block — defines reusable functions *)
GumboLibrary       ::= 'library' Functions

(* Thread-level GUMBO block — specifies component behavior *)
GumboSubclause     ::= State? Functions? Invariants? Integration?
                        Initialize? Compute?

(* Data-type-level GUMBO block — constrains data values *)
DataInvariants     ::= Invariants
```

---

## 3. State Declarations

Declare component-local state variables visible in contracts.
These generate fields in the Rust component struct.

```bnf
State              ::= 'state' StateVarDecl+
StateVarDecl       ::= ID ':' QualifiedName ';'
```

Example:
```
state
    lastCmd: Isolette_Data_Model::On_Off;
    payload_sum: Base_Types::Integer_32;
```

---

## 4. Functions

Define pure helper functions usable in contracts.
In a `library` block, these become available to other packages via
qualified name `PackageName::GUMBO__Library::functionName(...)`.
In a subclause, they are local to the thread definition.

```bnf
Functions          ::= 'functions' FuncDef+

FuncDef            ::= DefMods? 'def' ID FuncParams ':' Type
                       (':=' DefContract? Expr)? ';'

DefMods            ::= '@strictpure' | '@pure' | '@spec'

FuncParams         ::= '(' (FuncParam (',' FuncParam)*)? ')'
FuncParam          ::= ID ':' Type

(* Contract on function body — rarely used in practice *)
DefContract        ::= (Reads ';')? (FuncRequires ';')?
                       (Modifies ';')? (FuncEnsures ';')?
Reads              ::= 'reads' Expr (',' Expr)*
FuncRequires       ::= 'assume' Expr (',' Expr)*
Modifies           ::= 'modifies' Expr (',' Expr)*
FuncEnsures        ::= 'guarantee' Expr (',' Expr)*
```

Example (library):
```
functions
    def clampedPayload(m: SNG_Data_Model::Message): Base_Types::Boolean :=
        (clampedPayloadLowerBound() <= m.payload)
        and (m.payload <= clampedPayloadUpperBound());
```

Example (subclause):
```
functions
    def Temp_Lower_Bound(): Base_Types::Integer_32 := 95 [i32];
```

---

## 5. Invariants

Constrain data type values or component state. Used in `part def :> Data`
blocks and occasionally in subclauses.

```bnf
Invariants         ::= 'invariants' InvSpec+
InvSpec            ::= 'inv' ID STRING? ':' Expr ';'
```

Example (on a data type):
```
invariants
    inv MaxMinEarthTemperatures "Temps within recorded range" :
        -128 [s32] <= degrees and degrees <= 134 [s32];
```

---

## 6. Integration Constraints

Constraints on port values at component boundaries. Assumes constrain
inputs (what this component may receive); guarantees constrain outputs
(what this component promises to send). Checked across connections
by HAMR Logika.

Integration constraints apply to the **payload** of the port directly —
the port name refers to the unwrapped value, not `Option<T>`.

```bnf
Integration        ::= 'integration' SpecStatement+
SpecStatement      ::= AssumeStatement | GuaranteeStatement
```

Example:
```
integration
    assume No_Critical_Input:
        GumboLib::GUMBO__Library::allowedSecurityLevel(input);
    guarantee No_Critical_Output:
        GumboLib::GUMBO__Library::allowedSecurityLevel(output);
```

---

## 7. Initialize Contracts

Constrain the initialize entry point. Only guarantee clauses allowed
(no assumes — there are no inputs during initialization).
May include a `modifies` clause for state variables.

```bnf
Initialize         ::= 'initialize' (Modifies ';')? GuaranteeStatement*
                        InfoFlowClause*
```

Example:
```
initialize
    guarantee initlastCmd: lastCmd == Isolette_Data_Model::On_Off.Off;
    guarantee REQ_THERM_1 "Heat Control Off initially":
        heat_control == Isolette_Data_Model::On_Off.Off;
```

---

## 8. Compute Contracts

Constrain the compute entry point (timeTriggered for periodic threads).
Supports three contract styles that can be mixed:

1. **General assumes/guarantees** — top-level preconditions and postconditions
2. **Compute cases** — case-by-case specifications with paired assume/guarantee
3. **Handler clauses** — for sporadic threads, per-port event handlers

```bnf
Compute            ::= 'compute' (Modifies ';')?
                        AssumeStatement*
                        GuaranteeStatement*
                        ComputeCases*
                        HandlerClause*
                        InfoFlowClause*

ComputeCases       ::= 'compute_cases' CaseClause+

CaseClause         ::= 'case' ID STRING? ':'
                        AnonAssume? AnonGuarantee

(* Assumes/guarantees in case clauses are anonymous — no name *)
AnonAssume         ::= 'assume' Expr ';'
AnonGuarantee      ::= 'guarantee' Expr ';'
```

### Style A: Guarantee-only with `implies` (e.g., SNG Gate)

Each guarantee covers one scenario using `implies` to guard:

```
compute
    guarantee Req_C_Drop_Critical:
        HasEvent(input) and (input.security_level == SecurityLevel.Critical)
        implies NoSend(output);
    guarantee No_Input_No_Output:
        (not HasEvent(input)) implies NoSend(output);
```

### Style B: Compute cases (e.g., SNG Filter, Isolette Thermostat)

Cases partition the input space with assume/guarantee pairs:

```
compute
  compute_cases
    case Req_P_Public_Pass:
        assume HasEvent(input) and (input.security_level == SecurityLevel.Public);
        guarantee HasEvent(output) and equalMessage(input, output);
    case No_Input:
        assume (not HasEvent(input));
        guarantee NoSend(output);
```

### Style C: Mixed general + cases (e.g., Isolette Thermostat)

A general guarantee combined with compute_cases:

```
compute
    assume ASSM_LDT_LE_UDT: desired_temp.lower.degrees <= desired_temp.upper.degrees;
    guarantee lastCmd "Set lastCmd": lastCmd == heat_control;
    compute_cases
        case REQ_THERM_2: ...
        case REQ_THERM_3: ...
```

---

## 9. Handler Clauses (Sporadic Threads)

For sporadic threads, contracts are specified per triggering event port:

```bnf
HandlerClause      ::= 'handle' ID ':'
                        (Modifies ';')?
                        AssumeStatement*
                        GuaranteeStatement*
                        ComputeCases*
```

---

## 10. Named Assumes and Guarantees

```bnf
AssumeStatement    ::= 'assume' ID STRING? ':' Expr ';'
GuaranteeStatement ::= 'guarantee' ID STRING? ':' Expr ';'
```

The STRING is an optional documentation string. Multi-line strings
use `|` as a continuation character:

```
guarantee REQ_THERM_2 "If Current Temperature is less than
                      |the Lower Desired Temperature, the
                      |Heat Control shall be set to On.":
    heat_control == Isolette_Data_Model::On_Off.Onn;
```

---

## 11. Information Flow Clauses

Declare information flow relationships (rarely used in practice):

```bnf
InfoFlowClause     ::= 'infoflow' ID STRING? ':'
                        'from' '(' (ID (',' ID)*)? ')' ','
                        'to' '(' (ID (',' ID)*)? ')' ';'
```

---

## 12. Expressions

### Precedence (lowest to highest)

```bnf
Expr               ::= CondExpr

(* if-then-else *)
CondExpr           ::= ImpliesExpr
                     | 'if' ImpliesExpr '?' Expr 'else' Expr

(* logical implication — right-associative *)
ImpliesExpr        ::= OrExpr ('implies' OrExpr)*

(* logical or — 'or' is short-circuit, '|' is non-short-circuit *)
OrExpr             ::= XorExpr (('or' | '|') XorExpr)*

(* logical xor *)
XorExpr            ::= AndExpr ('xor' AndExpr)*

(* logical and — 'and' is short-circuit, '&' is non-short-circuit *)
AndExpr            ::= EqExpr (('and' | '&') EqExpr)*

(* equality *)
EqExpr             ::= RelExpr (('==' | '!=') RelExpr)*

(* relational *)
RelExpr            ::= AddExpr (('<' | '>' | '<=' | '>=') AddExpr)*

(* arithmetic *)
AddExpr            ::= MulExpr (('+' | '-') MulExpr)*
MulExpr            ::= ExpExpr (('*' | '/' | '%') ExpExpr)*
ExpExpr            ::= UnaryExpr (('**' | '^') ExpExpr)?

(* unary *)
UnaryExpr          ::= ('+' | '-' | '~' | 'not') UnaryExpr
                     | PostfixExpr
```

### Postfix and Primary Expressions

```bnf
PostfixExpr        ::= PrimaryExpr ('.' FieldAccess)*

(* Field access and array indexing *)
FieldAccess        ::= ID
                     | '#' '(' Expr ')'

PrimaryExpr        ::= Literal
                     | PortOrVarRef
                     | FunctionCall
                     | BuiltInPredicate
                     | '(' Expr ')'
```

### Literals

```bnf
Literal            ::= BoolLiteral | IntLiteral | RealLiteral | StringLiteral

BoolLiteral        ::= 'true' | 'false'

(* Typed integer literal — type suffix in brackets *)
IntLiteral         ::= DECIMAL_VALUE '[' TypeSuffix ']'
                     | DECIMAL_VALUE

(* Common type suffixes *)
TypeSuffix         ::= 'i32' | 's32' | 'u8' | 'u16' | 'u32' | 'i64'

RealLiteral        ::= DECIMAL_VALUE? '.' DECIMAL_VALUE

StringLiteral      ::= '"' ... '"'
```

### References

```bnf
(* Port name, state variable, enum value, or function parameter *)
PortOrVarRef       ::= QualifiedName

(* Function call — may be to local, library, or subclause function *)
FunctionCall       ::= QualifiedName '(' (Expr (',' Expr)*)? ')'
```

---

## 13. Built-In Predicates (Not in Grammar)

These are realized as uninterpreted KerML functions. They do NOT
appear in the GUMBO.g4 grammar — they parse as ordinary function
calls — but they have special semantics in HAMR.

### Event Port Predicates (EventDataPort only)

```
HasEvent(portName)    -- true if the port received a value this dispatch
NoSend(portName)      -- guarantees no value is sent on this output port
MustSend(portName)    -- guarantees a value IS sent on this output port
```

- `HasEvent` is used in assumes and guarantees for **both** input and output ports
- `NoSend` is used **only** in guarantees on **output** ports
- `MustSend` is used **only** in guarantees on **output** ports
- There is NO `NoEvent` predicate — use `not HasEvent(portName)` instead

When `HasEvent(port)` is true, `port` refers to the unwrapped payload value
(not `Option<T>`). Always guard field access with `HasEvent`:

```
HasEvent(input) and (input.security_level == SecurityLevel.Public)
```

### State Variable Previous-Value Reference

```
In(stateVarName)      -- value of state variable at start of current dispatch
```

Used in compute contracts to reference the previous-dispatch value:

```
guarantee heat_control == In(lastCmd);
```

### Implication Operator (Alternative Syntax)

```
'->:' (antecedent, consequent)
```

Function-call-style implication, equivalent to `antecedent implies consequent`.
Used in some projects (isolette INSPECTA-models). Example:

```
def Allowed_LowerAlarmTempWStatus(lower: TempWstatus_i): Base_Types::Boolean :=
    '->:' (isValidTempWstatus(lower), Allowed_LowerAlarmTemp(lower.degrees));
```

---

## 14. Qualified Names and Naming Conventions

```bnf
QualifiedName      ::= (Name '::')* Name

Name               ::= ID | UNRESTRICTED_NAME

ID                 ::= [a-zA-Z_][a-zA-Z0-9_]*
UNRESTRICTED_NAME  ::= '\'' ... '\''
```

### Library Function Qualification

When a GUMBO `library` block is defined inside a package, the `library`
keyword creates an intermediate namespace called `GUMBO__Library`.
To call library functions from other packages:

```
PackageName::GUMBO__Library::functionName(args)
```

Example: `GumboLib::GUMBO__Library::allowedSecurityLevel(output)`

Within the same library block, functions call each other unqualified:
`clampedPayloadLowerBound()`.

### Enum Value Notation

Enum values use **dot** notation (not `::`):

```
Package::EnumDef.EnumValue
```

Example: `SNG_Data_Model::SecurityLevel.Public`

Note the asymmetry: `::` separates packages/types, `.` separates
the enum type from its value.

### Field Access

Struct fields use dot notation. Port names resolve to their payload type:

```
input.security_level           -- field of the port's data type
input.payload                  -- another field
desired_temp.lower.degrees     -- nested struct field access
EthernetFramesTx0.amessage     -- struct field of port payload
```

### Array Element Access

Array elements use `#(index)` syntax (not `[index]`):

```
aframe#(12)                    -- element at index 12
```

---

## 15. Types in GUMBO

```bnf
Type               ::= QualifiedName
```

Common base types:

| GUMBO Type | Description |
|---|---|
| `Base_Types::Boolean` | Boolean |
| `Base_Types::Integer_32` | 32-bit signed integer |
| `Base_Types::Integer_8` | 8-bit signed integer |
| `Base_Types::Unsigned_8` | 8-bit unsigned integer |
| `Base_Types::Unsigned_16` | 16-bit unsigned integer |
| `Base_Types::Float_32` | 32-bit float |

Application types use package-qualified names: `SNG_Data_Model::Message`.

---

## 16. Lexical Tokens

```bnf
DECIMAL_VALUE      ::= [0-9]+
ID                 ::= [a-zA-Z_][a-zA-Z0-9_]*
STRING             ::= '"' (escape | ~["\\])* '"'
COMMENT            ::= '//' ...           (* line comment, ignored *)
```

---

## 17. DataPort vs EventDataPort Contract Patterns

**DataPort** — always has a value after initialization. Port name refers
directly to the value. No `HasEvent`/`NoSend` needed:

```
integration
    guarantee temp_range:
        96 [i32] <= current_temp.degrees & current_temp.degrees <= 103 [i32];
```

**EventDataPort** — may or may not have a value each dispatch. Use
`HasEvent` to guard access, `NoSend` to specify no output:

```
compute
    guarantee Req_C_Drop_Critical:
        HasEvent(input) and (input.security_level == SecurityLevel.Critical)
        implies NoSend(output);
```

Integration constraints on EventDataPort apply to the unwrapped
value (when present) — no `HasEvent` guard needed in integration blocks.
