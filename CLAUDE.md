# Project Purpose

Develop context material used to guide Claude in helping software engineers apply the HAMR model driven development toolchain.  The context material is held in the @hamr-claude-training folder and includes documentation, example systems, as well as a CLAUDE.md file the describes the context material to Claude.  When a developer works on future HAMR-based project (a project developed with the HAMR tool chain), they should be able to point Claude to the @hamr-claude-training folder to provide Claude with the appropriate context information for automating development tasks for that project.  Pointing out the material to Claude could be done in multiple ways: having it on a website and having Claude access the website, or copy the folder into a local project.

# Needs

Ask Claude to help improve the training material so that Claude can use it to work most effectively with HAMR projects.  I am not a Claude expert so I need advice on things like
 - designing lower-level CLAUDE.md files
 - designing SKILLS.md (if appropriate)
 - formatting the training material to make it easier for Claude to access
 - making effective use of HAMR's MCP interfaces (and providing instructures to developers for that)

# Development Approach

To evaluate the effectiveness of the @hamr-claude-training contents and to discover what additional information needs to be added to it to improve Claude performance, an evaluation HAMR project is in the @evaluation-project folder.  Claude will be asked to consult the @hamr-clause-training material and then help perform development tasks for the @evaluation-project.  Based on situations where Claude doesn't perform a task well on the evaluation project, the contents of @hamr-claude-training will be updated to provided better context information to Claude.

# Assessment

My colleagues and I are trying to understand how to best use Claude.  Therefore we would like Claude to produce a log of interactions with it.



