#!/usr/bin/env python3
"""
Convert cargo bench output to custom JSON format with million links/second metric.

For tree benchmarks, we want to show throughput (M links/sec) rather than
time (ns/iter), as it's more meaningful and readable for understanding performance.
Values like "115.08 M links/sec" are much easier to read than "115076784 links/sec".
"""

import json
import re
import sys


def parse_bench_output(output_file):
    """Parse cargo bench output and extract benchmark results."""
    with open(output_file, 'r') as f:
        content = f.read()

    # Pattern to match benchmark lines:
    # test sbt_insert_100 ... bench:       2,715.35 ns/iter (+/- 958.21)
    pattern = r'test\s+(\w+)\s+\.\.\.\s+bench:\s+([\d,]+(?:\.\d+)?)\s+ns/iter\s+\(\+/-\s+([\d,]+(?:\.\d+)?)\)'

    results = []
    for match in re.finditer(pattern, content):
        bench_name = match.group(1)
        time_ns = float(match.group(2).replace(',', ''))
        variance = float(match.group(3).replace(',', ''))

        # Extract the number of operations from benchmark name
        # sbt_insert_100 -> 100 links
        # sbt_insert_1000 -> 1000 links
        # sbt_insert_10000 -> 10000 links
        # sbt_insert_search_100 -> 100 links (insert) + 100 links (search) = 200 operations
        # sbt_full_cycle_100 -> 100 links (insert) + 100 links (remove) = 200 operations

        num_match = re.search(r'_(\d+)$', bench_name)
        if not num_match:
            continue

        n = int(num_match.group(1))

        # Determine the number of operations based on benchmark type
        if 'insert_search' in bench_name:
            # Insert n elements, then search n elements
            num_operations = n + n
        elif 'full_cycle' in bench_name:
            # Insert n elements, then remove n elements
            num_operations = n + n
        else:
            # Just insert n elements
            num_operations = n

        # Calculate links/second (throughput)
        # time_ns is time for num_operations operations
        # links/second = num_operations / (time_ns / 1e9)
        time_seconds = time_ns / 1e9
        links_per_second = num_operations / time_seconds

        # Convert to million links per second for better readability
        million_links_per_second = links_per_second / 1_000_000

        # Calculate variance in links/second
        # If time varies by ± variance_ns, then throughput varies inversely
        time_low = (time_ns - variance) / 1e9
        time_high = (time_ns + variance) / 1e9
        links_per_second_high = num_operations / time_low if time_low > 0 else links_per_second
        links_per_second_low = num_operations / time_high if time_high > 0 else links_per_second

        # Use the average of the differences as the range (in millions)
        range_value = (links_per_second_high - links_per_second_low) / 2 / 1_000_000

        # Create a more descriptive name for the chart
        # Group benchmarks by operation type
        if 'insert_search' in bench_name:
            bench_type = 'Insert + Search'
        elif 'full_cycle' in bench_name:
            bench_type = 'Insert + Remove'
        elif 'insert' in bench_name:
            bench_type = 'Insert Only'
        else:
            bench_type = 'Unknown'

        chart_name = f'{bench_type} ({n} elements)'

        results.append({
            'name': chart_name,
            'unit': 'M links/sec',
            'value': round(million_links_per_second, 2),
            'range': f'± {round(range_value, 2)}',
            'extra': f'{num_operations} operations in {time_ns:.2f} ns/iter'
        })

    return results


def main():
    if len(sys.argv) != 3:
        print(f"Usage: {sys.argv[0]} <input_file> <output_file>")
        sys.exit(1)

    input_file = sys.argv[1]
    output_file = sys.argv[2]

    results = parse_bench_output(input_file)

    with open(output_file, 'w') as f:
        json.dump(results, f, indent=2)

    print(f"Converted {len(results)} benchmarks to {output_file}")
    print(f"Results: {json.dumps(results, indent=2)}")


if __name__ == '__main__':
    main()
