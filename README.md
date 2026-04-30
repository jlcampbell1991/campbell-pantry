# Setting Up systemd
## Create systemd service
`sudo nano /etc/systemd/system/campbell-pantry.service`
```
[Unit]
Description=campbell-pantry

[Service]
Type=simple
User=jcampbell
ExecStart=bash /home/jcampbell/rust/campbell-pantry/start.sh

[Install]
WantedBy=multi-user.target
```
`sudo systemctl enable --now campbell-pantry`
`sudo journalctl -xfu campbell-pantry`