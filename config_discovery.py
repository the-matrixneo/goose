#!/usr/bin/env python3
"""
Comprehensive Configuration Discovery Tool for Goose

This script reliably finds ALL configuration usage in the Goose codebase:
1. Config file parameters (get_param, set_param, delete)
2. Secret storage (get_secret, set_secret, delete_secret)
3. Environment variables (std::env::var, env::var, etc.)
4. CLI flags (clap annotations)

Features:
- Deduplication and categorization
- Context extraction
- False positive filtering
- Multiple output formats
- Detailed analysis with file locations

Usage:
    python3 config_discovery.py --help
    python3 config_discovery.py --output config_report.md
    python3 config_discovery.py --json config_data.json --include-context
"""

import os
import re
import json
import argparse
from pathlib import Path
from collections import defaultdict
from dataclasses import dataclass, asdict
from typing import Dict, List, Set, Optional, Tuple

@dataclass
class ConfigUsage:
    key: str
    category: str      # 'config_file', 'secrets', 'environment', 'cli_flags'
    method: str        # specific method used (get_param, env::var, etc.)
    file_path: str     # relative path from root
    line_number: int
    context: Optional[str] = None
    description: Optional[str] = None
    is_test: bool = False

class ConfigDiscovery:
    def __init__(self, root_path: str, include_tests: bool = False):
        self.root_path = Path(root_path)
        self.include_tests = include_tests
        self.usages: List[ConfigUsage] = []
        self.const_map: Dict[str, str] = {}
        
        # Comprehensive patterns for different configuration types
        self.config_patterns = {
            # Unified configuration API
            'unified_get': [
                r'unified::get::<[^>]*>\s*\(\s*"([^"]+)"',
                r'unified::get_or::<[^>]*>\s*\(\s*"([^"]+)"',
                r'unified::get_or\s*\(\s*"([^"]+)"',
                r'unified::resolve::<[^>]*>\s*\(\s*"([^"]+)"',
                r'unified::get_secret::<[^>]*>\s*\(\s*"([^"]+)"',
                r'goose::config::unified::get::<[^>]*>\s*\(\s*"([^"]+)"',
                r'goose::config::unified::get_or::<[^>]*>\s*\(\s*"([^"]+)"',
                r'goose::config::unified::get_or\s*\(\s*"([^"]+)"',
                r'goose::config::unified::resolve::<[^>]*>\s*\(\s*"([^"]+)"',
            ],
            'unified_set': [
                r'unified::set\s*\(\s*"([^"]+)"',
                r'unified::set_secret\s*\(\s*"([^"]+)"',
                r'unified::unset\s*\(\s*"([^"]+)"',
            ],
            # Config file operations (narrowed to Config usage to avoid JSON .get false positives)
            'config_get': [
                r'Config::global\(\)\.get_param\s*:?\s*<[^>]*>\s*\(\s*["\']([^"\']+)["\']',
                r'Config::global\(\)\.get_param\s*\(\s*["\']([^"\']+)["\']',
                r'config\.get_param\s*:?\s*<[^>]*>\s*\(\s*["\']([^"\']+)["\']',
                r'config\.get_param\s*\(\s*["\']([^"\']+)["\']',
                # Generic function form (when in scope)
                r'get_param\s*\(\s*["\']([^"\']+)["\']',
                # Config::global().get("KEY", bool)
                r'Config::global\(\)\.get\s*\(\s*["\']([^"\']+)["\']\s*,\s*(true|false)\s*\)',
                r'config\.get\s*\(\s*["\']([^"\']+)["\']\s*,\s*(true|false)\s*\)'
            ],
            'config_set': [
                r'Config::global\(\)\.set_param\s*\(\s*["\']([^"\']+)["\']',
                r'config\.set_param\s*\(\s*["\']([^"\']+)["\']',
                r'set_param\s*\(\s*["\']([^"\']+)["\']'
            ],
            'config_delete': [
                r'Config::global\(\)\.delete\s*\(\s*["\']([^"\']+)["\']',
                r'config\.delete\s*\(\s*["\']([^"\']+)["\']'
            ],
            
            # Secret storage operations
            'secret_get': [
                r'Config::global\(\)\.get_secret\s*:?\s*<[^>]*>\s*\(\s*["\']([^"\']+)["\']',
                r'Config::global\(\)\.get_secret\s*\(\s*["\']([^"\']+)["\']',
                r'config\.get_secret\s*:?\s*<[^>]*>\s*\(\s*["\']([^"\']+)["\']',
                r'config\.get_secret\s*\(\s*["\']([^"\']+)["\']',
                r'get_secret\s*\(\s*["\']([^"\']+)["\']',
            ],
            'secret_set': [
                r'Config::global\(\)\.set_secret\s*\(\s*["\']([^"\']+)["\']',
                r'config\.set_secret\s*\(\s*["\']([^"\']+)["\']',
                r'set_secret\s*\(\s*["\']([^"\']+)["\']',
            ],
            'secret_delete': [
                r'Config::global\(\)\.delete_secret\s*\(\s*["\']([^"\']+)["\']',
                r'config\.delete_secret\s*\(\s*["\']([^"\']+)["\']',
                r'delete_secret\s*\(\s*["\']([^"\']+)["\']',
            ],
            
            # Environment variable operations (literal keys)
            'env_var': [
                r'std::env::var\s*\(\s*["\']([^"\']+)["\']',
                r'env::var\s*\(\s*["\']([^"\']+)["\']',
            ],
            'env_set': [
                r'std::env::set_var\s*\(\s*["\']([^"\']+)["\']',
                r'env::set_var\s*\(\s*["\']([^"\']+)["\']',
            ],
            'env_remove': [
                r'std::env::remove_var\s*\(\s*["\']([^"\']+)["\']',
                r'env::remove_var\s*\(\s*["\']([^"\']+)["\']',
            ],
            'env_var_os': [
                r'std::env::var_os\s*\(\s*["\']([^"\']+)["\']',
                r'env::var_os\s*\(\s*["\']([^"\']+)["\']',
            ],
            
            # Constants and environment/config key references
            'env_const': [
                # Map CONST_NAME -> "VALUE"
                r'(?:pub\s+)?const\s+([A-Z_][A-Z0-9_]*)\s*:\s*&str\s*=\s*["\']([A-Z0-9_]+)["\']',
                r'(?:pub\s+)?static\s+([A-Z_][A-Z0-9_]*)\s*:\s*&str\s*=\s*["\']([A-Z0-9_]+)["\']',
            ],
            
            # Provider config key declarations
            'provider_config_decl': [
                r'ConfigKey::new_oauth\s*\(\s*\"([^\"]+)\"\s*,\s*(true|false)\s*,\s*(true|false)',
                r'ConfigKey::new\s*\(\s*\"([^\"]+)\"\s*,\s*(true|false)\s*,\s*(true|false)'
            ],
        }
        
    def find_rust_files(self) -> List[Path]:
        """Find all relevant Rust files."""
        rust_files = []
        for rust_file in self.root_path.rglob("*.rs"):
            # Skip target directory and git
            if 'target' in rust_file.parts or '.git' in rust_file.parts:
                continue
                
            # Skip test files unless explicitly included
            if not self.include_tests:
                if (
                    '/tests/' in str(rust_file) or
                    rust_file.name.endswith('_test.rs') or
                    rust_file.name.startswith('test_') or
                    'test' in rust_file.stem.lower()
                ):
                    continue
                    
            rust_files.append(rust_file)
            
        return rust_files
    
    def extract_context(self, content: str, line_num: int, context_lines: int = 2) -> str:
        """Extract code context around a match."""
        lines = content.split('\n')
        start = max(0, line_num - context_lines - 1)
        end = min(len(lines), line_num + context_lines)
        return '\n'.join(lines[start:end]).strip()
    
    def is_valid_env_var(self, key: str) -> bool:
        """Validate if this looks like a real environment variable."""
        # Skip obvious false positives
        false_positives = {
            'OK', 'ERR', 'SOME', 'NONE', 'TRUE', 'FALSE', 'DEBUG', 'INFO', 'WARN', 'ERROR',
            'GET', 'POST', 'PUT', 'DELETE', 'HEAD', 'OPTIONS', 'PATCH', 'CONNECT', 'TRACE',
            'HTTP', 'HTTPS', 'FTP', 'SSH', 'TCP', 'UDP', 'IP', 'DNS', 'SSL', 'TLS',
            'JSON', 'XML', 'YAML', 'HTML', 'CSS', 'JS', 'CSV', 'PDF', 'PNG', 'JPG',
            'UTF', 'ASCII', 'UNICODE', 'BASE64', 'HEX', 'MD5', 'SHA', 'UUID',
            'API', 'URL', 'URI', 'ID', 'UID', 'GID', 'PID', 'SID',
        }
        
        if key in false_positives:
            return False
            
        # Must be at least 3 characters for most cases, but allow some 2-char exceptions
        if len(key) < 2:
            return False
        
        # Allow some known 2-character environment variables
        known_two_char = {'CI', 'OS'}
        if len(key) == 2 and key not in known_two_char:
            return False
            
        # Skip test-related variables unless they start with GOOSE
        if not key.startswith('GOOSE') and any(test_word in key.lower() for test_word in ['test', 'mock', 'temp', 'tmp', 'dummy']):
            return False
            
        # Must be uppercase with underscores or known single-word vars
        known_single = {'HOME', 'USER', 'PATH', 'LANG', 'TERM', 'SHELL', 'PWD', 'PORT', 'CI', 'OS'}
        if not (key.isupper() and ('_' in key or key in known_single)):
            return False
            
        return True
    
    def categorize_usage(self, method: str) -> str:
        """Categorize usage by method type."""
        if method.startswith('unified_'):
            return 'unified_config'
        elif method.startswith('config_'):
            return 'config_file'
        elif method.startswith('secret_'):
            return 'secrets'
        elif method.startswith('env_'):
            return 'environment'
        elif method.startswith('cli_'):
            return 'cli_flags'
        else:
            return 'other'
    
    def analyze_file(self, file_path: Path, include_context: bool = False):
        """Analyze a single file for configuration usage."""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
        except (UnicodeDecodeError, PermissionError):
            return
            
        relative_path = str(file_path.relative_to(self.root_path))
        is_test_file = 'test' in relative_path.lower()
        
        # Search for configuration patterns
        for method, patterns in self.config_patterns.items():
            for pattern in patterns:
                for match in re.finditer(pattern, content, re.MULTILINE):
                    key = match.group(1)
                    line_num = content[:match.start()].count('\n') + 1
                    
                    # Filter environment variables
                    if method.startswith('env_') and not self.is_valid_env_var(key):
                        continue
                    
                    # Skip very short keys that are likely false positives
                    if len(key) < 2:
                        continue
                    
                    context = None
                    if include_context:
                        context = self.extract_context(content, line_num)
                    
                    usage = ConfigUsage(
                        key=key,
                        category=self.categorize_usage(method),
                        method=method,
                        file_path=relative_path,
                        line_number=line_num,
                        context=context,
                        is_test=is_test_file
                    )
                    
                    self.usages.append(usage)
        
        # Analyze CLI flags separately
        self.analyze_cli_flags(content, relative_path, include_context, is_test_file)
    
    def analyze_cli_flags(self, content: str, file_path: str, include_context: bool, is_test: bool):
        """Analyze CLI flags from clap annotations."""
        # Find #[arg(...)] patterns
        arg_pattern = r'#\[arg\(([^)]+)\)\]'
        for match in re.finditer(arg_pattern, content, re.MULTILINE | re.DOTALL):
            line_num = content[:match.start()].count('\n') + 1
            arg_content = match.group(1)
            
            context = None
            if include_context:
                context = self.extract_context(content, line_num, 3)
            
            # Extract long flag
            long_match = re.search(r'long\s*=\s*["\']([^"\']+)["\']', arg_content)
            if long_match:
                flag_name = long_match.group(1)
                
                # Extract help text for description
                help_match = re.search(r'help\s*=\s*["\']([^"\']*)["\']', arg_content)
                description = help_match.group(1) if help_match else None
                
                self.usages.append(ConfigUsage(
                    key=f"--{flag_name}",
                    category='cli_flags',
                    method='clap_long',
                    file_path=file_path,
                    line_number=line_num,
                    context=context,
                    description=description,
                    is_test=is_test
                ))
            
            # Extract short flag
            short_match = re.search(r'short\s*=\s*["\'](.)["\']', arg_content)
            if short_match:
                flag_char = short_match.group(1)
                
                self.usages.append(ConfigUsage(
                    key=f"-{flag_char}",
                    category='cli_flags',
                    method='clap_short',
                    file_path=file_path,
                    line_number=line_num,
                    context=context,
                    is_test=is_test
                ))
    
    def run_analysis(self, include_context: bool = False):
        """Run the complete configuration analysis."""
        rust_files = self.find_rust_files()
        print(f"Analyzing {len(rust_files)} Rust files...")
        
        for i, file_path in enumerate(rust_files):
            if i % 25 == 0:
                print(f"Progress: {i}/{len(rust_files)} files")
            self.analyze_file(file_path, include_context)
        
        print(f"Analysis complete. Found {len(self.usages)} configuration usages.")
    
    def deduplicate(self) -> Dict[str, List[ConfigUsage]]:
        """Group usages by key, removing duplicates."""
        by_key = defaultdict(list)
        
        for usage in self.usages:
            by_key[usage.key].append(usage)
        
        # Sort usages within each key by file path and line number
        for key in by_key:
            by_key[key].sort(key=lambda x: (x.file_path, x.line_number))
        
        return dict(by_key)
    
    def generate_summary_stats(self) -> Dict:
        """Generate summary statistics."""
        by_category = defaultdict(set)
        by_method = defaultdict(int)
        files_with_config = set()
        
        for usage in self.usages:
            by_category[usage.category].add(usage.key)
            by_method[usage.method] += 1
            files_with_config.add(usage.file_path)
        
        return {
            'total_usages': len(self.usages),
            'unique_keys': len(set(usage.key for usage in self.usages)),
            'by_category': {cat: len(keys) for cat, keys in by_category.items()},
            'by_method': dict(by_method),
            'files_analyzed': len(files_with_config),
            'test_usages': len([u for u in self.usages if u.is_test])
        }
    
    def generate_markdown_report(self) -> str:
        """Generate a comprehensive markdown report."""
        grouped = self.deduplicate()
        stats = self.generate_summary_stats()
        
        report = "# Comprehensive Goose Configuration Analysis\n\n"
        
        # Summary section
        report += "## Summary\n\n"
        report += f"- **Total Configuration Usages:** {stats['total_usages']}\n"
        report += f"- **Unique Configuration Keys:** {stats['unique_keys']}\n"
        report += f"- **Files with Configuration:** {stats['files_analyzed']}\n"
        if stats['test_usages'] > 0:
            report += f"- **Test-related Usages:** {stats['test_usages']}\n"
        report += "\n"
        
        # Category breakdown
        report += "### By Category\n\n"
        category_names = {
            'unified_config': 'Unified Configuration API',
            'config_file': 'Config File Parameters',
            'secrets': 'Secret Storage',
            'environment': 'Environment Variables',
            'cli_flags': 'CLI Flags'
        }
        
        for category, count in stats['by_category'].items():
            name = category_names.get(category, category.title())
            report += f"- **{name}:** {count} unique keys\n"
        report += "\n"
        
        # Detailed sections
        for category in ['unified_config', 'config_file', 'secrets', 'environment', 'cli_flags']:
            if category not in stats['by_category']:
                continue
                
            category_usages = [
                (key, usages) for key, usages in grouped.items()
                if usages[0].category == category
            ]
            
            if not category_usages:
                continue
            
            name = category_names.get(category, category.title())
            report += f"## {name}\n\n"
            
            for key, usages in sorted(category_usages):
                report += f"### `{key}`\n\n"
                
                # Show description if available
                if usages[0].description:
                    report += f"**Description:** {usages[0].description}\n\n"
                
                # Show method(s)
                methods = list(set(u.method for u in usages))
                report += f"**Method(s):** {', '.join(methods)}\n\n"
                
                # Show usage locations
                report += f"**Usage Locations ({len(usages)}):**\n\n"
                for usage in usages[:10]:  # Limit to first 10
                    test_marker = " (test)" if usage.is_test else ""
                    report += f"- `{usage.file_path}:{usage.line_number}`{test_marker}\n"
                
                if len(usages) > 10:
                    report += f"- ... and {len(usages) - 10} more locations\n"
                
                # Show context for first non-test usage if available
                non_test_usages = [u for u in usages if not u.is_test and u.context]
                if non_test_usages:
                    report += f"\n**Example Context:**\n```rust\n{non_test_usages[0].context}\n```\n"
                
                report += "\n"
        
        return report
    
    def export_json(self) -> dict:
        """Export data as JSON."""
        grouped = self.deduplicate()
        stats = self.generate_summary_stats()
        
        return {
            'summary': stats,
            'configuration_items': {
                key: {
                    'key': key,
                    'category': usages[0].category,
                    'methods': list(set(u.method for u in usages)),
                    'description': usages[0].description,
                    'usage_count': len(usages),
                    'locations': [
                        {
                            'file': u.file_path,
                            'line': u.line_number,
                            'method': u.method,
                            'is_test': u.is_test,
                            'context': u.context
                        }
                        for u in usages
                    ]
                }
                for key, usages in grouped.items()
            }
        }

