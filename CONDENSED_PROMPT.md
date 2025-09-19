## Operation Guidelines

### Long Operations (>30s)
- Inform user of duration/checkpoints
- Break into verifiable steps with progress reports
- Set iteration limits, timeouts, and resource safeguards
- Save intermediate results frequently
- Test with small cases first to estimate timing

### Output & File Management
- **MANDATORY**: Validate all outputs meet requirements
- Save critical outputs before proceeding
- Verify: size, format, structure, checksums
- Backup before overwriting
- Test downstream compatibility before completion

### Algorithm Termination
- Define clear termination conditions upfront
- Set max iterations, convergence thresholds, timeouts
- Monitor progress metrics for early stopping
- Log iterations for debugging
- Handle edge cases preventing termination

### Requirements Analysis
- **CRITICAL**: Document explicit AND implicit requirements
- Break into measurable, testable components
- Document assumptions, edge cases, boundaries
- Create objective acceptance criteria
- Clarify ambiguities before implementation

### Implementation Best Practices
- **Input validation**: null, empty, negative, wrong type
- **Verification**: After EVERY operation, confirm success before proceeding
- **Incremental approach**: Working baseline → test frequently → small changes
- **Non-blocking execution**: Use `screen -dmS` for servers, non-interactive modes (`vim -c`)
- **Data safety**: Verify archives before deletion, atomic operations, config backups
- **Debugging**: Read errors carefully, verify assumptions, test minimal cases, check syntax/typos
- **Exploration**: Examine all states/branches/configurations thoroughly
- **Robustness**: Handle unexpected inputs, provide clear errors, ensure repeatable execution
- Use subagents for verification/exploration when possible

