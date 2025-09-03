"""Utility for safely finding git repository and project root directories."""

import os
import logging
from pathlib import Path
from typing import Any
from openai import OpenAI
import tiktoken
from jinja2 import Environment, FileSystemLoader
from git import Repo, InvalidGitRepositoryError, GitCommandError
from logging import Logger


def get_git_root_dir(start_path: str = '.', logger: Logger | None = None) -> Path:
    """
    Safely get the git repository root directory with fallback strategies.
    
    Args:
        start_path: Starting path to search from (defaults to current directory)
        logger: Optional logger instance for debug/info messages
        
    Returns:
        Path object pointing to the project root directory
        
    Raises:
        FileNotFoundError: If no suitable root directory can be found
        PermissionError: If root directory exists but is not accessible
    """
    
    # Strategy 1: Try git repository detection
    try:
        repo = Repo(start_path, search_parent_directories=True)
        
        if repo.working_tree_dir is None:
            raise RuntimeError("Git repository has no working tree (bare repository?)")
            
        root_path = Path(repo.working_tree_dir).resolve()
        
        # Validate git root
        if not root_path.exists():
            raise FileNotFoundError(f"Git root directory does not exist: {root_path}")
            
        if not root_path.is_dir():
            raise RuntimeError(f"Git root path is not a directory: {root_path}")
            
        if not os.access(root_path, os.R_OK):
            raise PermissionError(f"No read access to git root directory: {root_path}")
            
        if logger:
            logger.info(f"Found git repository root: {root_path}")
        return root_path
        
    except (InvalidGitRepositoryError, GitCommandError, RuntimeError) as e:
        if logger:
            logger.debug(f"Git repository detection failed: {e}")
    
    # Strategy 2: Look for pyproject.toml
    project_root = _find_pyproject_root(start_path, logger)
    if project_root:
        return project_root
    
    # Strategy 3: Look for pyproject.toml starting from script's directory
    try:
        script_dir = Path(__file__).parent.absolute()
        project_root = _find_pyproject_root(str(script_dir), logger)
        if project_root:
            return project_root
    except NameError:
        # __file__ not available (e.g., in REPL)
        if logger:
            logger.debug("__file__ not available, skipping script directory search")
    
    # Strategy 4: Current working directory
    cwd = Path.cwd().resolve()
    if cwd.exists() and cwd.is_dir() and os.access(cwd, os.R_OK):
        if logger:
            logger.info(f"Using current working directory: {cwd}")
        return cwd
    
    # Strategy 5: Script's directory (if available)
    try:
        script_dir = Path(__file__).parent.absolute().resolve()
        if script_dir.exists() and script_dir.is_dir() and os.access(script_dir, os.R_OK):
            if logger:
                logger.info(f"Using script directory: {script_dir}")
            return script_dir
    except NameError:
        pass
    
    # If all else fails
    raise FileNotFoundError(
        f"Could not find suitable project root directory starting from: {start_path}"
    )


def _find_pyproject_root(start_path: str, logger: Logger | None = None) -> Path | None:
    """
    Find project root by looking for pyproject.toml.
    Searches backwards from start_path looking for pyproject.toml.
    
    Args:
        start_path: Starting path to search from
        logger: Optional logger instance
        
    Returns:
        Path to project root if found, None otherwise
    """
    
    try:
        current_path = Path(start_path).resolve()
    except (OSError, RuntimeError) as e:
        if logger:
            logger.debug(f"Could not resolve start_path {start_path}: {e}")
        return None
    
    # Search backwards through parent directories
    for path in [current_path] + list(current_path.parents):
        try:
            pyproject_path = path / 'pyproject.toml'
            if pyproject_path.exists() and pyproject_path.is_file():
                # Verify we have read access to the directory
                if os.access(path, os.R_OK):
                    if logger:
                        logger.info(f"Found project root using pyproject.toml: {path}")
                    return path
                else:
                    if logger:
                        logger.debug(f"Found pyproject.toml but no read access: {path}")
        except (OSError, PermissionError):
            # Skip directories we can't access
            continue
                
    if logger:
        logger.debug(f"No pyproject.toml found starting from: {start_path}")
    return None


