%:
	if [ -e inputs/day$@ ]; then \
	    cargo run --bin day$@ < inputs/day$@; \
	else \
	    cargo run --bin day$@; \
	fi
