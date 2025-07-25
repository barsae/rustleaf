#!/usr/bin/env python3
"""
Generate test-summary.html from integration test files.
Scans all .md files in tests/integration/ and extracts their status circles.
"""

import os
import re
from datetime import datetime
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

def generate_html_template():
    """Generate the HTML template with CSS."""
    return '''<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Test Summary</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif;
            max-width: 900px;
            margin: 40px auto;
            padding: 20px;
            line-height: 1.6;
            background-color: #0d1117;
            color: #f0f6fc;
        }
        h1 {
            color: #58a6ff;
            border-bottom: 1px solid #30363d;
            padding-bottom: 10px;
            font-weight: 700;
            font-size: 2em;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        .main-title {
            flex: 1;
        }
        .main-summary {
            font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
            color: #8b949e;
            font-size: 0.6em;
            font-weight: normal;
        }
        .timestamp {
            font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
            color: #6e7681;
            font-size: 0.85em;
            font-weight: normal;
            margin-top: 10px;
            text-align: center;
        }
        h2 {
            color: #f0f6fc;
            margin-top: 30px;
        }
        details {
            margin: 10px 0;
            border: 1px solid #30363d;
            border-radius: 6px;
            padding: 10px;
            background-color: #161b22;
        }
        summary {
            font-weight: 600;
            cursor: pointer;
            padding: 5px;
            color: #58a6ff;
            display: flex;
            justify-content: space-between;
            align-items: center;
            position: relative;
        }
        summary::before {
            content: '▶';
            margin-right: 8px;
            transition: transform 0.2s ease;
        }
        details[open] summary::before {
            transform: rotate(90deg);
        }
        summary:hover {
            background-color: #21262d;
        }
        .section-title {
            flex: 1;
        }
        .section-summary {
            font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
            font-size: 0.9em;
            color: #8b949e;
            text-align: right;
        }
        ul {
            margin: 10px 0;
            padding-left: 20px;
        }
        li {
            margin: 5px 0;
        }
        a {
            color: #58a6ff;
            text-decoration: none;
        }
        a:hover {
            text-decoration: underline;
        }
        .stats {
            background-color: #161b22;
            border: 1px solid #30363d;
            border-radius: 6px;
            padding: 15px;
            margin-top: 20px;
        }
        .pass-rate {
            font-size: 1.1em;
            margin-top: 10px;
        }
    </style>
</head>
<body>'''

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
        
        # Create relative path for HTML link
        link_path = str(relative_path)
        
        categories[category].append((test_name, status, link_path))
    
    # Sort categories and tests within each category
    sorted_categories = sorted(categories.items())
    for category, tests in sorted_categories:
        tests.sort(key=lambda x: x[0])
    
    # Calculate overall statistics
    total_tests = sum(len(tests) for _, tests in sorted_categories)
    passing_tests = sum(1 for _, tests in sorted_categories for _, status, _ in tests if status == '🟢')
    failing_tests = sum(1 for _, tests in sorted_categories for _, status, _ in tests if status == '🔴')
    no_assert_tests = sum(1 for _, tests in sorted_categories for _, status, _ in tests if status == '🟡')
    
    # Build overall summary for header
    header_summary_parts = []
    if passing_tests > 0:
        header_summary_parts.append(f"{passing_tests} 🟢")
    if failing_tests > 0:
        header_summary_parts.append(f"{failing_tests} 🔴")
    if no_assert_tests > 0:
        header_summary_parts.append(f"{no_assert_tests} 🟡")
    
    header_summary = " ".join(header_summary_parts)
    
    # Get current timestamp
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    
    # Start building HTML
    html_content = generate_html_template()
    
    # Add header with summary and timestamp
    html_content += f'''
    <h1>
        <span class="main-title">Test Summary</span>
        <span class="main-summary">{header_summary}</span>
    </h1>
    <div class="timestamp">{timestamp}</div>

'''
    
    # Add collapsible sections
    for category, tests in sorted_categories:
        if category == "root":
            section_title = "Root"
        else:
            section_title = category.replace('_', ' ').title()
        
        # Count statuses for this category
        category_passing = sum(1 for _, status, _ in tests if status == '🟢')
        category_failing = sum(1 for _, status, _ in tests if status == '🔴')
        category_no_assert = sum(1 for _, status, _ in tests if status == '🟡')
        
        # Build category summary (omit zeros)
        summary_parts = []
        if category_passing > 0:
            summary_parts.append(f"{category_passing} 🟢")
        if category_failing > 0:
            summary_parts.append(f"{category_failing} 🔴")
        if category_no_assert > 0:
            summary_parts.append(f"{category_no_assert} 🟡")
        
        summary_text = " ".join(summary_parts)
        
        html_content += f'''    <details>
        <summary>
            <span class="section-title">{section_title}</span>
            <span class="section-summary">{summary_text}</span>
        </summary>
        <ul>
'''
        
        for test_name, status, link_path in tests:
            # Create absolute path for VSCode URL
            absolute_path = project_root / "tests" / "integration" / link_path
            vscode_url = f"vscode://file/{absolute_path}"
            html_content += f'            <li><a href="{vscode_url}">{test_name}</a>: {status}</li>\n'
        
        html_content += '''        </ul>
    </details>

'''
    
    # Close HTML
    html_content += '''</body>
</html>'''
    
    # Write summary file
    summary_path = project_root / "test-summary.html"
    with open(summary_path, 'w', encoding='utf-8') as f:
        f.write(html_content)
    
    print(f"Generated test summary: {summary_path}")
    print(f"Total: {total_tests}, Passing: {passing_tests}, Failing: {failing_tests}")

if __name__ == "__main__":
    generate_test_summary()