def generate_with_openai(
    template_path: Path,
    template_context: dict[str, Any],
    uploaded_files: list[Path] = None,
    model: str = "gpt-5.1",
    temperature: float = 0.1,
    max_output_tokens: int = 4000,
    logger: logging.Logger = None,
    response_schema: dict[str, Any] | None = None,
) -> str:
    """
    Generic OpenAI generation utility using Jinja2 templates and file uploads.
    
    Args:
        template_path: Path to Jinja2 template file
        template_context: Context dictionary for template rendering
        uploaded_files: Optional list of file paths to upload to OpenAI
        model: OpenAI model to use
        temperature: Temperature setting for generation
        max_tokens: Maximum tokens for response
        logger: Optional logger instance
        
    Returns:
        Generated content from OpenAI
    """
    client = OpenAI()
    
    # Load and render Jinja2 template
    jinja_env = Environment(loader=FileSystemLoader(template_path.parent))
    template = jinja_env.get_template(template_path.name)
    system_prompt = template.render(**template_context)
    
    # Allow model override via environment variable
    env_model = os.getenv("DL_OPENAI_MODEL") or os.getenv("OPENAI_MODEL")
    if env_model:
        model = env_model

    uploaded_file_ids = []
    
    try:
        # Upload files if provided (Responses API expects input_file parts with file_id)
        user_content_parts: list[dict[str, Any]] = [
            {
                "type": "input_text",
                "text": "Generate comprehensive Python models based on the provided context and files."
            }
        ]

        # Token budget for input text parts (rough heuristic)
        # Reserve 25% of max_output_tokens for output; use up to 3x that for input context
        try:
            encoding = tiktoken.get_encoding("o200k_base")
        except Exception:
            encoding = tiktoken.get_encoding("cl100k_base")

        def count_tokens(s: str) -> int:
            try:
                return len(encoding.encode(s))
            except Exception:
                return len(s) // 3

        input_token_budget = max(8000, max_output_tokens * 3)
        used_tokens = count_tokens(system_prompt)

        if uploaded_files:
            for file_path in uploaded_files:
                if not file_path.exists():
                    raise FileNotFoundError(f"Uploaded file not found: {file_path}")

                suffix = file_path.suffix.lower()
                if suffix == ".pdf":
                    # PDFs: upload and reference as input_file
                    with open(file_path, 'rb') as f:
                        file_response = client.files.create(file=f, purpose="user_data")
                    uploaded_file_ids.append(file_response.id)
                    user_content_parts.append({
                        "type": "input_text",
                        "text": f"Attached PDF file: {file_path.name}"
                    })
                    user_content_parts.append({
                        "type": "input_file",
                        "file_id": file_response.id
                    })
                else:
                    # Non-PDFs: inline as input_text with truncation
                    try:
                        file_text = file_path.read_text(encoding='utf-8', errors='ignore')
                    except Exception:
                        with open(file_path, 'rb') as f:
                            raw_bytes = f.read()
                        file_text = raw_bytes.decode('utf-8', errors='ignore')

                    header = f"Filename: {file_path.name}\n---\n"
                    remaining_tokens = max(0, input_token_budget - used_tokens)
                    if remaining_tokens <= 0:
                        break

                    # Trim text approximately to remaining tokens
                    approx_chars = remaining_tokens * 3
                    trimmed_text = (file_text[:approx_chars]) if len(file_text) > approx_chars else file_text
                    part_text = header + trimmed_text
                    part_tokens = count_tokens(part_text)
                    if used_tokens + part_tokens > input_token_budget:
                        # Try to trim proportionally
                        allowed = max(0, input_token_budget - used_tokens)
                        if allowed <= 0:
                            break
                        # crude trim by characters
                        trimmed_text = trimmed_text[: allowed * 3]
                        part_text = header + trimmed_text
                        part_tokens = count_tokens(part_text)
                        if part_tokens == 0 or used_tokens + part_tokens > input_token_budget:
                            break

                    user_content_parts.append({
                        "type": "input_text",
                        "text": part_text
                    })
                    used_tokens += part_tokens

        # Token estimation (best-effort) for prompt; select encoding suitable for GPT-5/4o families
        prompt_token_count = count_tokens(system_prompt)
        if logger:
            logger.debug(f"Estimated prompt tokens: {prompt_token_count}")

        # Prepare additional guidance for structured outputs when schema provided
        instructions = system_prompt
        if response_schema is not None:
            instructions = (
                f"{system_prompt}\n\nYou must return output that strictly conforms to the following JSON Schema. "
                f"Do not include any prose or explanations. Only emit a single JSON object. Schema name: "
                f"{response_schema.get('name', 'structured_output')}\nSchema: {response_schema['schema']}"
            )

        # Make OpenAI Responses request
        input_messages = [
            {
                "type": "message",
                "role": "system",
                "content": [
                    {"type": "input_text", "text": instructions}
                ],
            },
            {
                "type": "message",
                "role": "user",
                "content": user_content_parts,
            },
        ]

        request_kwargs: dict[str, Any] = {
            "model": model,
            "input": input_messages,
            "max_output_tokens": max_output_tokens,
        }
        # Some latest models may not support temperature; include only when supported
        if temperature is not None and not str(model).startswith("gpt-5"):
            request_kwargs["temperature"] = temperature

        response = client.responses.create(**request_kwargs)

        # Prefer simplified accessor when available
        if hasattr(response, "output_text") and response.output_text:
            return response.output_text

        # Fallback extraction
        try:
            first_block = response.output[0]
            first_content = first_block.content[0]
            if first_content.type == "output_text":
                return first_content.text
        except Exception:
            pass

        # As last resort, return stringified response
        return str(response)
        
    finally:
        # Clean up uploaded files
        for file_id in uploaded_file_ids:
            try:
                client.files.delete(file_id)
            except Exception:
                pass


def render_template_to_file(
    template_path: Path, 
    output_path: Path,
    context: dict[str, Any],
    logger: logging.Logger = None
) -> None:
    """
    Render a Jinja2 template to a file.
    
    Args:
        template_path: Path to template file
        output_path: Path to output file
        context: Template context dictionary
        logger: Optional logger instance
    """
    jinja_env = Environment(loader=FileSystemLoader(template_path.parent))
    template = jinja_env.get_template(template_path.name)
    
    rendered_content = template.render(**context)
    
    output_path.parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write(rendered_content)
        
    if logger:
        logger.info(f"Rendered template {template_path.name} to {output_path}")
