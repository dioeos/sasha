#!/usr/bin/env bash
set -euo pipefail
systemctl --user stop sasha.service
journalctl --user -u sasha.service -n 5
