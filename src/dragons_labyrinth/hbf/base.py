"""
Base mixin classes for HBF operations
Provides common property accessors for SQLite and DataFrame operations
"""

from dragons_labyrinth.models import OrchestratorState


class SQLiteMixin:
    """Base mixin providing SQLite-related property accessors"""
    
    def __init__(self, state: OrchestratorState):
        """Initialize with shared orchestrator state"""
        self.state = state
    
    @property
    def sqlite_conn(self):
        """Raw SQLite connection"""
        return self.state.sqlite_conn
    
    @property
    def sqlalchemy_engine(self):
        """SQLAlchemy engine"""
        return self.state.sqlalchemy_engine
    
    @property
    def log(self):
        """Logger instance"""
        return self.state.log
    
    @property
    def console(self):
        """Rich console instance"""
        return self.state.console
    
    @property
    def config(self):
        """HBF configuration"""
        return self.state.config


class DataFrameMixin:
    """Base mixin providing DataFrame-related property accessors"""
    
    def __init__(self, state: OrchestratorState):
        """Initialize with shared orchestrator state"""
        self.state = state
    
    @property
    def entities_df(self):
        """Entities DataFrame"""
        return self.state.entities_df
    
    @property
    def refs_df(self):
        """References DataFrame"""
        return self.state.refs_df
    
    @property
    def log(self):
        """Logger instance"""
        return self.state.log
    
    @property
    def console(self):
        """Rich console instance"""
        return self.state.console
    
    @property
    def config(self):
        """HBF configuration"""
        return self.state.config
