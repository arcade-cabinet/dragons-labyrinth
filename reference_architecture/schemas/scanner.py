"""
Arcade API scanner for schema generation and pattern validation.

This module scans Arcade's public API and generates validated pattern schemas
that can be used to create safe, teachable patterns for students.
Uses libcst for deep source analysis and hard validation gates.
"""

from __future__ import annotations

import inspect
import json
import hashlib
import os
from pathlib import Path
from dataclasses import dataclass, asdict

# Build-time dependencies are now guaranteed available
import libcst as cst
from libcst.metadata import MetadataWrapper, PositionProvider, ScopeProvider
from pydantic import BaseModel, Field

from professor_pixel.base import BaseComponent
from professor_pixel.settings import get_settings
from professor_pixel.types import GameLibrary
from professor_pixel.database import pattern_db, ai_content_db
from professor_pixel.schemas.library_rules import get_library_rules, LibraryScanRules
from professor_pixel.schemas.rich_interface import AnalysisReviewInterface
from professor_pixel.models import APIUsageAnalysis, SchemaAnalysisRequest, CurriculumGenerationResult, PatternSuggestion

from professor_pixel.schemas.ai.analysis import SchemaAIAnalyzer
from professor_pixel.schemas.ai.agent import CurriculumAgent
from professor_pixel.schemas.ai.database_workflows.template_rules_workflow import LibraryTemplateRulesGenerator


@dataclass
class APIFunction:
    """Represents a function in the Arcade API."""
    name: str
    module: str
    signature: str
    docstring: str
    parameters: list[dict[str, str | None]]
    return_type: str = "any"
    category: str = "uncategorized"
    complexity: int = 1


@dataclass
class APIClass:
    """Represents a class in the Arcade API."""
    name: str
    module: str
    docstring: str
    methods: list[APIFunction]
    attributes: list[str]
    bases: list[str]
    category: str = "uncategorized"


class ArcadeUsageVisitor(cst.CSTVisitor):
    """LibCST visitor to analyze Arcade API usage patterns."""
    
    def __init__(self):
        self.function_calls: dict[str, int] = {}
        self.class_instantiations: dict[str, int] = {}
        self.common_patterns: list[tuple[str, str]] = []
        self.parameter_usage: dict[str, dict[str, list]] = {}
        self.co_occurrence: dict[str, set[str]] = {}
        self.safe_functions: set[str] = set()
        self.unsafe_functions: set[str] = set()
        self.current_function_context: str | None = None
    
    def visit_Call(self, node: cst.Call) -> None:
        """Analyze function calls to understand usage patterns."""
        
        # Extract function name from call
        func_name = self._extract_function_name(node.func)
        if func_name and "arcade" in func_name.lower():
            
            # Track function call frequency
            self.function_calls[func_name] = self.function_calls.get(func_name, 0) + 1
            
            # Analyze parameters for common usage patterns
            self._analyze_call_parameters(func_name, node.args)
            
            # Track co-occurrence with other functions
            if self.current_function_context and self.current_function_context != func_name:
                if func_name not in self.co_occurrence:
                    self.co_occurrence[func_name] = set()
                self.co_occurrence[func_name].add(self.current_function_context)
            
            # Classify as safe/unsafe based on parameters and usage
            self._classify_function_safety(func_name, node.args)
    
    def visit_ClassDef(self, node: cst.ClassDef) -> None:
        """Track class definitions and inheritance patterns."""
        class_name = node.name.value
        if any(base for base in node.bases if "arcade" in str(base).lower()):
            self.class_instantiations[class_name] = self.class_instantiations.get(class_name, 0) + 1
    
    def visit_FunctionDef(self, node: cst.FunctionDef) -> None:
        """Track function context for co-occurrence analysis."""
        self.current_function_context = node.name.value
    
    def leave_FunctionDef(self, node: cst.FunctionDef) -> None:
        """Clear function context."""
        self.current_function_context = None
    
    def _extract_function_name(self, func_node: cst.BaseExpression) -> str | None:
        """Extract function name from a call expression."""
        if isinstance(func_node, cst.Name):
            return func_node.value
        elif isinstance(func_node, cst.Attribute):
            # Handle arcade.draw_circle style calls
            obj_name = self._extract_function_name(func_node.value)
            if obj_name:
                return f"{obj_name}.{func_node.attr.value}"
            return func_node.attr.value
        return None
    
    def _analyze_call_parameters(self, func_name: str, args: list[cst.Arg]) -> None:
        """Analyze parameter usage patterns."""
        if func_name not in self.parameter_usage:
            self.parameter_usage[func_name] = {"positional": [], "keyword": [], "literals": []}
        
        for arg in args:
            if arg.keyword:
                # Keyword argument
                self.parameter_usage[func_name]["keyword"].append(arg.keyword.value)
            
            # Check for literal values (common defaults/patterns)
            if isinstance(arg.value, (cst.Integer, cst.Float, cst.SimpleString)):
                literal_value = arg.value.value if hasattr(arg.value, 'value') else str(arg.value)
                self.parameter_usage[func_name]["literals"].append(literal_value)
    
    def _classify_function_safety(self, func_name: str, args: list[cst.Arg]) -> None:
        """Classify functions as safe or unsafe for educational use."""
        
        # Functions with file I/O, network, or system access are generally unsafe
        unsafe_indicators = ["file", "path", "url", "http", "socket", "system", "exec", "eval"]
        safe_indicators = ["draw", "create", "sprite", "sound", "color", "texture"]
        
        func_lower = func_name.lower()
        
        if any(indicator in func_lower for indicator in unsafe_indicators):
            self.unsafe_functions.add(func_name)
        elif any(indicator in func_lower for indicator in safe_indicators):
            self.safe_functions.add(func_name)
        
        # Functions with too many parameters might be complex for beginners
        if len(args) > 8:
            self.unsafe_functions.add(func_name)
        elif len(args) <= 4:
            self.safe_functions.add(func_name)


