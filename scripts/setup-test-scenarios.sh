#!/bin/bash

set -e

mkdir -p test-scenarios/{install,remove,build,legacy,versions,cross}

cat > test-scenarios/install/opencli.toml << 'EOF'
[build]
entry_file = "gamemode.pwn"
output_file = "gamemode.amx"
compiler_version = "v3.10.11"

[build.includes]
paths = ["include"]

[build.args]
args = ["-d3", "-O2"]
EOF

echo 'main() {}' > test-scenarios/install/gamemode.pwn

cat > test-scenarios/remove/opencli.toml << 'EOF'
[build]
entry_file = "gamemode.pwn"
output_file = "gamemode.amx"
compiler_version = "v3.10.11"

[build.includes]
paths = ["include"]

[build.args]
args = ["-d3", "-O2"]
EOF

echo 'main() {}' > test-scenarios/remove/gamemode.pwn

cat > test-scenarios/build/opencli.toml << 'EOF'
[build]
entry_file = "gamemodes/gamemode.pwn"
output_file = "gamemodes/gamemode.amx"
compiler_version = "v3.10.11"

[build.includes]
paths = ["include"]

[build.args]
args = ["-d3", "-O2"]

[packages]
"Y-Less/sscanf" = { version = "^2.13.0", target = "components" }
EOF

mkdir -p test-scenarios/build/gamemodes
echo '#include <sscanf2>
main() {
    new str[] = "123 456";
    new a, b;
    sscanf(str, "dd", a, b);
}' > test-scenarios/build/gamemodes/gamemode.pwn

cat > test-scenarios/legacy/opencli.toml << 'EOF'
[build]
entry_file = "gamemode.pwn"
output_file = "gamemode.amx"
compiler_version = "v3.10.11"

[build.includes]
paths = ["include"]

[build.args]
args = ["-d3", "-O2"]
EOF

echo 'main() {}' > test-scenarios/legacy/gamemode.pwn

cat > test-scenarios/versions/opencli.toml << 'EOF'
[build]
entry_file = "gamemode.pwn"
output_file = "gamemode.amx"
compiler_version = "v3.10.11"

[build.includes]
paths = ["include"]

[build.args]
args = ["-d3", "-O2"]
EOF

echo 'main() {}' > test-scenarios/versions/gamemode.pwn

cp -r test-scenarios/install/* test-scenarios/cross/

echo "Test scenarios created successfully!"
echo "Run: docker-compose -f docker-compose.test.yml up"
