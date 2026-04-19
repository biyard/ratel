MAESTRO ?= $(HOME)/.maestro/bin/maestro
DEVICE ?=

# If DEVICE is set (e.g. emulator-5554), pass --device flag.
# Otherwise maestro auto-picks the connected device.
DEVICE_FLAG := $(if $(DEVICE),--device $(DEVICE),)

.PHONY: test test-login test-post test-settings test-error test-logout

test:
	$(MAESTRO) test $(DEVICE_FLAG) tests/

test-login:
	$(MAESTRO) test $(DEVICE_FLAG) tests/01-login.yaml

test-post:
	$(MAESTRO) test $(DEVICE_FLAG) tests/02-create-post.yaml

test-settings:
	$(MAESTRO) test $(DEVICE_FLAG) tests/03-settings-theme-language.yaml

test-error:
	$(MAESTRO) test $(DEVICE_FLAG) tests/04-error-page-recovery.yaml

test-logout:
	$(MAESTRO) test $(DEVICE_FLAG) tests/05-logout.yaml
