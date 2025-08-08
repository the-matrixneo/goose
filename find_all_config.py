#!/usr/bin/env python3
"""
Comprehensive Configuration Discovery Script for Goose

This script systematically searches the Goose codebase for ALL configuration usage patterns:
1. Config file parameters (get_param, set_param, etc.)
2. Environment variables (env::var, std::env::var)
3. CLI flags (clap annotations)
4. Secret storage (get_secret, set_secret)

Usage: python3 find_all_config.py [--output-format json|markdown|text]
"""

import os
import re
import json
import argparse
from pathlib import Path
from collections import defaultdict, namedtuple
from dataclasses import dataclass, asdict
from typing import Dict, List, Set, Optional, Union

@dataclass
class ConfigUsage:
    key: str
    usage_type: str  # 'config_param', 'env_var', 'cli_flag', 'secret'
    method: str      # 'get_param', 'env::var', 'clap', etc.
    file_path: str
    line_number: int
    context: str     # surrounding code context
    description: Optional[str] = None

class ConfigFinder:
    def __init__(self, root_path: str):
        self.root_path = Path(root_path)
        self.results: List[ConfigUsage] = []
        self.rust_files = []
        
        # Patterns for different config types
        self.patterns = {
            # Config file parameters
            'config_param_get': [
                r'config\.get_param\s*\(\s*["\']([^"\']+)["\']',
                r'\.get_param\s*:?\s*<[^>]*>\s*\(\s*["\']([^"\']+)["\']',
                r'get_param\s*\(\s*["\']([^"\']+)["\']',
            ],
            'config_param_set': [
                r'config\.set_param\s*\(\s*["\']([^"\']+)["\']',
                r'\.set_param\s*\(\s*["\']([^"\']+)["\']',
                r'set_param\s*\(\s*["\']([^"\']+)["\']',
            ],
            'config_delete': [
                r'config\.delete\s*\(\s*["\']([^"\']+)["\']',
                r'\.delete\s*\(\s*["\']([^"\']+)["\']',
            ],
            
            # Secret storage
            'secret_get': [
                r'config\.get_secret\s*\(\s*["\']([^"\']+)["\']',
                r'\.get_secret\s*\(\s*["\']([^"\']+)["\']',
                r'get_secret\s*\(\s*["\']([^"\']+)["\']',
            ],
            'secret_set': [
                r'config\.set_secret\s*\(\s*["\']([^"\']+)["\']',
                r'\.set_secret\s*\(\s*["\']([^"\']+)["\']',
                r'set_secret\s*\(\s*["\']([^"\']+)["\']',
            ],
            'secret_delete': [
                r'config\.delete_secret\s*\(\s*["\']([^"\']+)["\']',
                r'\.delete_secret\s*\(\s*["\']([^"\']+)["\']',
                r'delete_secret\s*\(\s*["\']([^"\']+)["\']',
            ],
            
            # Environment variables
            'env_var_std': [
                r'std::env::var\s*\(\s*["\']([^"\']+)["\']',
                r'env::var\s*\(\s*["\']([^"\']+)["\']',
            ],
            'env_var_ok': [
                r'std::env::var\s*\(\s*["\']([^"\']+)["\'].*\.ok\(\)',
                r'env::var\s*\(\s*["\']([^"\']+)["\'].*\.ok\(\)',
            ],
            'env_var_is_ok': [
                r'std::env::var\s*\(\s*["\']([^"\']+)["\'].*\.is_ok\(\)',
                r'env::var\s*\(\s*["\']([^"\']+)["\'].*\.is_ok\(\)',
            ],
            'env_var_unwrap': [
                r'std::env::var\s*\(\s*["\']([^"\']+)["\'].*\.unwrap_or',
                r'env::var\s*\(\s*["\']([^"\']+)["\'].*\.unwrap_or',
            ],
            'env_set_var': [
                r'std::env::set_var\s*\(\s*["\']([^"\']+)["\']',
                r'env::set_var\s*\(\s*["\']([^"\']+)["\']',
            ],
            'env_remove_var': [
                r'std::env::remove_var\s*\(\s*["\']([^"\']+)["\']',
                r'env::remove_var\s*\(\s*["\']([^"\']+)["\']',
            ],
            'env_var_os': [
                r'std::env::var_os\s*\(\s*["\']([^"\']+)["\']',
                r'env::var_os\s*\(\s*["\']([^"\']+)["\']',
            ],
            
            # String literals that might be env vars (common patterns)
            'potential_env_vars': [
                r'["\']([A-Z][A-Z0-9_]*)["\']',  # ALL_CAPS strings
                r'["\'](GOOSE_[A-Z0-9_]*)["\']',  # GOOSE_ prefixed
                r'["\']([A-Z]+_[A-Z0-9_]*)["\']',  # Pattern with underscores
            ],
        }
        
        # CLI flag patterns (clap annotations)
        self.cli_patterns = {
            'arg_long': r'#\[arg\([^)]*long\s*=\s*["\']([^"\']+)["\']',
            'arg_short': r'#\[arg\([^)]*short\s*=\s*["\']([^"\']+)["\']',
            'arg_short_char': r'#\[arg\([^)]*short\s*=\s*["\'](.)["\']',
            'arg_help': r'#\[arg\([^)]*help\s*=\s*["\']([^"\']*)["\']',
            'command_about': r'#\[command\([^)]*about\s*=\s*["\']([^"\']*)["\']',
        }
        
    def find_rust_files(self):
        """Find all Rust files in the codebase."""
        for rust_file in self.root_path.rglob("*.rs"):
            # Skip target directory and other build artifacts
            if 'target' not in rust_file.parts and '.git' not in rust_file.parts:
                self.rust_files.append(rust_file)
        print(f"Found {len(self.rust_files)} Rust files to analyze")
    
    def extract_context(self, content: str, line_num: int, context_lines: int = 2) -> str:
        """Extract surrounding context for a match."""
        lines = content.split('\n')
        start = max(0, line_num - context_lines - 1)
        end = min(len(lines), line_num + context_lines)
        context = '\n'.join(lines[start:end])
        return context.strip()
    
    def find_config_usage_in_file(self, file_path: Path):
        """Find all configuration usage in a single file."""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
        except (UnicodeDecodeError, PermissionError):
            return
        
        lines = content.split('\n')
        relative_path = str(file_path.relative_to(self.root_path))
        
        # Search for each pattern type
        for usage_type, patterns in self.patterns.items():
            for pattern in patterns:
                for match in re.finditer(pattern, content, re.MULTILINE | re.IGNORECASE):
                    # Find line number
                    line_num = content[:match.start()].count('\n') + 1
                    key = match.group(1)
                    
                    # Skip very short or obviously non-config strings
                    if len(key) < 2 or key.isdigit():
                        continue
                        
                    # For potential env vars, apply additional filtering
                    if usage_type == 'potential_env_vars':
                        if not self.is_likely_env_var(key, match.group(0)):
                            continue
                    
                    context = self.extract_context(content, line_num)
                    
                    self.results.append(ConfigUsage(
                        key=key,
                        usage_type=usage_type,
                        method=usage_type,
                        file_path=relative_path,
                        line_number=line_num,
                        context=context
                    ))
        
        # Search for CLI flags
        self.find_cli_flags_in_file(file_path, content)
    
    def is_likely_env_var(self, key: str, full_match: str) -> bool:
        """Filter potential environment variables to reduce false positives."""
        # Skip common false positives
        false_positives = {
            'OK', 'ERR', 'SOME', 'NONE', 'TRUE', 'FALSE', 'DEBUG', 'INFO', 'WARN', 'ERROR',
            'GET', 'POST', 'PUT', 'DELETE', 'HEAD', 'OPTIONS', 'PATCH',
            'UTF', 'ASCII', 'JSON', 'YAML', 'XML', 'HTML', 'CSS', 'JS',
            'HTTP', 'HTTPS', 'FTP', 'SSH', 'TCP', 'UDP', 'IP',
            'API', 'URL', 'URI', 'ID', 'UUID', 'MD5', 'SHA',
        }
        
        if key in false_positives:
            return False
            
        # Must be at least 3 characters
        if len(key) < 3:
            return False
            
        # Should contain at least one underscore for multi-word env vars
        # or be a known single-word env var
        known_single_word = {'HOME', 'USER', 'PATH', 'LANG', 'TERM', 'SHELL', 'PWD', 'TEMP', 'TMP'}
        if '_' not in key and key not in known_single_word:
            return False
            
        # Should be mostly uppercase
        if not key.isupper():
            return False
            
        return True
    
    def find_cli_flags_in_file(self, file_path: Path, content: str):
        """Find CLI flags defined with clap annotations."""
        relative_path = str(file_path.relative_to(self.root_path))
        
        # Look for clap argument definitions
        arg_pattern = r'#\[arg\([^)]+\)\]\s*(\w+):\s*([^,\n]+)'
        for match in re.finditer(arg_pattern, content, re.MULTILINE | re.DOTALL):
            line_num = content[:match.start()].count('\n') + 1
            arg_annotation = match.group(0)
            field_name = match.group(1)
            
            # Extract long flag name
            long_match = re.search(r'long\s*=\s*["\']([^"\']+)["\']', arg_annotation)
            if long_match:
                flag_name = long_match.group(1)
                context = self.extract_context(content, line_num, 3)
                
                # Extract help text if available
                help_match = re.search(r'help\s*=\s*["\']([^"\']*)["\']', arg_annotation)
                description = help_match.group(1) if help_match else None
                
                self.results.append(ConfigUsage(
                    key=f"--{flag_name}",
                    usage_type='cli_flag',
                    method='clap_long',
                    file_path=relative_path,
                    line_number=line_num,
                    context=context,
                    description=description
                ))
            
            # Extract short flag name
            short_match = re.search(r'short\s*=\s*["\'](.)["\']', arg_annotation)
            if short_match:
                flag_char = short_match.group(1)
                context = self.extract_context(content, line_num, 3)
                
                self.results.append(ConfigUsage(
                    key=f"-{flag_char}",
                    usage_type='cli_flag',
                    method='clap_short',
                    file_path=relative_path,
                    line_number=line_num,
                    context=context
                ))
        
        # Look for subcommand definitions
        command_pattern = r'#\[command\([^)]+\)\]\s*(\w+)\s*{'
        for match in re.finditer(command_pattern, content, re.MULTILINE):
            line_num = content[:match.start()].count('\n') + 1
            command_annotation = match.group(0)
            
            # Extract about text
            about_match = re.search(r'about\s*=\s*["\']([^"\']*)["\']', command_annotation)
            if about_match:
                context = self.extract_context(content, line_num, 3)
                
                self.results.append(ConfigUsage(
                    key=match.group(1),
                    usage_type='cli_command',
                    method='clap_command',
                    file_path=relative_path,
                    line_number=line_num,
                    context=context,
                    description=about_match.group(1)
                ))
    
    def analyze_all_files(self):
        """Analyze all Rust files for configuration usage."""
        print("Analyzing files for configuration usage...")
        for i, file_path in enumerate(self.rust_files):
            if i % 10 == 0:
                print(f"Progress: {i}/{len(self.rust_files)} files")
            self.find_config_usage_in_file(file_path)
        print(f"Analysis complete. Found {len(self.results)} configuration usages.")
    
    def deduplicate_results(self):
        """Remove duplicate results while preserving the most informative ones."""
        seen = set()
        deduplicated = []
        
        for result in self.results:
            # Create a key for deduplication
            key = (result.key, result.usage_type, result.file_path)
            if key not in seen:
                seen.add(key)
                deduplicated.append(result)
        
        self.results = deduplicated
        print(f"After deduplication: {len(self.results)} unique configuration usages.")
    
    def categorize_results(self) -> Dict[str, List[ConfigUsage]]:
        """Categorize results by type."""
        categories = defaultdict(list)
        
        for result in self.results:
            if result.usage_type.startswith('config_param'):
                categories['Config File Parameters'].append(result)
            elif result.usage_type.startswith('secret'):
                categories['Secret Storage'].append(result)
            elif result.usage_type.startswith('env_var') or result.usage_type == 'potential_env_vars':
                categories['Environment Variables'].append(result)
            elif result.usage_type.startswith('cli'):
                categories['CLI Flags'].append(result)
            else:
                categories['Other'].append(result)
        
        return categories
    
    def generate_report(self, output_format: str = 'markdown') -> str:
        """Generate a comprehensive report of all configuration usage."""
        categories = self.categorize_results()
        
        if output_format == 'json':
            return json.dumps({
                category: [asdict(usage) for usage in usages]
                for category, usages in categories.items()
            }, indent=2)
        
        elif output_format == 'markdown':
            report = "# Comprehensive Goose Configuration Analysis\n\n"
            report += f"**Total Configuration Items Found:** {len(self.results)}\n\n"
            
            for category, usages in categories.items():
                report += f"## {category} ({len(usages)} items)\n\n"
                
                # Group by key for better organization
                by_key = defaultdict(list)
                for usage in usages:
                    by_key[usage.key].append(usage)
                
                for key in sorted(by_key.keys()):
                    key_usages = by_key[key]
                    report += f"### `{key}`\n\n"
                    
                    if key_usages[0].description:
                        report += f"**Description:** {key_usages[0].description}\n\n"
                    
                    report += "**Usage locations:**\n"
                    for usage in key_usages:
                        report += f"- `{usage.file_path}:{usage.line_number}` ({usage.method})\n"
                    
                    # Show context for the first usage
                    if key_usages[0].context:
                        report += f"\n**Example context:**\n```rust\n{key_usages[0].context}\n```\n"
                    
                    report += "\n"
            
            return report
        
        else:  # text format
            report = f"Goose Configuration Analysis - {len(self.results)} items found\n"
            report += "=" * 60 + "\n\n"
            
            for category, usages in categories.items():
                report += f"{category} ({len(usages)} items)\n"
                report += "-" * 40 + "\n"
                
                for usage in sorted(usages, key=lambda x: x.key):
                    report += f"{usage.key} ({usage.method}) - {usage.file_path}:{usage.line_number}\n"
                
                report += "\n"
            
            return report
    
    def save_detailed_results(self, output_file: str):
        """Save detailed results to a JSON file for further analysis."""
        detailed_data = {
            'summary': {
                'total_files_analyzed': len(self.rust_files),
                'total_config_items': len(self.results),
                'categories': {
                    category: len(usages) 
                    for category, usages in self.categorize_results().items()
                }
            },
            'results': [asdict(result) for result in self.results]
        }
        
        with open(output_file, 'w') as f:
            json.dump(detailed_data, f, indent=2)
        
        print(f"Detailed results saved to {output_file}")

