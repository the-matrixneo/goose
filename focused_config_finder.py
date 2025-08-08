#!/usr/bin/env python3
"""
Focused Configuration Discovery Script for Goose

This script finds the most important configuration patterns with high precision:
1. Config file parameters (get_param, set_param)
2. Environment variables (std::env::var, env::var)
3. CLI flags (clap annotations)
4. Secret storage (get_secret, set_secret)

Focuses on actual configuration usage rather than test code or false positives.
"""

import os
import re
import json
import argparse
from pathlib import Path
from collections import defaultdict
from dataclasses import dataclass, asdict
from typing import Dict, List, Set, Optional

@dataclass
class ConfigItem:
    key: str
    config_type: str  # 'config_param', 'env_var', 'cli_flag', 'secret'
    method: str       # specific method used
    locations: List[str]  # file:line combinations
    description: Optional[str] = None
    default_value: Optional[str] = None

class FocusedConfigFinder:
    def __init__(self, root_path: str):
        self.root_path = Path(root_path)
        self.config_items: Dict[str, ConfigItem] = {}
        
    def find_rust_files(self) -> List[Path]:
        """Find all non-test Rust files."""
        rust_files = []
        for rust_file in self.root_path.rglob("*.rs"):
            # Skip test files, target directory, and build artifacts
            if (
                'target' not in rust_file.parts and 
                '.git' not in rust_file.parts and
                not rust_file.name.endswith('_test.rs') and
                '/tests/' not in str(rust_file)
            ):
                rust_files.append(rust_file)
        return rust_files
    
    def extract_config_params(self, content: str, file_path: str):
        """Extract config.get_param and config.set_param usage."""
        patterns = [
            # get_param patterns
            (r'\.get_param\s*:?\s*<[^>]*>\s*\(\s*["\']([^"\']+)["\']', 'get_param'),
            (r'config\.get_param\s*\(\s*["\']([^"\']+)["\']', 'get_param'),
            (r'get_param\s*\(\s*["\']([^"\']+)["\']', 'get_param'),
            
            # set_param patterns
            (r'\.set_param\s*\(\s*["\']([^"\']+)["\']', 'set_param'),
            (r'config\.set_param\s*\(\s*["\']([^"\']+)["\']', 'set_param'),
            (r'set_param\s*\(\s*["\']([^"\']+)["\']', 'set_param'),
        ]
        
        for pattern, method in patterns:
            for match in re.finditer(pattern, content):
                key = match.group(1)
                line_num = content[:match.start()].count('\n') + 1
                location = f"{file_path}:{line_num}"
                
                if key not in self.config_items:
                    self.config_items[key] = ConfigItem(
                        key=key,
                        config_type='config_param',
                        method=method,
                        locations=[location]
                    )
                else:
                    if location not in self.config_items[key].locations:
                        self.config_items[key].locations.append(location)
    
    def extract_secrets(self, content: str, file_path: str):
        """Extract secret storage usage."""
        patterns = [
            (r'\.get_secret\s*\(\s*["\']([^"\']+)["\']', 'get_secret'),
            (r'config\.get_secret\s*\(\s*["\']([^"\']+)["\']', 'get_secret'),
            (r'\.set_secret\s*\(\s*["\']([^"\']+)["\']', 'set_secret'),
            (r'config\.set_secret\s*\(\s*["\']([^"\']+)["\']', 'set_secret'),
            (r'\.delete_secret\s*\(\s*["\']([^"\']+)["\']', 'delete_secret'),
        ]
        
        for pattern, method in patterns:
            for match in re.finditer(pattern, content):
                key = match.group(1)
                line_num = content[:match.start()].count('\n') + 1
                location = f"{file_path}:{line_num}"
                
                if key not in self.config_items:
                    self.config_items[key] = ConfigItem(
                        key=key,
                        config_type='secret',
                        method=method,
                        locations=[location]
                    )
                else:
                    if location not in self.config_items[key].locations:
                        self.config_items[key].locations.append(location)
    
    def extract_env_vars(self, content: str, file_path: str):
        """Extract environment variable usage."""
        patterns = [
            (r'std::env::var\s*\(\s*["\']([^"\']+)["\']', 'std::env::var'),
            (r'env::var\s*\(\s*["\']([^"\']+)["\']', 'env::var'),
            (r'std::env::set_var\s*\(\s*["\']([^"\']+)["\']', 'std::env::set_var'),
            (r'env::set_var\s*\(\s*["\']([^"\']+)["\']', 'env::set_var'),
        ]
        
        for pattern, method in patterns:
            for match in re.finditer(pattern, content):
                key = match.group(1)
                line_num = content[:match.start()].count('\n') + 1
                location = f"{file_path}:{line_num}"
                
                # Filter out obvious test variables and false positives
                if self.is_valid_env_var(key, content, match.start()):
                    if key not in self.config_items:
                        self.config_items[key] = ConfigItem(
                            key=key,
                            config_type='env_var',
                            method=method,
                            locations=[location]
                        )
                    else:
                        if location not in self.config_items[key].locations:
                            self.config_items[key].locations.append(location)
    
    def is_valid_env_var(self, key: str, content: str, match_pos: int) -> bool:
        """Validate if this is likely a real environment variable."""
        # Skip test-only variables
        test_indicators = ['test_', 'TEST_', 'temp_', 'TEMP_', 'mock_', 'MOCK_']
        if any(key.startswith(indicator) for indicator in test_indicators):
            return False
        
        # Skip very short keys
        if len(key) < 3:
            return False
        
        # Skip common false positives
        false_positives = {
            'OK', 'ERR', 'GET', 'POST', 'PUT', 'DELETE', 'HEAD', 'OPTIONS',
            'JSON', 'YAML', 'XML', 'HTML', 'HTTP', 'HTTPS', 'API', 'URL', 'URI',
            'UTF', 'ASCII', 'DEBUG', 'INFO', 'WARN', 'ERROR', 'TRUE', 'FALSE'
        }
        if key in false_positives:
            return False
        
        # Must be mostly uppercase with underscores or known single-word env vars
        known_single = {'HOME', 'USER', 'PATH', 'LANG', 'TERM', 'SHELL', 'PWD'}
        if not (key.isupper() and ('_' in key or key in known_single)):
            return False
        
        return True
    
    def extract_cli_flags(self, content: str, file_path: str):
        """Extract CLI flags from clap annotations."""
        # Find #[arg(...)] patterns with long flags
        arg_pattern = r'#\[arg\(([^)]+)\)\]'
        for match in re.finditer(arg_pattern, content, re.MULTILINE):
            line_num = content[:match.start()].count('\n') + 1
            location = f"{file_path}:{line_num}"
            arg_content = match.group(1)
            
            # Extract long flag
            long_match = re.search(r'long\s*=\s*["\']([^"\']+)["\']', arg_content)
            if long_match:
                flag_name = f"--{long_match.group(1)}"
                
                # Extract help text
                help_match = re.search(r'help\s*=\s*["\']([^"\']*)["\']', arg_content)
                description = help_match.group(1) if help_match else None
                
                if flag_name not in self.config_items:
                    self.config_items[flag_name] = ConfigItem(
                        key=flag_name,
                        config_type='cli_flag',
                        method='clap_long',
                        locations=[location],
                        description=description
                    )
                else:
                    if location not in self.config_items[flag_name].locations:
                        self.config_items[flag_name].locations.append(location)
            
            # Extract short flag
            short_match = re.search(r'short\s*=\s*["\'](.)["\']', arg_content)
            if short_match:
                flag_name = f"-{short_match.group(1)}"
                
                if flag_name not in self.config_items:
                    self.config_items[flag_name] = ConfigItem(
                        key=flag_name,
                        config_type='cli_flag',
                        method='clap_short',
                        locations=[location]
                    )
                else:
                    if location not in self.config_items[flag_name].locations:
                        self.config_items[flag_name].locations.append(location)
    
    def analyze_file(self, file_path: Path):
        """Analyze a single file for all configuration patterns."""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
        except (UnicodeDecodeError, PermissionError):
            return
        
        relative_path = str(file_path.relative_to(self.root_path))
        
        self.extract_config_params(content, relative_path)
        self.extract_secrets(content, relative_path)
        self.extract_env_vars(content, relative_path)
        self.extract_cli_flags(content, relative_path)
    
    def run_analysis(self):
        """Run the complete analysis."""
        rust_files = self.find_rust_files()
        print(f"Analyzing {len(rust_files)} Rust files...")
        
        for i, file_path in enumerate(rust_files):
            if i % 20 == 0:
                print(f"Progress: {i}/{len(rust_files)} files")
            self.analyze_file(file_path)
        
        print(f"Analysis complete. Found {len(self.config_items)} unique configuration items.")
    
    def generate_summary_report(self) -> str:
        """Generate a focused summary report."""
        by_type = defaultdict(list)
        for item in self.config_items.values():
            by_type[item.config_type].append(item)
        
        report = "# Focused Goose Configuration Summary\n\n"
        report += f"**Total Unique Configuration Items:** {len(self.config_items)}\n\n"
        
        for config_type in ['config_param', 'env_var', 'secret', 'cli_flag']:
            items = sorted(by_type[config_type], key=lambda x: x.key)
            if not items:
                continue
                
            type_names = {
                'config_param': 'Config File Parameters',
                'env_var': 'Environment Variables',
                'secret': 'Secret Storage',
                'cli_flag': 'CLI Flags'
            }
            
            report += f"## {type_names[config_type]} ({len(items)} items)\n\n"
            
            for item in items:
                report += f"### `{item.key}`\n"
                if item.description:
                    report += f"**Description:** {item.description}\n"
                report += f"**Method:** {item.method}\n"
                report += f"**Locations:** {len(item.locations)} usage(s)\n"
                
                # Show first few locations
                for loc in item.locations[:3]:
                    report += f"- {loc}\n"
                if len(item.locations) > 3:
                    report += f"- ... and {len(item.locations) - 3} more\n"
                report += "\n"
        
        return report
    
    def generate_json_export(self) -> dict:
        """Generate JSON export of all configuration items."""
        return {
            'summary': {
                'total_items': len(self.config_items),
                'by_type': {
                    config_type: len([item for item in self.config_items.values() 
                                    if item.config_type == config_type])
                    for config_type in ['config_param', 'env_var', 'secret', 'cli_flag']
                }
            },
            'items': {key: asdict(item) for key, item in self.config_items.items()}
        }

