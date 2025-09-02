"""
SQLModel models for file management in Dragon's Labyrinth.

Defines SQLModel classes for idempotency tracking and file management.
Extracted from root models.py as part of subpackage reorganization.
"""

from __future__ import annotations

import re
from datetime import datetime
from typing import Any

from pydantic import validator, root_validator, Field as PydanticField
from sqlmodel import SQLModel, Field

from dragons_labyrinth.db.types import (
    # File types and paths
    FileKey, FilePath, ContentHash,
    # Enums
    PipelineStage,
)


class FileRecord(SQLModel, table=True):
    """Files table: Idempotency tracking replacing old idempotency.py"""
    __tablename__ = "files"
    
    # Primary fields with type-safe constraints
    key: FileKey = Field(primary_key=True, description="Unique idempotency key")
    hash: ContentHash = Field(description="SHA256 hash of content")
    created_at: datetime = Field(default_factory=datetime.now, description="Creation timestamp")
    updated_at: datetime = Field(default_factory=datetime.now, description="Last update timestamp")
    
    # File metadata with validation
    file_path: FilePath | None = Field(default=None, description="Associated file path")
    file_size: int | None = Field(default=None, ge=0, description="File size in bytes")
    content_type: str | None = Field(default=None, description="MIME type or content classification")
    
    # Pipeline context with enum validation
    pipeline_source: str | None = Field(default=None, description="Pipeline that created this record")
    pipeline_stage: PipelineStage | None = Field(default=None, description="Processing stage identifier")
    
    # Validation and status
    validated: bool = Field(default=True, description="Whether content passed validation")
    error_details: str | None = Field(default=None, description="Error details if validation failed")
    
    @validator("hash")
    def validate_content_hash(cls, v: str) -> str:
        """Validate SHA256 hash format"""
        if not re.match(r"^[a-f0-9]{64}$", v.lower()):
            raise ValueError("Hash must be a valid SHA256 hex string (64 characters)")
        return v.lower()
    
    @validator("file_path")
    def validate_file_path(cls, v: str | None) -> str | None:
        """Validate file path format if provided"""
        if v is None:
            return v
        if not isinstance(v, str) or len(v.strip()) == 0:
            raise ValueError("File path must be a non-empty string")
        # Basic path validation - no absolute paths, no parent directory traversal
        if v.startswith("/") or ".." in v:
            raise ValueError("File path must be relative and safe")
        return v.strip()
    
    @validator("pipeline_stage", pre=True)
    def validate_pipeline_stage(cls, v: str | PipelineStage | None) -> PipelineStage | None:
        """Convert string to PipelineStage enum"""
        if v is None:
            return v
        if isinstance(v, PipelineStage):
            return v
        # Try to match string to enum
        return PipelineStage.from_string(v)