def main():
    parser = argparse.ArgumentParser(
        description="Comprehensive configuration discovery for Goose",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python3 config_discovery.py --output config_report.md
  python3 config_discovery.py --json config_data.json --include-context
  python3 config_discovery.py --include-tests --format json
        """
    )
    
    parser.add_argument("--root", default=".", help="Root directory to analyze")
    parser.add_argument("--output", help="Output markdown file")
    parser.add_argument("--json", help="Output JSON file")
    parser.add_argument("--format", choices=['markdown', 'json'], default='markdown',
                       help="Output format when printing to stdout")
    parser.add_argument("--include-context", action='store_true',
                       help="Include code context in results")
    parser.add_argument("--include-tests", action='store_true',
                       help="Include test files in analysis")
    
    args = parser.parse_args()
    
    # Run analysis
    discovery = ConfigDiscovery(args.root, args.include_tests)
    discovery.run_analysis(args.include_context)
    
    # Generate outputs
    if args.output or args.format == 'markdown':
        report = discovery.generate_markdown_report()
        if args.output:
            with open(args.output, 'w') as f:
                f.write(report)
            print(f"Markdown report saved to {args.output}")
        else:
            print(report)
    
    if args.json or args.format == 'json':
        data = discovery.export_json()
        json_str = json.dumps(data, indent=2)
        if args.json:
            with open(args.json, 'w') as f:
                f.write(json_str)
            print(f"JSON data saved to {args.json}")
        else:
            print(json_str)
    
    # Print summary
    stats = discovery.generate_summary_stats()
    print(f"\n=== Analysis Summary ===")
    print(f"Total configuration usages: {stats['total_usages']}")
    print(f"Unique configuration keys: {stats['unique_keys']}")
    print(f"Files with configuration: {stats['files_analyzed']}")
    for category, count in stats['by_category'].items():
        print(f"  {category}: {count} keys")

if __name__ == "__main__":
    main()
