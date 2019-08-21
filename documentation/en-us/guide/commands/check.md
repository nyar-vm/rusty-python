# Syntax and Semantic Check (Check)

The `rbq check` command is used to verify the correctness of `.rbq` files, including syntax parsing and semantic analysis.

## Common Commands

### Check Specified Files or Directories
```bash
rbq check ./models
```

## Check Content

1.  **Syntax Check**：Ensure compliance with RBQ language specifications (such as `;` at the end, correct type definitions, etc.).
2.  **Semantic Check**：
    - Whether type references exist.
    - Circular reference detection.
    - Annotation parameter legality verification.
    - Namespace conflict check.

## CI/CD Integration

It is recommended to run `rbq check` in code commit hooks (Git Hooks) or CI pipelines to ensure that model definitions entering the codebase are always valid.

## Configuration File Options

Check-related options in the `rbq.toml` configuration file:

### [project] Section
- `source_dir`: RBQ source file directory, containing all `.rbq` files, default is "models"

### [databases] Section
- `url`: Database connection string, format depends on database type
- `schema`: Optional database schema name, such as "public" for PostgreSQL