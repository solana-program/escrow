#!/usr/bin/env bash
set -e

mkdir -p .cus
rm -f .cus/results.txt

# Run tests sequentially to avoid race conditions when writing to .cus/results.txt
CU_TRACKING=1 cargo test -p tests-escrow-program -- --test-threads=1

echo ""
echo "╔═══════════════════════════════════════════════════════════════════════════════════╗"
echo "║                            Compute Units Summary                                  ║"
echo "╠══════════════════════════════════════╤═════════╤═════════╤═════════╤═════════════╣"
echo "║ Instruction                          │    Best │     Avg │   Worst │ Count       ║"
echo "╠══════════════════════════════════════╪═════════╪═════════╪═════════╪═════════════╣"
if [ -f .cus/results.txt ]; then
	awk -F',' '
	{
		name = $1
		cus = $2
		count[name]++
		sum[name] += cus
		if (!(name in min) || cus < min[name]) min[name] = cus
		if (!(name in max) || cus > max[name]) max[name] = cus
	}
	END {
		for (name in count) {
			avg = int(sum[name] / count[name])
			printf "║ %-36s │ %7d │ %7d │ %7d │ %7d     ║\n", name, min[name], avg, max[name], count[name]
		}
	}' .cus/results.txt | sort
fi
echo "╚═══════════════════════════════════════════════════════════════════════════════════╝"

# Generate the new CU section
{
	echo "<!-- CU_SUMMARY_START -->"
	echo ""
	echo "| Instruction | Best | Avg | Worst | Count |"
	echo "| ----------- | ---- | --- | ----- | ----- |"
	awk -F',' '
	{
		name = $1
		cus = $2
		count[name]++
		sum[name] += cus
		if (!(name in min) || cus < min[name]) min[name] = cus
		if (!(name in max) || cus > max[name]) max[name] = cus
	}
	END {
		for (name in count) {
			avg = int(sum[name] / count[name])
			printf "| %s | %d | %d | %d | %d |\n", name, min[name], avg, max[name], count[name]
		}
	}' .cus/results.txt | sort
	echo ""
	echo "<!-- CU_SUMMARY_END -->"
} > .cus/cu_section.tmp

# Remove old CU section from docs/CU_BENCHMARKS.md (everything between markers, inclusive)
awk '
/<!-- CU_SUMMARY_START -->/ { skip = 1; next }
/<!-- CU_SUMMARY_END -->/ { skip = 0; next }
!skip { print }
' docs/CU_BENCHMARKS.md > .cus/benchmarks_without_cu.tmp

# Find line number containing "CU_TRACKING" and insert new section after it
line_num=$(grep -n "CU_TRACKING" .cus/benchmarks_without_cu.tmp | head -1 | cut -d: -f1)

if [ -n "$line_num" ]; then
	head -n "$line_num" .cus/benchmarks_without_cu.tmp > docs/CU_BENCHMARKS.md
	echo "" >> docs/CU_BENCHMARKS.md
	cat .cus/cu_section.tmp >> docs/CU_BENCHMARKS.md
	tail -n +"$((line_num + 1))" .cus/benchmarks_without_cu.tmp >> docs/CU_BENCHMARKS.md
else
	echo "Warning: Could not find CU_TRACKING marker, appending to end"
	cat .cus/benchmarks_without_cu.tmp > docs/CU_BENCHMARKS.md
	cat .cus/cu_section.tmp >> docs/CU_BENCHMARKS.md
fi

rm -f .cus/cu_section.tmp .cus/benchmarks_without_cu.tmp
echo ""
echo "docs/CU_BENCHMARKS.md updated with CU summary."
