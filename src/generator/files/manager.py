"""
Files table interface for Dragon's Labyrinth.

Provides idempotency tracking and blob storage for pipeline-generated files.
Replaces the old IdempotencyStore with a proper SQLModel-based interface.
"""

from __future__ import annotations

import hashlib
import json
from datetime import datetime
from pathlib import Path
from typing import Any

from sqlmodel import Session, select, func

from dragons_labyrinth.db.files.models import FileRecord
from dragons_labyrinth.db.files.constants import (
    DEFAULT_OCTET_STREAM, DEFAULT_TEXT_PLAIN, DEFAULT_JSON, 
    BYTES_TO_MB_DIVISOR, DEFAULT_FILE_ENCODING
)


class FilesMixin:
    """
    Mixin providing files table functionality.
    
    Provides methods for:
    - Idempotency tracking (replacing old IdempotencyStore)
    - Blob storage for generated content
    - File change detection
    - Pipeline file tracking
    
    Expected: self.session (Session) to be available from inheriting class
    """
    
    def _hash_content(self, content: str | bytes) -> str:
        """Generate SHA256 hash of content."""
        if isinstance(content, str):
            content = content.encode(DEFAULT_FILE_ENCODING)
        return hashlib.sha256(content).hexdigest()
    
    def record_file(
        self,
        key: str,
        content: str | bytes,
        file_path: str | None = None,
        content_type: str | None = None,
        pipeline_source: str | None = None,
        pipeline_stage: str | None = None,
        validated: bool = True,
        error_details: str | None = None
    ) -> FileRecord:
        """
        Record a file for idempotency tracking.
        
        Args:
            key: Unique idempotency key
            content: File content (string or bytes)
            file_path: Associated file path
            content_type: MIME type or content classification
            pipeline_source: Pipeline that created this file
            pipeline_stage: Processing stage identifier
            validated: Whether content passed validation
            error_details: Error details if validation failed
            
        Returns:
            The created or updated file record
        """
        content_hash = self._hash_content(content)
        file_size = len(content) if isinstance(content, bytes) else len(content.encode(DEFAULT_FILE_ENCODING))
        
        # Check if record exists
        statement = select(FileRecord).where(FileRecord.key == key)
        existing = self.session.exec(statement).first()
        
        if existing:
            # Update existing record
            existing.hash = content_hash
            existing.updated_at = datetime.utcnow()
            existing.file_path = file_path
            existing.file_size = file_size
            existing.content_type = content_type
            existing.pipeline_source = pipeline_source
            existing.pipeline_stage = pipeline_stage
            existing.validated = validated
            existing.error_details = error_details
            
            self.session.add(existing)
            self.session.commit()
            return existing
        else:
            # Create new record
            new_record = FileRecord(
                key=key,
                hash=content_hash,
                created_at=datetime.utcnow(),
                updated_at=datetime.utcnow(),
                file_path=file_path,
                file_size=file_size,
                content_type=content_type,
                pipeline_source=pipeline_source,
                pipeline_stage=pipeline_stage,
                validated=validated,
                error_details=error_details
            )
            
            self.session.add(new_record)
            self.session.commit()
            return new_record
    
    def has_changed(self, key: str, content: str | bytes) -> bool:
        """
        Check if content has changed since last recording.
        
        Args:
            key: Idempotency key
            content: Current content to check
            
        Returns:
            True if content has changed or is new, False if unchanged
        """
        content_hash = self._hash_content(content)
        statement = select(FileRecord).where(FileRecord.key == key)
        existing = self.session.exec(statement).first()
        
        if not existing:
            return True  # New content
        
        return existing.hash != content_hash
    
    def write_if_changed(
        self,
        path: Path,
        content: str | bytes,
        key: str,
        pipeline_source: str | None = None,
        pipeline_stage: str | None = None
    ) -> bool:
        """
        Write content to file only if it has changed.
        
        Args:
            path: Path to write to
            content: Content to write
            key: Idempotency key
            pipeline_source: Pipeline that created this file
            pipeline_stage: Processing stage identifier
            
        Returns:
            True if file was written, False if unchanged
        """
        if not self.has_changed(key, content):
            if path.exists():
                return False  # Content unchanged and file exists
        
        # Write the file
        path.parent.mkdir(parents=True, exist_ok=True)
        
        if isinstance(content, bytes):
            path.write_bytes(content)
            content_type = DEFAULT_OCTET_STREAM
        else:
            path.write_text(content, encoding=DEFAULT_FILE_ENCODING)
            content_type = DEFAULT_TEXT_PLAIN
        
        # Record in database
        self.record_file(
            key=key,
            content=content,
            file_path=str(path),
            content_type=content_type,
            pipeline_source=pipeline_source,
            pipeline_stage=pipeline_stage
        )
        
        return True
    
    def write_json_if_changed(
        self,
        path: Path,
        data: dict[str, Any],
        key: str,
        pipeline_source: str | None = None,
        pipeline_stage: str | None = None
    ) -> bool:
        """
        Write JSON data to file only if it has changed.
        
        Args:
            path: Path to write to
            data: JSON data to write
            key: Idempotency key
            pipeline_source: Pipeline that created this file
            pipeline_stage: Processing stage identifier
            
        Returns:
            True if file was written, False if unchanged
        """
        content = json.dumps(data, sort_keys=True, ensure_ascii=False, indent=2)
        
        if not self.has_changed(key, content):
            if path.exists():
                return False
        
        # Write the file
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding=DEFAULT_FILE_ENCODING)
        
        # Record in database
        self.record_file(
            key=key,
            content=content,
            file_path=str(path),
            content_type=DEFAULT_JSON,
            pipeline_source=pipeline_source,
            pipeline_stage=pipeline_stage
        )
        
        return True
    
    def store_blob(
        self,
        key: str,
        content: bytes,
        content_type: str | None = None,
        pipeline_source: str | None = None,
        pipeline_stage: str | None = None
    ) -> FileRecord:
        """
        Store binary content without writing to file.
        
        This is useful for intermediate pipeline data or generated images
        that should be tracked but not immediately written to disk.
        
        Args:
            key: Unique key for this blob
            content: Binary content
            content_type: MIME type
            pipeline_source: Pipeline that created this blob
            pipeline_stage: Processing stage identifier
            
        Returns:
            The created or updated file record
        """
        return self.record_file(
            key=key,
            content=content,
            file_path=None,  # No file path for blobs
            content_type=content_type or DEFAULT_OCTET_STREAM,
            pipeline_source=pipeline_source,
            pipeline_stage=pipeline_stage
        )
    
    def flush_blob_to_file(
        self,
        key: str,
        path: Path,
        content: bytes | None = None
    ) -> bool:
        """
        Flush a stored blob to an actual file.
        
        Args:
            key: Blob key
            path: Path to write to
            content: Optional content (if not provided, just updates path)
            
        Returns:
            True if successful, False if blob not found
        """
        statement = select(FileRecord).where(FileRecord.key == key)
        record = self.session.exec(statement).first()
        if not record:
            return False
        
        # Write content if provided
        if content is not None:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(content)
            record.file_size = len(content)
        
        # Update record with file path
        record.file_path = str(path)
        record.updated_at = datetime.utcnow()
        self.session.add(record)
        self.session.commit()
        
        return True
    
    def get_record(self, key: str) -> FileRecord | None:
        """
        Get a file record by key.
        
        Args:
            key: Idempotency key
            
        Returns:
            File record or None if not found
        """
        statement = select(FileRecord).where(FileRecord.key == key)
        return self.session.exec(statement).first()
    
    def get_records_by_pipeline(
        self,
        pipeline_source: str,
        pipeline_stage: str | None = None
    ) -> list[FileRecord]:
        """
        Get all file records from a specific pipeline.
        
        Args:
            pipeline_source: Pipeline name
            pipeline_stage: Optional stage filter
            
        Returns:
            List of file records
        """
        statement = select(FileRecord).where(FileRecord.pipeline_source == pipeline_source)
        
        if pipeline_stage:
            statement = statement.where(FileRecord.pipeline_stage == pipeline_stage)
        
        return list(self.session.exec(statement).all())
    
    def get_validation_errors(self) -> list[FileRecord]:
        """
        Get all file records with validation errors.
        
        Returns:
            List of file records with errors
        """
        statement = select(FileRecord).where(FileRecord.validated == False)
        return list(self.session.exec(statement).all())
    
    def cleanup_orphaned_records(self) -> int:
        """
        Remove records where the associated file no longer exists.
        
        Returns:
            Number of records cleaned up
        """
        statement = select(FileRecord).where(FileRecord.file_path.isnot(None))
        records = list(self.session.exec(statement).all())
        
        orphaned = []
        for record in records:
            if record.file_path and not Path(record.file_path).exists():
                orphaned.append(record)
        
        for record in orphaned:
            self.session.delete(record)
        
        self.session.commit()
        return len(orphaned)
    
    def get_file_stats(self) -> dict[str, Any]:
        """
        Get statistics about files in the database.
        
        Returns:
            Dictionary of statistics
        """
        # Count totals
        total_statement = select(func.count(FileRecord.id))
        total = self.session.exec(total_statement).one()
        
        with_files_statement = select(func.count(FileRecord.id)).where(FileRecord.file_path.isnot(None))
        with_files = self.session.exec(with_files_statement).one()
        
        blobs_statement = select(func.count(FileRecord.id)).where(FileRecord.file_path.is_(None))
        blobs = self.session.exec(blobs_statement).one()
        
        validated_statement = select(func.count(FileRecord.id)).where(FileRecord.validated == True)
        validated = self.session.exec(validated_statement).one()
        
        errors_statement = select(func.count(FileRecord.id)).where(FileRecord.validated == False)
        errors = self.session.exec(errors_statement).one()
        
        # Count by pipeline
        by_pipeline = {}
        pipeline_statement = select(FileRecord.pipeline_source, func.count(FileRecord.id)).group_by(FileRecord.pipeline_source)
        for pipeline, count in self.session.exec(pipeline_statement).all():
            if pipeline:
                by_pipeline[pipeline] = count
        
        # Calculate total size
        size_statement = select(func.sum(FileRecord.file_size))
        total_bytes = self.session.exec(size_statement).one() or 0
        
        return {
            "total_records": total,
            "records_with_files": with_files,
            "blob_records": blobs,
            "validated_records": validated,
            "error_records": errors,
            "records_by_pipeline": by_pipeline,
            "total_size_bytes": total_bytes,
            "total_size_mb": round(total_bytes / BYTES_TO_MB_DIVISOR, 2)
        }
    
    # Legacy compatibility methods for backward compatibility
    def seen(self, key: str, content_hash: str) -> bool:
        """
        Legacy method: Check if a content hash has been seen.
        
        Args:
            key: Idempotency key
            content_hash: Content hash to check
            
        Returns:
            True if hash matches existing record
        """
        statement = select(FileRecord).where(FileRecord.key == key)
        record = self.session.exec(statement).first()
        return bool(record and record.hash == content_hash)
    
    def record(self, key: str, content_hash: str) -> None:
        """
        Legacy method: Record a hash for a key.
        
        Args:
            key: Idempotency key
            content_hash: Content hash to record
        """
        statement = select(FileRecord).where(FileRecord.key == key)
        existing = self.session.exec(statement).first()
        
        if existing:
            existing.hash = content_hash
            existing.updated_at = datetime.utcnow()
            self.session.add(existing)
        else:
            new_record = FileRecord(
                key=key,
                hash=content_hash,
                created_at=datetime.utcnow(),
                updated_at=datetime.utcnow()
            )
            self.session.add(new_record)
        
        self.session.commit()
