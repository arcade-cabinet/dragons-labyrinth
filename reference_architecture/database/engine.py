"""
Database engine and session management for Professor Pixel's Arcade Academy.

Handles SQLAlchemy engine creation, session management, and database lifecycle.
"""

from __future__ import annotations

from contextlib import contextmanager
from typing import Generator

from sqlalchemy import create_engine, Engine
from sqlalchemy.orm import sessionmaker, Session

from professor_pixel.settings import get_settings
from professor_pixel.database.models import DatabaseBase

# Global engine and session factory (created on first access)
_engine: Engine | None = None
_SessionFactory: sessionmaker[Session] | None = None


def get_database_engine() -> Engine:
    """Get or create the database engine."""
    global _engine
    
    if _engine is None:
        settings = get_settings()
        settings.paths.ensure_directories()
        
        # Create SQLite engine with optimized settings
        _engine = create_engine(
            settings.paths.main_database_url,
            echo=False,  # Set to True for SQL debugging
            connect_args={
                "check_same_thread": False,  # Allow multi-thread access for SQLite
                "timeout": 20,  # Connection timeout
            },
            pool_pre_ping=True,  # Verify connections before use
            pool_recycle=3600,   # Recycle connections after 1 hour
        )
    
    return _engine


def get_session_factory() -> sessionmaker[Session]:
    """Get or create the session factory."""
    global _SessionFactory
    
    if _SessionFactory is None:
        engine = get_database_engine()
        _SessionFactory = sessionmaker(
            bind=engine,
            autocommit=False,
            autoflush=False,  # Manual control over flushing
            expire_on_commit=True,
        )
    
    return _SessionFactory


def get_database_session() -> Session:
    """Create a new database session."""
    factory = get_session_factory()
    return factory()


@contextmanager
def database_session() -> Generator[Session, None, None]:
    """Context manager for database sessions with automatic cleanup."""
    session = get_database_session()
    try:
        yield session
        session.commit()
    except Exception:
        session.rollback()
        raise
    finally:
        session.close()


def create_all_tables() -> None:
    """Create all database tables."""
    engine = get_database_engine()
    DatabaseBase.metadata.create_all(bind=engine)


def drop_all_tables() -> None:
    """Drop all database tables (use with caution!)."""
    engine = get_database_engine()
    DatabaseBase.metadata.drop_all(bind=engine)


def reset_database() -> None:
    """Reset the database by dropping and recreating all tables."""
    drop_all_tables()
    create_all_tables()


def close_database_connections() -> None:
    """Close all database connections and reset globals."""
    global _engine, _SessionFactory
    
    if _SessionFactory:
        _SessionFactory.close_all()
        _SessionFactory = None
    
    if _engine:
        _engine.dispose()
        _engine = None
