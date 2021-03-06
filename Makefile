# Makefile based on https://github.com/getsentry/symbolic
NAME=griddfs
PYTHON=python3

all: wheel

version.mk: Cargo.toml
	sed -ne 's/^version = "\(.*\)"$$/VERSION=\1/p' $< > $@ && [ -s $@ ] || ($(RM) $@; false)

include version.mk

wheel-install: wheel
	pip install --user -U dist/$(NAME)-$(VERSION)-py2.py3-none-linux_x86_64.whl

wheel:
	$(PYTHON) setup.py bdist_wheel

IMAGE-x86_64=quay.io/pypa/manylinux1_x86_64
IMAGE-i686=quay.io/pypa/manylinux1_i686

wheel-manylinux: wheel-manylinux-x86_64 wheel-manylinux-i686

wheel-manylinux-x86_64 wheel-manylinux-i686: wheel-manylinux-%:
	d=`mktemp --tmpdir -d manylinux.XXXXXX` && ( \
	  git archive HEAD | tar -x -C $$d && \
	docker run --rm -it -v $$d:/work -w /work $(IMAGE-$*) sh manylinux.sh && \
	cp $$d/dist/*.whl -t dist ) ; \
	$(RM) -r $$d

.PHONY: all wheel wheel-manylinux wheel-manylinux-x86_64 wheel-manylinux-i686
