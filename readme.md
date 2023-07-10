# ideapad-toggle-conserve

A tiny tool to toggle battery conservation mode on Lenovo laptops. Designed to be bound to a key press for convenance.

The ideapad driver requires root to write to the conservation mode by default. You'll want to allow your user to write to the file by either:

```sh
IDEAPAD_FILE=/sys/bus/platform/drivers/ideapad_acpi/VPC2004:00/conservation_mode
# setting yourself as owner
chown $(whoami):$(whoami) $IDEAPAD_FILE
# or allowing anyone to write
chmod 646 $IDEAPAD_FILE
```

I've made a convenient systemd script to allow public write on boot up.

```sh
systemctl enable $(realpath ideapad-conservation-chmod.service)
```
