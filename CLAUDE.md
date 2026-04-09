# Project Purpose

Develop context material used to guide Claude in helping software engineers apply the HAMR model driven development toolchain.  The context material is held in the @hamr-claude-training folder and includes documentation, example systems, as well as a CLAUDE.md file the describes the context material to Claude.  When a developer works on future HAMR-based project (a project developed with the HAMR tool chain), they should be able to point Claude to the @hamr-claude-training folder to provide Claude with the appropriate context information for automating development tasks for that project.  Pointing out the material to Claude could be done in multiple ways: having it on a website and having Claude access the website, or copy the folder into a local project.

# Needs

Ask Claude to help improve the training material so that Claude can use it to work most effectively with HAMR projects.  I am not a Claude expert so I need advice on things like
 - designing lower-level CLAUDE.md files
 - designing SKILLS.md (if appropriate)
 - formatting the training material to make it easier for Claude to access
 - making effective use of HAMR's MCP interfaces (and providing instructures to developers for that)

# Development Approach

To evaluate the effectiveness of the @hamr-claude-training contents and to discover what additional information needs to be added to it to improve Claude performance, an evaluation HAMR project is placed in the @evaluation-project folder.  Claude is asked to consult the @hamr-claude-training material and then help perform development tasks for the evaluation project.  Based on situations where Claude doesn't perform a task well, the contents of @hamr-claude-training are updated to provide better context information to Claude.  When an evaluation is complete, the project artifacts are promoted into `hamr-claude-training/examples/` as an additional training example.

The first evaluation (Simple Network Guard / SNG) is complete and now lives at `hamr-claude-training/examples/HAMR-SysMLv2-Rust-seL4-P-EDP-SNG-Example/`.  The @evaluation-project folder is ready for the next evaluation project.

# Change Reports

When changes span multiple artifact levels (requirements, models, code), Claude should generate an auditable change report following the template in `@hamr-claude-training/examples/HAMR-SysMLv2-Rust-seL4-P-EDP-SNG-Example/reports/CHANGE-REPORT-GUIDE.md`. Reports are stored in the evaluation project's `reports/` folder with the naming convention `CR-<NNN>-<short-description>.md`. The report must include traceability matrices connecting requirements to model elements, generated code, developer-written code, and tests. See the SNG example's `CR-001-security-level-rename.md` for a reference.

# Assessment

My colleagues and I are trying to understand how to best use Claude.  Therefore we would like Claude to produce a log of interactions with it.



