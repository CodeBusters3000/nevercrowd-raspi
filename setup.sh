#!/bin/bash

SCRIPT_NAME="overcrowd-raspi"
DEST_PATH="/usr/local/bin/$SCRIPT_NAME"
SERVICE_NAME="overcrowd.service"
SERVICE_PATH="/etc/systemd/system/$SERVICE_NAME"
ARGS="$*"

sudo cp "$SCRIPT_NAME" "$DEST_PATH"
sudo chmod +x "$DEST_PATH"

sudo bash -c "cat > $SERVICE_PATH" <<EOF
[Unit]
Description=Overcrowd Startup Script
After=network-online.target bluetooth.target
Wants=network-online.target bluetooth.target
Requires=network-online.target bluetooth.target

[Service]
ExecStart=$DEST_PATH $ARGS
Type=simple
RemainAfterExit=yes

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable $SERVICE_NAME
