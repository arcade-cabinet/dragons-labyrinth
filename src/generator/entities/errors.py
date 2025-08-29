"""
Custom exceptions for entity extraction - Fail fast, fail loud.

No silent failures. No useless defaults. When shit breaks, we need to know.
"""


class EntityExtractionError(Exception):
    """Base exception for all entity extraction errors."""
    pass


class MLProcessorError(EntityExtractionError):
    """ML processor failed to initialize or process."""
    pass


class PatternMatchError(EntityExtractionError):
    """Pattern matching failed catastrophically."""
    pass


class ContentRoutingError(EntityExtractionError):
    """Content router couldn't determine entity type."""
    pass


class TrainingSystemError(EntityExtractionError):
    """Training system failed to discover patterns."""
    pass


class InvalidEntityError(EntityExtractionError):
    """Entity content is invalid or corrupted."""
    pass


class MissingDependencyError(EntityExtractionError):
    """Required dependency not available."""
    pass


class DatabaseConnectionError(EntityExtractionError):
    """Database connection or operation failed."""
    pass


class ExtractionStatisticsError(EntityExtractionError):
    """Failed to compute or store extraction statistics."""
    pass


class RegionDiscoveryError(EntityExtractionError):
    """Failed to auto-discover regions from hex tiles."""
    pass


class ClusteringError(EntityExtractionError):
    """ML clustering failed."""
    pass


class VectorizationError(EntityExtractionError):
    """Text vectorization failed."""
    pass


class EmbeddingError(EntityExtractionError):
    """Embedding generation or similarity computation failed."""
    pass


class CSVGenerationError(EntityExtractionError):
    """Failed to generate analysis CSV."""
    pass


class MemoryUpdateError(EntityExtractionError):
    """Failed to update entity memory for continuous learning."""
    pass


def raise_extraction_error(error_type: str, details: str, original_error: Exception = None):
    """
    Raise appropriate extraction error with full context.
    
    Args:
        error_type: Type of error (ml, pattern, routing, etc.)
        details: Detailed error message
        original_error: Original exception if any
    """
    
    error_map = {
        "ml": MLProcessorError,
        "pattern": PatternMatchError,
        "routing": ContentRoutingError,
        "training": TrainingSystemError,
        "invalid": InvalidEntityError,
        "dependency": MissingDependencyError,
        "database": DatabaseConnectionError,
        "stats": ExtractionStatisticsError,
        "region": RegionDiscoveryError,
        "clustering": ClusteringError,
        "vectorization": VectorizationError,
        "embedding": EmbeddingError,
        "csv": CSVGenerationError,
        "memory": MemoryUpdateError
    }
    
    error_class = error_map.get(error_type, EntityExtractionError)
    
    if original_error:
        raise error_class(f"{details}\nOriginal error: {str(original_error)}") from original_error
    else:
        raise error_class(details)
