"""
HBF Diagnostics Mixin
Analyzes and diagnoses data structure issues in HBF files
"""

import json
from collections import Counter
from typing import Any

from rich.table import Table
from rich import box
from rich.panel import Panel
from rich.syntax import Syntax

from dragons_labyrinth.hbf.base import SQLiteMixin


class HBFDiagnostics(SQLiteMixin):
    """Diagnostics mixin for HBF database files"""
    
    def diagnose_json_issues(self) -> dict:
        """Diagnose why JSON parsing is failing"""
        self.console.print(Panel.fit("[bold cyan]🔍 JSON Parsing Diagnostics[/bold cyan]"))
        
        if not self.sqlite_conn:
            return {"error": "No SQLite connection available"}
        
        cursor = self.sqlite_conn.cursor()
        cursor.execute("SELECT uuid, value FROM Entities")
        
        issues = {
            'total': 0,
            'empty_string': 0,
            'null': 0,
            'empty_object': 0,
            'valid_json': 0,
            'invalid_json': 0,
            'malformed': [],
            'non_string': 0,
            'types_found': Counter()
        }
        
        for uuid, value in cursor:
            issues['total'] += 1
            
            # Check various conditions
            if value is None:
                issues['null'] += 1
            elif value == '':
                issues['empty_string'] += 1
            elif value == '{}':
                issues['empty_object'] += 1
            else:
                try:
                    data = json.loads(value)
                    issues['valid_json'] += 1
                    if 'entity_type' in data:
                        issues['types_found'][data['entity_type']] += 1
                except (json.JSONDecodeError, TypeError) as e:
                    issues['invalid_json'] += 1
                    if len(issues['malformed']) < 5:  # Keep first 5 examples
                        issues['malformed'].append({
                            'uuid': uuid[:8] + '...',
                            'error': str(e),
                            'sample': value[:100] if value else None
                        })
        
        # Display results
        table = Table(title="JSON Parsing Analysis", box=box.ROUNDED)
        table.add_column("Issue Type", style="cyan")
        table.add_column("Count", style="magenta")
        table.add_column("Percentage", style="green")
        
        total = issues['total']
        table.add_row("Total Entities", f"{total:,}", "100%")
        table.add_row("Valid JSON", f"{issues['valid_json']:,}", 
                     f"{(issues['valid_json']/total*100):.1f}%")
        table.add_row("Invalid JSON", f"{issues['invalid_json']:,}", 
                     f"{(issues['invalid_json']/total*100):.1f}%")
        table.add_row("Empty Objects {}", f"{issues['empty_object']:,}", 
                     f"{(issues['empty_object']/total*100):.1f}%")
        table.add_row("Empty Strings", f"{issues['empty_string']:,}", 
                     f"{(issues['empty_string']/total*100):.1f}%")
        table.add_row("NULL Values", f"{issues['null']:,}", 
                     f"{(issues['null']/total*100):.1f}%")
        
        self.console.print(table)
        
        if issues['malformed']:
            self.console.print("\n[red]Sample Malformed Entries:[/red]")
            for entry in issues['malformed']:
                self.console.print(f"  UUID: {entry['uuid']}")
                self.console.print(f"  Error: {entry['error']}")
                if entry['sample']:
                    self.console.print(f"  Sample: {entry['sample']}")
        
        return issues
    
    def check_schema(self) -> dict:
        """Check database schema"""
        self.console.print("\n[cyan]Database Schema:[/cyan]")
        
        if not self.sqlite_conn:
            return {"error": "No SQLite connection available"}
        
        cursor = self.sqlite_conn.cursor()
        
        # Get all tables
        cursor.execute("SELECT name FROM sqlite_master WHERE type='table'")
        tables = [row[0] for row in cursor.fetchall()]
        
        schema_info = {}
        for table in tables:
            # Get table schema
            cursor.execute(f"PRAGMA table_info({table})")
            columns = cursor.fetchall()
            
            # Get row count
            cursor.execute(f"SELECT COUNT(*) FROM {table}")
            count = cursor.fetchone()[0]
            
            schema_info[table] = {
                "columns": [{"name": col[1], "type": col[2]} for col in columns],
                "row_count": count
            }
            
            self.console.print(f"  [yellow]{table}:[/yellow] {count:,} rows")
            for col in columns:
                self.console.print(f"    - {col[1]} ({col[2]})")
        
        return schema_info
    
    def analyze_entities(self) -> dict:
        """Analyze Entities table"""
        self.console.print("\n[cyan]Entities Analysis:[/cyan]")
        
        if not self.sqlite_conn:
            return {"error": "No SQLite connection available"}
        
        cursor = self.sqlite_conn.cursor()
        
        # Total count
        cursor.execute("SELECT COUNT(*) FROM Entities")
        total = cursor.fetchone()[0]
        
        # Count empty entities
        cursor.execute("SELECT COUNT(*) FROM Entities WHERE value = '{}'")
        empty = cursor.fetchone()[0]
        
        # Sample some entities
        cursor.execute("SELECT uuid, value FROM Entities WHERE value != '{}' LIMIT 5")
        samples = cursor.fetchall()
        
        analysis = {
            "total": total,
            "empty": empty,
            "with_content": total - empty,
            "samples": []
        }
        
        # Display summary
        table = Table(title="Entity Statistics", box=box.ROUNDED)
        table.add_column("Metric", style="cyan")
        table.add_column("Value", style="magenta")
        table.add_column("Percentage", style="green")
        
        table.add_row("Total Entities", f"{total:,}", "100%")
        table.add_row("With Content", f"{total - empty:,}", f"{((total-empty)/total*100):.1f}%")
        table.add_row("Empty", f"{empty:,}", f"{(empty/total*100):.1f}%")
        
        self.console.print(table)
        
        # Show samples
        if samples:
            self.console.print("\n[cyan]Sample Entities:[/cyan]")
            for i, (uuid, value) in enumerate(samples[:3], 1):
                self.console.print(f"\n  [yellow]Sample {i} (UUID: {uuid[:8]}...):[/yellow]")
                try:
                    data = json.loads(value)
                    # Pretty print JSON with syntax highlighting
                    json_str = json.dumps(data, indent=2)[:500]  # Limit to 500 chars
                    syntax = Syntax(json_str, "json", theme="monokai", line_numbers=False)
                    self.console.print(syntax)
                    analysis["samples"].append({"uuid": uuid, "data": data})
                except json.JSONDecodeError:
                    self.console.print(f"    [red]Invalid JSON[/red]")
        
        return analysis
    
    def analyze_references(self) -> dict:
        """Analyze References table"""
        self.console.print("\n[cyan]References Analysis:[/cyan]")
        
        if not self.sqlite_conn:
            return {"error": "No SQLite connection available"}
        
        cursor = self.sqlite_conn.cursor()
        
        # Check if Refs table exists
        cursor.execute("SELECT name FROM sqlite_master WHERE type='table' AND name='Refs'")
        if not cursor.fetchone():
            self.console.print("  [yellow]No Refs table found[/yellow]")
            return {"exists": False}
        
        # Total count
        cursor.execute("SELECT COUNT(*) FROM Refs")
        total = cursor.fetchone()[0]
        
        # Count unique source entities
        cursor.execute("SELECT COUNT(DISTINCT uuid) FROM Refs")
        unique_sources = cursor.fetchone()[0]
        
        # Count unique target entities
        cursor.execute("SELECT COUNT(DISTINCT value) FROM Refs")
        unique_targets = cursor.fetchone()[0]
        
        analysis = {
            "exists": True,
            "total_references": total,
            "unique_sources": unique_sources,
            "unique_targets": unique_targets
        }
        
        table = Table(title="Reference Statistics", box=box.ROUNDED)
        table.add_column("Metric", style="cyan")
        table.add_column("Value", style="magenta")
        
        table.add_row("Total References", f"{total:,}")
        table.add_row("Unique Source Entities", f"{unique_sources:,}")
        table.add_row("Unique Target Entities", f"{unique_targets:,}")
        
        self.console.print(table)
        
        return analysis
