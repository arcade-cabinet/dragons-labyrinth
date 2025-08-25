"""
Type definitions and aliases for Dragon's Labyrinth HBF analysis
"""

from typing import TypeAlias
from pathlib import Path

import pandas as pd
from sqlalchemy.engine import Engine
import networkx as nx

# Path types
FilePath: TypeAlias = Path | str

# DataFrame types
EntitiesDataFrame: TypeAlias = pd.DataFrame
ReferencesDataFrame: TypeAlias = pd.DataFrame
ClusterDataFrame: TypeAlias = pd.DataFrame

# Database types
SQLiteEngine: TypeAlias = Engine

# Graph types
EntityGraph: TypeAlias = nx.DiGraph

# ID types
EntityID: TypeAlias = str
ClusterID: TypeAlias = str
ReferenceType: TypeAlias = str

# Data types
EntityDict: TypeAlias = dict[str, any]
SummaryDict: TypeAlias = dict[str, int | dict[str, int]]
MetricsDict: TypeAlias = dict[str, float | int | None]

# Token types
TokenCount: TypeAlias = int
