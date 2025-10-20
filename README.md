# Glide
<center>
[![Build](https://github.com/null-proto/glide/actions/workflows/rust.yml/badge.svg)](https://github.com/null-proto/glide/actions/workflows/rust.yml)
<center/>
Glide is a minimal tool for hosting bare Git repositories over HTTP. It’s intended for personal or embedded use cases where a full Git platform is unnecessary.

The implementation uses own HTTP/1.1 server, written from scratch and performance-tuned. It uses `git http-backend` for clone/sync git repo.


### Setup/Run

Create system user of your choice eg. `git`, create directory and assign it as HOME of the system user.

```
useradd git -c "Git user" --home-dir /srv/git --system --create-home
```
To start from other user

```
sudo -u git ./glide --project-root=/srv/git
```


### Contributions
PR and issues are welcome.