def main():
    parser = argparse.ArgumentParser(description="Find focused configuration usage in Goose")
    parser.add_argument("--root", default=".", help="Root directory to search")
    parser.add_argument("--output", help="Output markdown file")
    parser.add_argument("--json", help="Output JSON file")
    parser.add_argument("--format", choices=['markdown', 'json'], default='markdown')
    
    args = parser.parse_args()
    
    finder = FocusedConfigFinder(args.root)
    finder.run_analysis()
    
    if args.format == 'json' or args.json:
        json_data = finder.generate_json_export()
        json_output = json.dumps(json_data, indent=2)
        
        if args.json:
            with open(args.json, 'w') as f:
                f.write(json_output)
            print(f"JSON export saved to {args.json}")
        else:
            print(json_output)
    
    if args.format == 'markdown' or args.output:
        report = finder.generate_summary_report()
        
        if args.output:
            with open(args.output, 'w') as f:
                f.write(report)
            print(f"Report saved to {args.output}")
        else:
            print(report)
    
    # Print summary
    by_type = defaultdict(int)
    for item in finder.config_items.values():
        by_type[item.config_type] += 1
    
    print(f"\nConfiguration Summary:")
    print(f"  Config File Parameters: {by_type['config_param']}")
    print(f"  Environment Variables: {by_type['env_var']}")
    print(f"  Secret Storage: {by_type['secret']}")
    print(f"  CLI Flags: {by_type['cli_flag']}")
    print(f"  Total: {len(finder.config_items)}")

if __name__ == "__main__":
    main()
