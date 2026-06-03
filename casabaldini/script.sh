#!/bin/bash

# Passo 1: build senza bundle
dx build --package CasaBaldini --release --android --target aarch64-linux-android

# Passo 2: sovrascrive network_security_config.xml
set NETWORK_CONFIG="gen/android/app/src/main/res/xml/network_security_config.xml"

cat > "$NETWORK_CONFIG" << 'EOF'
<?xml version="1.0" encoding="utf-8"?>
<network-security-config>
    <base-config cleartextTrafficPermitted="false">
        <trust-anchors>
            <certificates src="system"/>
        </trust-anchors>
    </base-config>
    <domain-config cleartextTrafficPermitted="true">
        <domain includeSubdomains="true">127.0.0.1</domain>
    </domain-config>
</network-security-config>
EOF

echo "network_security_config.xml aggiornato"

# Passo 3: bundle finale
dx bundle --package CasaBaldini --release --android --target aarch64-linux-android
