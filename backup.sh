#!/bin/bash

# Arcadia Backup Script
# This script backs up the database data and environment files

set -e  # Exit on any error

# Configuration
BACKUP_DIR="backup_$(date +%Y%m%d_%H%M%S)"
ZIP_FILE="arcadia_backup_$(date +%Y%m%d_%H%M%S).zip"
DB_CONTAINER="arcadia_db"
DB_NAME="arcadia"
DB_USER="arcadia"

echo "Starting Arcadia backup..."

# Create temporary backup directory
mkdir -p "$BACKUP_DIR"

# Function to cleanup on exit
cleanup() {
    echo "Cleaning up temporary files..."
    rm -rf "$BACKUP_DIR"
}
trap cleanup EXIT

# Check if Docker is running
if ! docker info >/dev/null 2>&1; then
    echo "Error: Docker is not running or not accessible"
    exit 1
fi

# Check if database container is running
if ! docker ps --format "table {{.Names}}" | grep -q "^$DB_CONTAINER$"; then
    echo "Error: Database container '$DB_CONTAINER' is not running"
    echo "Please start the database with: docker compose up db -d"
    exit 1
fi

echo "Backing up database..."
# Backup database data only (no schema)
docker exec "$DB_CONTAINER" pg_dump -U "$DB_USER" -d "$DB_NAME" --data-only --no-owner --no-privileges > "$BACKUP_DIR/database_data.sql"

if [ $? -eq 0 ]; then
    echo "Database backup completed successfully"
else
    echo "Error: Database backup failed"
    exit 1
fi

echo "Copying environment files..."
# Copy backend .env file if it exists
if [ -f "backend/.env" ]; then
    cp "backend/.env" "$BACKUP_DIR/backend.env"
    echo "Backend .env file copied"
else
    echo "Warning: backend/.env file not found, skipping..."
fi

# Copy frontend .env file if it exists
if [ -f "frontend/.env" ]; then
    cp "frontend/.env" "$BACKUP_DIR/frontend.env"
    echo "Frontend .env file copied"
else
    echo "Warning: frontend/.env file not found, skipping..."
fi

# Create backup info file
echo "Backup created on: $(date)" > "$BACKUP_DIR/backup_info.txt"
echo "Database: $DB_NAME" >> "$BACKUP_DIR/backup_info.txt"
echo "Container: $DB_CONTAINER" >> "$BACKUP_DIR/backup_info.txt"
echo "Backup type: Data only" >> "$BACKUP_DIR/backup_info.txt"

echo "Creating zip archive..."
# Create zip file
if command -v zip >/dev/null 2>&1; then
    zip -r "$ZIP_FILE" "$BACKUP_DIR"
else
    echo "Error: zip command not found. Please install zip utility"
    exit 1
fi

if [ $? -eq 0 ]; then
    echo "Backup completed successfully!"
    echo "Backup file: $ZIP_FILE"
    echo "Backup size: $(du -h "$ZIP_FILE" | cut -f1)"
else
    echo "Error: Failed to create zip archive"
    exit 1
fi

echo "Backup process finished."