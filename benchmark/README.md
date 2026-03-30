### 1. TC1

```bash
    TARGET=tc1 make prepare
    TARGET=tc1 make ec2.run
```

### 2. TC2

```bash
    TARGET=tc2 make prepare
    TARGET=tc2 make ec2.run
```

### 3. TC3

```bash
    TARGET=tc3 make prepare
    TARGET=tc3 make ec2.run
```

### 4. TC4

```bash
    TARGET=tc4 make ec2.run
```

### 5. TC5

```bash
    TARGET=tc5 make ec2.run
```

`tc5` uses JMeter properties, so override the target space at runtime:

```bash
    TARGET=tc5 SPACE_ID=<space-id> make ec2.run
```

You can also run `tc5` locally if JMeter is installed:

```bash
    TARGET=tc5 SPACE_ID=<space-id> make local.run
```

For local servers, set host, port, and protocol separately:

```bash
    TARGET=tc5 SPACE_ID=<space-id> SPACE_PAGE=overview WEB_HOST=localhost WEB_PORT=8000 WEB_PROTOCOL=http make local.run
```

Or use the helper script for the default 4,000-user local run:

```bash
    cd benchmark/tc5
    SPACE_ID=<space-id> ./run-local-4000.sh
```
