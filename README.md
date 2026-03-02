[![release](https://img.shields.io/badge/v0.0.1-a6e3a1?style=for-the-badge&labelColor=1e1e2e&logoColor=a6e3a1&label=release)](https://github.com/AndyLocks/swhook/releases/tag/swhook-v0.0.1)
[![gpl](https://img.shields.io/badge/gpl-f9e2af?style=for-the-badge&label=license&labelColor=1e1e2e)](https://github.com/AndyLocks/swhook/blob/master/LICENSE)
[![gpl](https://img.shields.io/badge/AUR-89b4fa?style=for-the-badge&labelColor=1e1e2e&logo=archlinux&logoColor=cdd6f4)](https://aur.archlinux.org/packages/swhook)

A minimalistic **Webhook server**.

# Quick Start

Create a test directory containing an executable bash script file:

```bash
mkdir ~/swhook-test
cd ~/swhook-test
touch hello_world
$EDITOR hello_world
```

with the following content:

```bash
#!/bin/bash

echo "All arguments: $@"
if read -t 0 _
then
  read -r stdin 
  echo "Stdin: $stdin"
fi
```

and make it executable:

```bash
chmod +x hello_world
```

Edit file `/etc/swhook.conf`:

```toml
[server]
port = 3225
host = "127.0.0.1"

[hooks]
hello_world = "/path/to/your/home/directory/swhook-test/hello_world"
```

And then invoke hook:

```bash
curl -X POST -d 'Hello World!' "http://127.0.0.1:3225/hello_world?1=arg1&3=arg3&2=arg2"
```

You will see the following log from server:

```
Executing ["/path/to/your/home/directory/swhook-test/hello_world"]...
["/path/to/your/home/directory/swhook-test/hello_world"] stdout:
All arguments: arg1 arg2 arg3
Stdin: Hello World!
["/path/to/your/home/directory/swhook-test/hello_world"] exited with exit code: 0
```

# Commands

## `server`

Starts the server.

```bash
$ swhook server
Listening to unix socket [/tmp/swhook.sock]...
Listening on http://127.0.0.1:3225
```

## `stop`

Sends a stop signal via UNIX socket (`/tmp/swhook.sock`).

```bash
$ swhook stop
Sending stop request to /tmp/swhook.sock...
```

## `reload`

Sends a reload signal via UNIX socket (`/tmp/swhook.sock`), which reloads the **config file**. This does not affect the listening server port. A full reload is required to change the port.

```bash
$ swhook reload
Sending reload request to /tmp/swhook.sock...
```

---

<p align="center">
	<a href="https://github.com/AndyLocks/swhook?tab=GPL-3.0-1-ov-file#readme"><img src="https://img.shields.io/badge/GPL-cba6f7?style=for-the-badge&label=license&labelColor=1e1e2e"></a>
</p>
