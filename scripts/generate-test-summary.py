#!/usr/bin/env python3
"""
Generate test-summary.md from integration test files.
Scans all .md files in tests/integration/ and extracts their status circles.
"""

import os
import re
from pathlib import Path
from collections import defaultdict

def extract_test_status(file_path):
    """Extract the status circle from a test file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Look for "Status: 🟢", "Status: 🔴", or "Status: 🟡"
        status_match = re.search(r'Status: ([🟢🔴🟡])', content)
        if status_match:
            return status_match.group(1)
        
        # Fallback: look for old format "# Program 🟢", "# Program 🔴", or "# Program 🟡"
        match = re.search(r'# Program ([🟢🔴🟡])', content)
        if match:
            return match.group(1)
        
        # Fallback: look for the circles anywhere in the first few lines
        lines = content.split('\n')[:10]
        for line in lines:
            if '🟢' in line:
                return '🟢'
            elif '🔴' in line:
                return '🔴'
            elif '🟡' in line:
                return '🟡'
                
        raise ValueError(f"No status found in {file_path}")
    except Exception as e:
        raise RuntimeError(f"Error reading {file_path}: {e}") from e

def generate_test_summary():
    """Generate the test summary."""
    project_root = Path(__file__).parent.parent
    tests_dir = project_root / "tests" / "integration"
    
    if not tests_dir.exists():
        print(f"Tests directory not found: {tests_dir}")
        return
    
    # Find all .md files
    test_files = list(tests_dir.glob("**/*.md"))
    
    # Group by category (directory)
    categories = defaultdict(list)
    
    for file_path in test_files:
        relative_path = file_path.relative_to(tests_dir)
        
        # Get category (parent directory or "root" for top-level files)
        if relative_path.parent.name == "integration":
            category = "root"
        else:
            category = relative_path.parent.name
        
        # Get test name (filename without .md)
        test_name = relative_path.stem
        
        # Extract status
        status = extract_test_status(file_path)
        
        # Create relative path for markdown link
        link_path = str(relative_path)
        
        categories[category].append((test_name, status, link_path))
    
    # Sort categories and tests within each category
    sorted_categories = sorted(categories.items())
    for category, tests in sorted_categories:
        tests.sort(key=lambda x: x[0])
    
    # Generate summary content
    summary_lines = [
        "# Test Summary",
        "",
        "Auto-generated test status summary from integration tests.",
        ""
    ]
    
    for category, tests in sorted_categories:
        if category == "root":
            section_title = "Root Tests"
        else:
            section_title = f"{category.title()} Tests"
        
        # Count statuses for this category
        category_passing = sum(1 for _, status, _ in tests if status == '🟢')
        category_failing = sum(1 for _, status, _ in tests if status == '🔴')
        category_no_assert = sum(1 for _, status, _ in tests if status == '🟡')
        category_total = len(tests)
        
        # Build summary
        summary_parts = []
        if category_passing > 0:
            summary_parts.append(f"{category_passing} 🟢")
        if category_failing > 0:
            summary_parts.append(f"{category_failing} 🔴")
        if category_no_assert > 0:
            summary_parts.append(f"{category_no_assert} 🟡")
        
        summary_text = " ".join(summary_parts) if summary_parts else f"{category_total} tests"
        
        summary_lines.append(f"<details>")
        summary_lines.append(f"<summary>{section_title}: {summary_text}</summary>")
        summary_lines.append("")
        
        for test_name, status, link_path in tests:
            summary_lines.append(f"- [{test_name}](tests/integration/{link_path}): {status}")
        
        summary_lines.append("")
        summary_lines.append("</details>")
        summary_lines.append("")
    
    # Add statistics
    total_tests = sum(len(tests) for _, tests in sorted_categories)
    passing_tests = sum(1 for _, tests in sorted_categories for _, status, _ in tests if status == '🟢')
    failing_tests = sum(1 for _, tests in sorted_categories for _, status, _ in tests if status == '🔴')
    no_assert_tests = sum(1 for _, tests in sorted_categories for _, status, _ in tests if status == '🟡')
    
    stats_lines = [
        "## Statistics",
        "",
        f"- Total tests: {total_tests}",
        f"- Passing: {passing_tests} 🟢",
        f"- Failing: {failing_tests} 🔴",
    ]
    
    # Only include no-assert if they are > 0
    if no_assert_tests > 0:
        stats_lines.append(f"- No asserts: {no_assert_tests} 🟡")
    
    stats_lines.extend([
        "",
        f"**Pass rate: {passing_tests/total_tests*100:.1f}%**" if total_tests > 0 else "**Pass rate: N/A**"
    ])
    
    summary_lines.extend(stats_lines)
    
    # Write summary file
    summary_path = project_root / "test-summary.md"
    with open(summary_path, 'w', encoding='utf-8') as f:
        f.write('\n'.join(summary_lines))
    
    print(f"Generated test summary: {summary_path}")
    print(f"Total: {total_tests}, Passing: {passing_tests}, Failing: {failing_tests}")

if __name__ == "__main__":
    generate_test_summary()