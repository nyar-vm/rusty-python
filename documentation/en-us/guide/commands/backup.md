# Backup & Restore

Data security is a core concern of RBQ. The RBQ CLI provides a unified interface that shields the parameter differences of backup tools from different engines (such as `pg_dump`, `mysqldump`).

## 1. Logical Backup

Logical backup exports data to RBQ's internal LIR serialization format or standard SQL. This method has good cross-version compatibility and is suitable for small and medium-sized databases.

### 1.1 Perform Backup

```bash
rbq backup --output ./backups/20240520.rbqdump
```

### 1.2 Export as Compressed Format

```bash
rbq backup --compress gzip --output ./backups/full.sql.gz
```

## 2. Physical Backup

For large databases (such as TB-level PostgreSQL), the RBQ driver layer supports calling native snapshot or physical replication tools.

```bash
# Requires target engine to support physical backup plugin
rbq backup --type physical --target s3://my-bucket/db-backups
```

## 3. Data Restore

### 3.1 Restore from File

```bash
rbq restore --from ./backups/20240520.rbqdump
```

### 3.2 Security Check Before Restore

RBQ will perform Schema verification by default before executing `restore`. If the Schema of the backup file does not match the `.rbq` model in the current code, the system will display a warning.

```bash
# Ignore Schema matching verification (force restore)
rbq restore --from ./dump.rbq --force
```

## 4. Scheduled Backups

Although the RBQ CLI does not directly run Cron tasks itself, it provides exit codes and log formats that are easy to integrate, facilitating integration with system-level scheduling tools:

```bash
# Linux Crontab example: backup at 2 AM every day
0 2 * * * /usr/local/bin/rbq backup --output /data/backup/daily.rbqdump >> /var/log/rbq-backup.log 2>&1
```

***

## 5. Cross-Engine Data Migration

Thanks to RBQ's unified IR, you can use `backup` and `restore` to implement low-cost database switching:

- Scenario: Migrate from SQLite to PostgreSQL.
- Operation:
  1. Connect to SQLite, execute `rbq backup` to generate a unified format dump.
  2. Modify the `database` configuration to point to PostgreSQL.
  3. Execute `rbq migrate up` to initialize the table structure.
  4. Execute `rbq restore` to import the data.

## 6. Configuration File Options

Backup/restore-related options in the `rbq.toml` configuration file:

### [databases] Section

- `url`: Database connection string, format depends on database type
- `schema`: Optional database schema name, such as "public" for PostgreSQL

### [databases.<name>.pool] Section (Recommended for Production Environment)

- `max_connections`: Maximum number of connections
- `min_connections`: Minimum number of connections
- `idle_timeout`: Idle connection timeout