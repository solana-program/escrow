---
name: add-extension
description: Generate a new extension for the escrow program
---

Use the extension-generator agent to create a new escrow program extension.

This command will guide you through:

1. Understanding your extension requirements
2. Generating all necessary code files
3. Updating integration points
4. Creating tests and fixtures
5. Validating the generated code
6. Running `just test` in a loop until all tests pass (no skipped or commented tests)

The agent will ask questions about:

- Extension name and purpose
- Data structure and fields
- Validation logic
- Custom errors
- Event payload

Start by describing the extension you want to add.