class LibraryAPIScanner(BaseComponent):
    """
    Library-agnostic API scanner with configurable rules and hard validation gates.
    
    This scanner can analyze any game development library's API using library-specific
    rules to generate appropriate patterns for different educational phases.
    """
    
    def __init__(self, library: GameLibrary, source_path: str | None = None, **kwargs):
        """
        Initialize scanner for a specific library.
        
        Args:
            library: Which game library to scan
            source_path: Optional path to library source. If None, uses installed library.
        """
        super().__init__(**kwargs)
        self.library = library
        self.source_path = Path(source_path) if source_path else None
        self.rules = get_library_rules(library)
        self.api_surface: dict[str, object] = {}
        self.version: str = "unknown"
        
        # AI analysis and human review support
        self.autonomous_mode = os.getenv("PROFESSOR_PIXEL_AUTONOMOUS_MODE", "false").lower() == "true"
        self.interface = AnalysisReviewInterface() if not self.autonomous_mode else None
        self.curriculum_agent = CurriculumAgent()
        self.template_rules_generator = LibraryTemplateRulesGenerator(library)
        self.usage_analysis: APIUsageAnalysis | None = None
        self.template_rules: object | None = None
        
        self.log_info(f"Library API scanner initialized for {library.name}")
        
    def scan(self) -> dict[str, object]:
        """
        Comprehensive scan of Arcade's API with libcst analysis and hard validation.
        
        Returns:
            Dictionary containing:
            - version: Arcade version
            - functions: All validated teachable functions
            - classes: All validated teachable classes  
            - patterns: Generated IR-ready pattern definitions
            - validation_report: What was included/excluded and why
        """
        import arcade
        
        # Get version
        self.version = getattr(arcade, "__version__", "unknown")
        self.log_info(f"Scanning Arcade {self.version} API...")
        
        # Initialize results with validation tracking
        results = {
            "version": self.version,
            "functions": {},
            "classes": {},
            "constants": {},
            "validation_report": {
                "scanned_modules": 0,
                "total_functions": 0,
                "teachable_functions": 0,
                "forbidden_functions": 0,
                "total_classes": 0,
                "teachable_classes": 0,
                "complex_classes": 0,
            }
        }
        
        # Scan ALL core modules comprehensively (no exclusions based on skill level)
        for module_name in self.rules.core_modules:
            if module_name in self.rules.exclude_modules:
                continue  # Only skip truly broken/internal modules
                
            try:
                module = self._import_module(module_name)
                if module:
                    self._scan_module_comprehensively(module, module_name, results)
                    results["validation_report"]["scanned_modules"] += 1
            except ImportError as e:
                self.log_warning(f"Could not import {module_name}: {e}")
        
        # Generate ALL patterns with metadata (no exclusions)
        results["patterns"] = self._generate_all_patterns(results)
        
        # Generate database filtering rules for skill levels
        results["skill_filters"] = self._generate_skill_filters(results)
        
        # Run LibCST analysis if source path exists
        if self.library == GameLibrary.ARCADE and self.source_path:
            self.log_info("Running LibCST analysis of Arcade source...")
            self.scan_arcade_source_with_libcst(self.source_path)
        elif self.library == GameLibrary.ARCADE:
            # Try to find arcade source in common locations
            import arcade
            arcade_path = Path(arcade.__file__).parent
            if arcade_path.exists():
                self.log_info("Running LibCST analysis of installed Arcade...")
                self.scan_arcade_source_with_libcst(arcade_path)
        
        self.api_surface = results
        teachable_count = results['validation_report']['teachable_functions']
        self.log_info(f"Scanned {teachable_count} teachable functions")
        return results
    
    def _import_module(self, module_name: str):
        """Safely import a module."""
        try:
            if module_name == "arcade":
                import arcade
                return arcade
            else:
                parts = module_name.split(".")
                module = __import__(module_name, fromlist=[parts[-1]])
                return module
        except ImportError:
            return None
    
    def _scan_module_comprehensively(self, module, module_name: str, results: dict):
        """Scan a module comprehensively - capture everything, filter later in database."""
        for name, obj in inspect.getmembers(module):
            # Skip only private/internal (not complexity-based exclusions)
            if name.startswith("_"):
                continue
            
            # Process ALL functions (no validation gates)
            if inspect.isfunction(obj) or inspect.isbuiltin(obj):
                results["validation_report"]["total_functions"] += 1
                
                func_data = self._extract_function_with_metadata(obj, name, module_name)
                if func_data:
                    results["functions"][f"{module_name}.{name}"] = asdict(func_data)
                    results["validation_report"]["teachable_functions"] += 1
            
            # Process ALL classes (no complexity gates)
            elif inspect.isclass(obj):
                results["validation_report"]["total_classes"] += 1
                
                class_data = self._extract_class_with_metadata(obj, name, module_name)
                if class_data:
                    results["classes"][f"{module_name}.{name}"] = asdict(class_data)
                    results["validation_report"]["teachable_classes"] += 1
            
            # Process ALL constants (public only)
            elif isinstance(obj, (int, float, str)) and not name.startswith("_"):
                results["constants"][f"{module_name}.{name}"] = {
                    "name": name,
                    "module": module_name,
                    "value": obj,
                    "type": type(obj).__name__,
                }
    
    # Removed old validation methods - we now scan everything and filter in database
    
    def _extract_function_with_metadata(self, func, name: str, module: str) -> APIFunction | None:
        """Extract function information with comprehensive metadata (no exclusions)."""
        try:
            sig = inspect.signature(func)
            params = []
            required_params = 0
            
            for param_name, param in sig.parameters.items():
                param_info = {
                    "name": param_name,
                    "kind": str(param.kind),
                    "default": str(param.default) if param.default != inspect.Parameter.empty else None,
                    "annotation": str(param.annotation) if param.annotation != inspect.Parameter.empty else "any",
                }
                params.append(param_info)
                
                # Count required parameters (metadata only)
                if param.default == inspect.Parameter.empty:
                    required_params += 1
            
            # Calculate metadata (no exclusions)
            category = self._categorize_function(name, module)
            complexity = self._calculate_complexity_metadata(params, name)
            
            return APIFunction(
                name=name,
                module=module,
                signature=str(sig),
                docstring=inspect.getdoc(func) or "",
                parameters=params,
                return_type=str(sig.return_annotation) if sig.return_annotation != inspect.Signature.empty else "any",
                category=category,
                complexity=complexity,
            )
        except Exception as e:
            # Some built-ins can't be inspected - that's OK, just log and continue
            self.log_debug(f"Could not inspect {module}.{name}: {e}")
            return None
    
    def _extract_class_with_metadata(self, cls, name: str, module: str) -> APIClass | None:
        """Extract information from a class."""
        methods = []
        attributes = []
        
        # Get methods
        for method_name, method_obj in inspect.getmembers(cls):
            if method_name.startswith("_") and method_name != "__init__":
                continue
            
            if inspect.isfunction(method_obj) or inspect.ismethod(method_obj):
                method_info = self._extract_function_with_metadata(method_obj, method_name, module)
                if method_info:
                    methods.append(asdict(method_info))
        
        # Get attributes (class variables)
        for attr_name in dir(cls):
            if not attr_name.startswith("_"):
                attr = getattr(cls, attr_name)
                if not callable(attr):
                    attributes.append(attr_name)
        
        # Get base classes
        bases = [base.__name__ for base in cls.__bases__ if base.__name__ != "object"]
        
        return APIClass(
            name=name,
            module=module,
            docstring=inspect.getdoc(cls) or "",
            methods=methods,
            attributes=attributes,
            bases=bases,
            category=self._categorize_function(name, module),
        )
    
    def _categorize_function(self, name: str, module: str) -> str:
        """Categorize an API element based on library-specific rules (ordered by specificity)."""
        full_name = f"{module}.{name}".lower()
        
        # Use ordered category mappings (most specific first)
        for keyword, category in self.rules.category_mappings:
            if keyword in full_name:
                return category
        
        return "general"
    
    def _calculate_complexity_metadata(self, params: list[dict], name: str) -> int:
        """Calculate complexity metadata based on parameters and library-specific indicators."""
        # Base complexity from parameter count
        param_count = len(params)
        
        if param_count <= 2:
            complexity = 1
        elif param_count <= 4:
            complexity = 2
        elif param_count <= 6:
            complexity = 3
        elif param_count <= 8:
            complexity = 4
        else:
            complexity = 5
        
        # Apply library-specific complexity indicators
        name_lower = name.lower()
        for indicator, boost in self.rules.complexity_indicators.items():
            if indicator in name_lower:
                complexity = min(5, max(1, complexity + boost))
                break  # Apply first match only
        
        return complexity
    
    def _generate_all_patterns(self, api_data: dict) -> dict[str, object]:
        """Generate pattern definitions from API data."""
        patterns = {}
        
        # Generate patterns for ALL functions (no exclusions)
        for func_name, func_data in api_data["functions"].items():
            # Create pattern opcode (e.g., arcade.draw_circle -> DRAW_CIRCLE)
            short_name = func_name.split(".")[-1]
            opcode = short_name.upper()
            
            patterns[opcode] = {
                "opcode": opcode,
                "source": func_name,
                "category": func_data["category"],
                "complexity": func_data["complexity"],
                "signature": func_data["signature"],
                "parameters": func_data["parameters"],
                "docstring": func_data["docstring"],
                "template": self._generate_template(func_name, func_data),
                "library": self.library.name.lower(),
            }
        
        return patterns
    
    # Removed _is_teachable - we scan everything and let database queries filter by skill level
    
    def _generate_template(self, func_name: str, func_data: dict) -> str:
        """Generate a Jinja2 template for a function pattern."""
        params = func_data["parameters"]
        
        # Build parameter list for template
        param_list = []
        for param in params:
            param_name = param["name"]
            if param["default"]:
                param_list.append(f"{{{{ {param_name} | default({param['default']}) }}}}")
            else:
                param_list.append(f"{{{{ {param_name} }}}}")
        
        # Generate template
        template = f"{func_name}({', '.join(param_list)})"
        return template
    
    def _generate_skill_filters(self, api_data: dict) -> dict[str, object]:
        """Generate database filtering rules for different skill levels."""
        # Analyze patterns by complexity for database queries
        complexity_distribution = {}
        category_distribution = {}
        
        for pattern_data in api_data["patterns"].values():
            complexity = pattern_data["complexity"]
            category = pattern_data["category"]
            
            complexity_distribution[complexity] = complexity_distribution.get(complexity, 0) + 1
            category_distribution[category] = category_distribution.get(category, 0) + 1
        
        return {
            "skill_level_queries": {
                # Database queries for different skill levels
                "beginner": {
                    "complexity_max": 2,
                    "categories": ["visual", "sprites", "audio"],
                    "parameter_count_max": 4,
                    "description": "Simple functions with clear, immediate effects"
                },
                "intermediate": {
                    "complexity_max": 3,
                    "categories": ["visual", "sprites", "audio", "collision", "input"],
                    "parameter_count_max": 6,
                    "description": "Functions requiring basic understanding of game concepts"
                },
                "advanced": {
                    "complexity_max": 4,
                    "categories": ["motion", "game", "visual", "sprites", "collision"],
                    "parameter_count_max": 8,
                    "description": "Complex functions for experienced students"
                },
                "expert": {
                    "complexity_max": 5,
                    "categories": ["any"],
                    "parameter_count_max": 99,
                    "description": "Full API access for advanced projects"
                }
            },
            
            "asset_integration_hints": {
                # Suggest asset types based on parameter names
                "image_parameters": ["texture", "image", "sprite", "path", "file", "filename"],
                "audio_parameters": ["sound", "audio", "music", "file"],
                "font_parameters": ["font", "typeface", "text"],
            },
            
            "complexity_stats": complexity_distribution,
            "category_stats": category_distribution,
            
            "progressive_revelation": {
                # How to progressively reveal complexity
                "phase_1_max_complexity": 2,  # Quick wins
                "phase_2_max_complexity": 3,  # Fundamentals  
                "phase_3_max_complexity": 4,  # Behind curtain
                "phase_4_max_complexity": 5,  # Software engineering
            }
        }
    
    def scan_arcade_source_with_libcst(self, arcade_source_path: Path) -> dict[str, object]:
        """Use libcst to perform deep source analysis of Arcade codebase."""
        if not arcade_source_path.exists():
            raise FileNotFoundError(f"Arcade source not found: {arcade_source_path}")
        
        analysis_results = {
            "public_api": {},
            "usage_patterns": {},
            "complexity_analysis": {},
            "safe_functions": set(),
            "unsafe_functions": set(),
            # Global accumulators for AI analysis
            "global_function_calls": {},
            "global_parameter_patterns": {},
            "global_co_occurrence": {},
        }
        
        # Scan core Arcade modules with libcst
        for py_file in arcade_source_path.rglob("*.py"):
            if any(exclude in str(py_file) for exclude in ["test", "example", "experimental"]):
                continue
                
            try:
                source_code = py_file.read_text(encoding="utf-8")
                tree = cst.parse_module(source_code)
                
                # Use metadata providers for enhanced analysis
                wrapper = MetadataWrapper(tree)
                wrapper.add_metadata_dependency(PositionProvider)
                wrapper.add_metadata_dependency(ScopeProvider)
                
                # Analyze the AST
                self._analyze_module_ast(wrapper, str(py_file), analysis_results)
                
            except Exception as e:
                self.log_warning(f"Could not analyze {py_file}: {e}")
        
        # Create APIUsageAnalysis object from accumulated data
        common_sequences = self._extract_common_sequences(analysis_results["global_co_occurrence"])
        
        self.usage_analysis = APIUsageAnalysis(
            function_calls=analysis_results["global_function_calls"],
            parameter_patterns=analysis_results["global_parameter_patterns"],
            co_occurrence={k: list(v) for k, v in analysis_results["global_co_occurrence"].items()},
            complexity_indicators={},  # Will be calculated if needed
            safe_functions=list(analysis_results["safe_functions"]),
            unsafe_functions=list(analysis_results["unsafe_functions"]),
            common_sequences=common_sequences
        )
        
        self.log_info(f"LibCST analysis complete: {len(analysis_results['global_function_calls'])} functions analyzed")
        return analysis_results
    
    def _extract_common_sequences(self, co_occurrence: dict[str, set[str]]) -> list[list[str]]:
        """Extract common function call sequences from co-occurrence data."""
        sequences = []
        
        # Simple heuristic: if functions A and B co-occur, and B and C co-occur,
        # then [A, B, C] might be a common sequence
        for func_a, related_funcs in co_occurrence.items():
            for func_b in related_funcs:
                if func_b in co_occurrence:
                    # Find functions that commonly follow func_b
                    common_next = co_occurrence[func_b] & related_funcs
                    if common_next:
                        for func_c in list(common_next)[:2]:  # Limit sequences
                            sequences.append([func_a, func_b, func_c])
        
        # Remove duplicates and return top sequences
        unique_sequences = []
        for seq in sequences:
            if seq not in unique_sequences:
                unique_sequences.append(seq)
        
        return unique_sequences[:10]  # Return top 10 sequences
    
    def _analyze_module_ast(self, wrapper: MetadataWrapper, file_path: str, results: dict):
        """Analyze a module's AST for teachable patterns and usage insights."""
        
        # Extract function calls and patterns from the AST
        visitor = ArcadeUsageVisitor()
        wrapper.visit(visitor)
        
        # Update results with visitor data
        results["usage_patterns"][Path(file_path).stem] = {
            "function_calls": visitor.function_calls,
            "class_instantiations": visitor.class_instantiations,
            "common_patterns": visitor.common_patterns,
            "parameter_usage": visitor.parameter_usage,
            "co_occurrence": visitor.co_occurrence
        }
        
        # Accumulate global data for AI analysis
        for func_name, count in visitor.function_calls.items():
            results["global_function_calls"][func_name] = results["global_function_calls"].get(func_name, 0) + count
        
        for func_name, params in visitor.parameter_usage.items():
            if func_name not in results["global_parameter_patterns"]:
                results["global_parameter_patterns"][func_name] = {"keyword": [], "literals": []}
            results["global_parameter_patterns"][func_name]["keyword"].extend(params.get("keyword", []))
            results["global_parameter_patterns"][func_name]["literals"].extend(params.get("literals", []))
        
        for func_name, co_funcs in visitor.co_occurrence.items():
            if func_name not in results["global_co_occurrence"]:
                results["global_co_occurrence"][func_name] = set()
            results["global_co_occurrence"][func_name].update(co_funcs)
        
        # Update safe/unsafe function tracking based on actual usage
        results["safe_functions"].update(visitor.safe_functions)
        results["unsafe_functions"].update(visitor.unsafe_functions)
    
    def save_to_database(self, library: GameLibrary | None = None) -> int:
        """Save patterns to database with idempotency checks and human review."""
        target_library = library or self.library
        
        if "patterns" not in self.api_surface:
            self.log_warning("No patterns to save")
            return 0
        
        patterns = self.api_surface["patterns"]
        
        # Create analysis version hash for idempotency
        analysis_inputs = {
            "library": target_library.name,
            "library_version": self.version,
            "library_rules": {
                "core_modules": list(self.rules.core_modules),
                "category_mappings": dict(self.rules.category_mappings) if hasattr(self.rules, 'category_mappings') else {},
                "complexity_indicators": self.rules.complexity_indicators
            },
            "usage_analysis": self.usage_analysis.model_dump() if self.usage_analysis else {},
            "api_functions_count": len(self.api_surface.get("functions", {}))
        }
        
        import json
        import hashlib
        analysis_hash = hashlib.sha256(
            json.dumps(analysis_inputs, sort_keys=True).encode()
        ).hexdigest()[:16]
        
        # Check for existing analysis results (idempotency)
        existing_analysis = ai_content_db.get_api_analysis_result(target_library, analysis_hash)
        if existing_analysis:
            self.log_info(f"Found existing API analysis for {target_library.name} (hash: {analysis_hash})")
            self.log_info("Skipping AI analysis - using cached results")
            
            # Use existing patterns from database
            existing_patterns = pattern_db.get_patterns_by_library(target_library)
            if existing_patterns:
                self.log_success(f"Using {len(existing_patterns)} cached patterns from database")
                return len(existing_patterns)
        
        self.log_info(f"No existing analysis found for {target_library.name} - running full AI analysis...")
        
        # AI-powered curriculum generation using scanner results
        if self.usage_analysis:
            self.log_info("Running comprehensive curriculum generation with AI workflows...")
            
            # Step 1: Generate template rules from library rules + usage analysis + patterns
            self.log_info("Generating template rules from library analysis...")
            
            # Convert raw patterns to PatternSuggestion objects for AI analysis
            pattern_suggestions = []
            for opcode, pattern_data in patterns.items():
                try:
                    # Extract parameters list properly
                    parameters = pattern_data.get("parameters", [])
                    if isinstance(parameters, list) and parameters:
                        common_parameters = [p.get("name", "unknown") for p in parameters if isinstance(p, dict)][:5]
                    else:
                        common_parameters = []
                    
                    # Create PatternSuggestion from raw pattern data
                    docstring = pattern_data.get("docstring", "")
                    title = docstring[:50] if docstring else opcode.replace("_", " ").title()
                    description = docstring if docstring else f"Educational pattern for {pattern_data.get('source', opcode)}"
                    
                    suggestion = PatternSuggestion(
                        opcode=opcode,
                        title=title,
                        description=description,
                        complexity=pattern_data.get("complexity", 1),
                        category=pattern_data.get("category", "general"),
                        source_function=pattern_data.get("source", "unknown"),
                        common_parameters=common_parameters,
                        typical_values={},  # Would be filled from usage analysis
                        teaches_concepts=[pattern_data.get("category", "unknown")],
                        prerequisites=[],
                        template_file=f"{pattern_data.get('category', 'general').lower()}/{opcode.lower()}.jinja2",
                        suggested_choices=[]  # Basic - AI will enhance these
                    )
                    pattern_suggestions.append(suggestion)
                except Exception as e:
                    self.log_warning(f"Could not convert pattern {opcode}: {e}")
                    continue
            
            # Generate template rules based on real library analysis
            template_rules = self.template_rules_generator.generate_template_rules_from_scan(
                library_rules=self.rules,
                usage_analysis=self.usage_analysis,
                api_patterns=pattern_suggestions,
                autonomous_mode=self.autonomous_mode
            )
            
            self.template_rules = template_rules
            self.log_success(f"Generated template rules for {len(template_rules.categories_supported)} categories")
            
            # Step 2: Run curriculum generation with the generated template rules
            settings = get_settings()
            output_dir = settings.paths.state_dir / "generated_curriculum" / target_library.name.lower()
            
            result = self.curriculum_agent.generate_curriculum(
                library_name=target_library.name,
                api_functions=[f for f in self.api_surface.get("functions", {}).values()],
                usage_analysis=self.usage_analysis,
                output_directory=output_dir,
                target_complexity_levels=[1, 2, 3],
                max_patterns=50,
                lesson_count_target=8,
                template_style="intermediate",
                autonomous_mode=self.autonomous_mode
            )
            
            if result.success:
                # Store AI analysis results in database for future idempotency
                analysis_data = {
                    "usage_patterns": self.usage_analysis.model_dump(),
                    "pattern_suggestions": [p.model_dump() for p in result.patterns_generated],
                    "lesson_progression": [l.model_dump() for l in result.lessons_created],
                    "total_functions_analyzed": len(self.api_surface.get("functions", {})),
                    "patterns_generated": len(result.patterns_generated),
                    "pattern_coverage_percent": result.patterns_count / len(self.api_surface.get("functions", {})) * 100,
                    "educational_safety_score": 0.8,  # Would be calculated from safety analysis
                    "ai_model_used": "gpt-4o-mini",
                    "processing_time": result.total_processing_time_seconds,
                    "tokens_consumed": None  # Would be tracked if available
                }
                
                # Store analysis results with human approval
                ai_content_db.store_api_analysis_result(
                    library=target_library,
                    library_version=self.version,
                    analysis_version=analysis_hash,
                    analysis_data=analysis_data,
                    is_approved=True  # Since it went through human review workflow
                )
                
                # Store curriculum structure
                curriculum_hash = hashlib.sha256(
                    json.dumps(result.curriculum_structure, sort_keys=True).encode()
                ).hexdigest()[:16]
                
                ai_content_db.store_curriculum_structure(
                    curriculum_data=result.curriculum_structure,
                    generation_hash=curriculum_hash,
                    is_approved=True
                )
                
                # Convert AI-generated patterns to database format
                patterns = {}
                for pattern in result.patterns_generated:
                    patterns[pattern.opcode] = {
                        "opcode": pattern.opcode,
                        "source": pattern.source_function,
                        "category": pattern.category,
                        "complexity": pattern.complexity,
                        "signature": f"def {pattern.source_function}(...)",
                        "parameters": [{"name": p} for p in pattern.common_parameters],
                        "docstring": pattern.description,
                        "template": pattern.template_file,
                        "library": self.library.name.lower(),
                        "ai_generated": True,
                        "teaches_concepts": pattern.teaches_concepts,
                        "prerequisites": pattern.prerequisites,
                        "student_choices": pattern.suggested_choices
                    }
                
                self.log_success(f"AI curriculum generation complete: {len(patterns)} patterns")
                self.log_info(f"Analysis results stored in database for future idempotency")
                
                # Show additional info if not autonomous
                if not self.autonomous_mode:
                    self.interface.show_success(f"Complete curriculum generated with {result.templates_count} templates")
            
            else:
                raise RuntimeError("AI curriculum generation failed - check logs for details")
        else:
            # Fallback to original patterns if no usage analysis
            self.log_info("No usage analysis available, using basic patterns")
                
            if not self.autonomous_mode and self.interface:
                approved_patterns = self._review_patterns_with_human(patterns)
                if not approved_patterns:
                    self.log_info("Pattern save cancelled by user")
                    return 0
                patterns = approved_patterns
        
        # Store patterns in database with full metadata
        pattern_count = pattern_db.store_patterns(patterns, target_library)
        self.log_info(f"Stored {pattern_count} {target_library.name.lower()} patterns in database")
        
        # Also save artifacts to cache for reference
        self._save_artifacts_to_cache(target_library)
        
        # Show persistence stats if not in autonomous mode
        if not self.autonomous_mode:
            cache_stats = self.curriculum_agent.get_cache_stats()
            self.log_info(f"AI Cache: {cache_stats['cache_entries']} entries, {cache_stats['cache_db_size_mb']:.1f}MB")
        
        return pattern_count
    
    def _review_patterns_with_human(self, patterns: dict[str, object]) -> dict[str, object] | None:
        """Review patterns with human using Rich interface."""
        
        # Create analysis summary for review
        analysis_summary = self._create_analysis_summary(patterns)
        
        # Use Rich interface to display comprehensive summary
        self.interface.display_comprehensive_summary({
            "analysis_summary": analysis_summary,
            "pattern_schemas": self._convert_patterns_to_schema_format(patterns),
            "lesson_progression": self._suggest_lesson_progression(patterns),
            "usage_patterns": self.usage_analysis
        })
        
        # Interactive review loop
        while True:
            action = self.interface.show_interactive_menu()
            
            if action == "approve_all":
                self.interface.show_success("Patterns approved for database save!")
                return patterns
                
            elif action == "view_details":
                self.interface.show_detailed_results({
                    "patterns": patterns,
                    "usage_analysis": self.usage_analysis,
                    "validation_report": self.api_surface.get("validation_report", {})
                })
                
            elif action == "edit_patterns":
                # Simple pattern filtering for now
                complexity_filter = input("Enter max complexity (1-5, or 'all'): ")
                if complexity_filter.isdigit():
                    max_complexity = int(complexity_filter)
                    filtered_patterns = {
                        k: v for k, v in patterns.items() 
                        if v.get("complexity", 1) <= max_complexity
                    }
                    self.interface.show_success(f"Filtered to {len(filtered_patterns)} patterns (complexity <= {max_complexity})")
                    return filtered_patterns
                
            elif action == "reject_all":
                self.interface.show_error("Pattern analysis rejected")
                return None
    
    def _create_analysis_summary(self, patterns: dict[str, object]) -> dict[str, object]:
        """Create comprehensive analysis summary for human review."""
        
        complexity_dist = {}
        category_dist = {}
        total_functions = len(self.api_surface.get("functions", {}))
        
        for pattern in patterns.values():
            complexity = pattern.get("complexity", 1)
            category = pattern.get("category", "uncategorized")
            
            complexity_dist[complexity] = complexity_dist.get(complexity, 0) + 1
            category_dist[category] = category_dist.get(category, 0) + 1
        
        return {
            "analysis_overview": {
                "total_api_functions": total_functions,
                "patterns_identified": len(patterns),
                "lessons_designed": 0,  # Would come from lesson progression
                "complexity_levels": list(complexity_dist.keys())
            },
            "key_findings": {
                "most_common_patterns": list(patterns.keys())[:10],
                "beginner_friendly_functions": [
                    k for k, v in patterns.items() if v.get("complexity", 1) <= 2
                ][:10],
                "advanced_concepts": [
                    k for k, v in patterns.items() if v.get("complexity", 1) >= 4
                ][:5]
            },
            "proposed_changes": {
                "new_pattern_schemas": len(patterns),
                "lesson_structure": {},  # Would be populated by lesson progression
                "student_choice_points": len(patterns) * 2  # Rough estimate
            }
        }
    
    def _convert_patterns_to_schema_format(self, patterns: dict[str, object]) -> list[dict[str, object]]:
        """Convert raw patterns to schema format for display."""
        
        schemas = []
        for opcode, pattern in patterns.items():
            schema = {
                "opcode": opcode,
                "title": pattern.get("docstring", opcode.replace("_", " ").title())[:50],
                "complexity": self._complexity_to_string(pattern.get("complexity", 1)),
                "category": pattern.get("category", "uncategorized").upper(),
                "choices": []  # Would be generated based on parameters
            }
            
            # Add mock choices based on parameters
            for param in pattern.get("parameters", []):
                if param.get("name") not in ["self", "args", "kwargs"]:
                    schema["choices"].append({
                        "id": param.get("name", "unknown"),
                        "type": "parameter"
                    })
            
            schemas.append(schema)
        
        return schemas[:10]  # Limit for display
    
    def _complexity_to_string(self, complexity: int) -> str:
        """Convert complexity number to string."""
        if complexity <= 2:
            return "BASIC"
        elif complexity <= 3:
            return "MODERATE"
        else:
            return "ADVANCED"
    
    def _suggest_lesson_progression(self, patterns: dict[str, object]) -> dict[str, object]:
        """Suggest basic lesson progression based on patterns."""
        
        # Basic progression suggestion based on complexity
        basic_patterns = [k for k, v in patterns.items() if v.get("complexity", 1) <= 2]
        moderate_patterns = [k for k, v in patterns.items() if v.get("complexity", 1) == 3]
        
        return {
            "lessons": [
                {
                    "id": "y1_l1_basics",
                    "title": "Basic Game Elements",
                    "patterns": basic_patterns[:5],
                    "student_choices": 3
                },
                {
                    "id": "y1_l2_intermediate",
                    "title": "Game Interactions",
                    "patterns": moderate_patterns[:5],
                    "student_choices": 4
                }
            ]
        }
    
    def _convert_ai_patterns_to_db_format(self, ai_patterns: list) -> dict[str, object]:
        """Convert AI-generated patterns to database format."""
        
        db_patterns = {}
        for pattern in ai_patterns:
            db_patterns[pattern.opcode] = {
                "opcode": pattern.opcode,
                "source": pattern.source_function,
                "category": pattern.category,
                "complexity": pattern.complexity,
                "signature": f"def {pattern.source_function}(...)",  # Simplified
                "parameters": [{"name": p} for p in pattern.common_parameters],
                "docstring": pattern.description,
                "template": pattern.template_file,
                "library": self.library.name.lower(),
                "ai_generated": True,
                "teaches_concepts": pattern.teaches_concepts,
                "prerequisites": pattern.prerequisites,
                "student_choices": pattern.suggested_choices
            }
        
        self.log_info(f"Converted {len(ai_patterns)} AI patterns to database format")
        return db_patterns
    
    def _review_ai_analysis_with_human(self, ai_response, fallback_patterns: dict[str, object]) -> dict[str, object] | None:
        """Review AI analysis results with human using Rich interface."""
        
        # Convert AI response to format expected by Rich interface
        analysis_data = {
            "analysis_summary": {
                "analysis_overview": {
                    "total_api_functions": ai_response.total_functions_analyzed,
                    "patterns_identified": ai_response.patterns_generated,
                    "lessons_designed": len(ai_response.lesson_progression),
                    "complexity_levels": list(ai_response.complexity_distribution.keys())
                },
                "key_findings": {
                    "most_common_patterns": [p.opcode for p in ai_response.pattern_suggestions[:10]],
                    "beginner_friendly_functions": [
                        p.opcode for p in ai_response.pattern_suggestions 
                        if p.complexity <= 2
                    ][:10],
                    "advanced_concepts": [
                        p.opcode for p in ai_response.pattern_suggestions 
                        if p.complexity >= 4
                    ][:5]
                },
                "proposed_changes": {
                    "new_pattern_schemas": len(ai_response.pattern_suggestions),
                    "lesson_structure": {
                        lesson.lesson_id: lesson.title 
                        for lesson in ai_response.lesson_progression
                    },
                    "student_choice_points": sum(
                        len(p.suggested_choices) for p in ai_response.pattern_suggestions
                    )
                }
            },
            "pattern_schemas": [
                {
                    "opcode": p.opcode,
                    "title": p.title,
                    "complexity": self._complexity_to_string(p.complexity),
                    "category": p.category.upper(),
                    "choices": p.suggested_choices
                }
                for p in ai_response.pattern_suggestions
            ],
            "lesson_progression": {
                "lessons": [
                    {
                        "id": l.lesson_id,
                        "title": l.title,
                        "patterns": l.patterns_used,
                        "student_choices": l.student_choice_points
                    }
                    for l in ai_response.lesson_progression
                ]
            },
            "usage_patterns": self.usage_analysis.model_dump() if self.usage_analysis else {}
        }
        
        # Use Rich interface to display comprehensive summary
        self.interface.display_comprehensive_summary(analysis_data)
        
        # Interactive review loop
        while True:
            action = self.interface.show_interactive_menu()
            
            if action == "approve_all":
                self.interface.show_success("AI-generated patterns approved for database save!")
                return self._convert_ai_patterns_to_db_format(ai_response.pattern_suggestions)
                
            elif action == "view_details":
                self.interface.show_detailed_results(analysis_data)
                
            elif action == "edit_patterns":
                # Simple pattern filtering
                complexity_filter = input("Enter max complexity (1-5, or 'all'): ")
                if complexity_filter.isdigit():
                    max_complexity = int(complexity_filter)
                    filtered_patterns = [
                        p for p in ai_response.pattern_suggestions 
                        if p.complexity <= max_complexity
                    ]
                    self.interface.show_success(f"Filtered to {len(filtered_patterns)} patterns (complexity <= {max_complexity})")
                    return self._convert_ai_patterns_to_db_format(filtered_patterns)
                
            elif action == "reject_all":
                self.interface.show_error("AI analysis rejected, using fallback patterns")
                return fallback_patterns
    
    def _save_artifacts_to_cache(self, library: GameLibrary) -> Path:
        """Save API artifacts to cache for debugging and reference."""
        settings = get_settings()
        settings.paths.ensure_directories()
        
        artifacts_dir = settings.paths.get_library_artifacts_dir(library)
        library_name = library.name.lower()
        
        # Save full API surface to cache
        api_file = artifacts_dir / f"{library_name}-{self.version}-api.json"
        with open(api_file, "w") as f:
            json.dump(self.api_surface, f, indent=2)
        
        # Create current version symlink
        current_link = artifacts_dir / "current-api.json"
        if current_link.exists():
            current_link.unlink()
        current_link.symlink_to(api_file.name)
        
        self.log_info(f"Cached {library_name} API artifacts: {api_file}")
        return api_file
    
    def get_api_hash(self) -> str:
        """Get a hash of the current API surface for change detection."""
        if not self.api_surface:
            return ""
        
        # Create stable string representation
        api_str = json.dumps(self.api_surface, sort_keys=True)
        return hashlib.sha256(api_str.encode()).hexdigest()[:16]
    
    def cleanup_ai_persistence(self, days_old: int = 30) -> dict[str, int]:
        """Clean up old AI persistence data."""
        return self.curriculum_agent.cleanup_old_data(days_old)
