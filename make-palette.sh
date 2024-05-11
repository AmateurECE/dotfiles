#!/bin/bash

if [[ -z "$1" ]]; then
	printf >&2 "Usage: $0 palette.txt\n"
	exit 1
fi

into_rgb() {
	printf 'rgb(%s,%s,%s)' $((16#${1:1:2})) $((16#${1:3:2})) $((16#${1:5:2}))
}

join_by() {
	local d=${1-} f=${2-}
	if shift 2; then
		printf %s "$f" "${@/#/$d}"
	fi
}

colors=($(while IFS= read -r line; do
	printf ' xc:%s' "$(into_rgb $line)"
done <$1))

convert -size 60x60 $(join_by " " "${colors[@]}") +append ${1%%.txt}.jpg
