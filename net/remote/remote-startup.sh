#!/bin/bash
#
# Runs at boot on each instance as root
#
# TODO: Make the following a requirement of the Instance image
#       instead of a manual install?

systemctl disable apt-daily.service # disable run when system boot
systemctl disable apt-daily.timer   # disable timer run
apt-get --assume-yes install rsync libssl-dev

cat > /etc/rsyncd.conf <<-EOF
[config]
path = /var/snap/solana/current/config
hosts allow = *
read only = true
EOF

systemctl enable rsync
systemctl start rsync