def main():
    parser = argparse.ArgumentParser(description="Find all configuration usage in Goose codebase")
    parser.add_argument("--root", default=".", help="Root directory to search (default: current directory)")
    parser.add_argument("--output-format", choices=['markdown', 'json', 'text'], default='markdown',
                       help="Output format (default: markdown)")
    parser.add_argument("--output-file", help="Output file (default: stdout)")
    parser.add_argument("--detailed-json", help="Save detailed JSON results to file")
    parser.add_argument("--include-potential", action='store_true', 
                       help="Include potential environment variables (may have false positives)")
    
    args = parser.parse_args()
    
    finder = ConfigFinder(args.root)
    
    # Find and analyze all files
    finder.find_rust_files()
    finder.analyze_all_files()
    
    # Filter out potential env vars if not requested
    if not args.include_potential:
        finder.results = [r for r in finder.results if r.usage_type != 'potential_env_vars']
    
    finder.deduplicate_results()
    
    # Generate report
    report = finder.generate_report(args.output_format)
    
    if args.output_file:
        with open(args.output_file, 'w') as f:
            f.write(report)
        print(f"Report saved to {args.output_file}")
    else:
        print(report)
    
    # Save detailed results if requested
    if args.detailed_json:
        finder.save_detailed_results(args.detailed_json)
    
    # Print summary
    categories = finder.categorize_results()
    print(f"\nSummary:")
    print(f"Total configuration items found: {len(finder.results)}")
    for category, usages in categories.items():
        print(f"  {category}: {len(usages)}")

if __name__ == "__main__":
    main()
