from __future__ import annotations

from pathlib import Path
from pydantic import BaseModel, Field
from pydantic_settings import BaseSettings, SettingsConfigDict


class CLIPathsSettings(BaseSettings):
    model_config = SettingsConfigDict(env_prefix="DL_", extra="ignore")

    templates_dir: Path = Field(default=Path("templates"))
    prompts_dir: Path = Field(default=Path("prompts"))
    specs_dir: Path = Field(default=Path("specs"))
    engine_base_dir: Path = Field(default=Path("crates/game-engine"))
    engine_assets_dir: Path = Field(default=Path("assets"))

    def assets_output_dir(self, category: str) -> Path:
        return self.engine_base_dir / self.engine_assets_dir / category


