"""
Unified Statistics Model

Standardized statistics tracking for all subpackages using SQLModel.
"""

from datetime import datetime
from typing import Any

from sqlmodel import SQLModel, Field


class RunStatistics(SQLModel):
    """Standardized statistics for subpackage runs."""
    
    # Basic info
    subpackage: str
    start_time: datetime = Field(default_factory=datetime.now)
    end_time: datetime | None = None
    
    # Core metrics
    items_processed: int = 0
    items_stored: int = 0
    items_failed: int = 0
    
    # Flexible metrics (each subpackage can add custom fields)
    metrics: dict[str, Any] = Field(default_factory=dict)
    
    # Error tracking
    errors: list[str] = Field(default_factory=list)
    warnings: list[str] = Field(default_factory=list)
    
    def finish(self):
        """Mark the run as complete."""
        self.end_time = datetime.now()
    
    @property
    def duration(self) -> float:
        """Get duration in seconds."""
        if not self.end_time:
            return (datetime.now() - self.start_time).total_seconds()
        return (self.end_time - self.start_time).total_seconds()
    
    @property
    def success_rate(self) -> float:
        """Calculate success rate."""
        if self.items_processed == 0:
            return 0.0
        return self.items_stored / self.items_processed
    
    def add_metric(self, key: str, value: Any):
        """Add a custom metric."""
        self.metrics[key] = value
    
    def add_error(self, error: str):
        """Add an error."""
        self.errors.append(error)
        self.items_failed += 1
    
    def add_warning(self, warning: str):
        """Add a warning."""
        self.warnings.append(warning)
    
    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary for serialization."""
        return {
            "subpackage": self.subpackage,
            "start_time": self.start_time.isoformat(),
            "end_time": self.end_time.isoformat() if self.end_time else None,
            "duration": self.duration,
            "items_processed": self.items_processed,
            "items_stored": self.items_stored,
            "items_failed": self.items_failed,
            "success_rate": self.success_rate,
            "metrics": self.metrics,
            "errors": self.errors,
            "warnings": self.warnings
        }
