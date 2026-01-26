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

# Update README with CU summary
{
	echo "<!-- CU_SUMMARY_START -->"
	echo "| Instruction | Best | Avg | Worst | Count |"
	echo "|-------------|------|-----|-------|-------|"
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
	echo "<!-- CU_SUMMARY_END -->"
} > .cus/readme_section.tmp

# Replace section in README
awk '
/<!-- CU_SUMMARY_START -->/ { skip = 1 }
/<!-- CU_SUMMARY_END -->/ { skip = 0; next }
!skip { print }
' README.md > .cus/readme_without_cu.tmp

# Insert new section after first line (title)
head -1 .cus/readme_without_cu.tmp > README.md
cat .cus/readme_section.tmp >> README.md
tail -n +2 .cus/readme_without_cu.tmp >> README.md

rm -f .cus/readme_section.tmp .cus/readme_without_cu.tmp
echo ""
echo "README.md updated with CU summary."
