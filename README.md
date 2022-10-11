# FormulaHybrid_MP6
Code repository for MSOE SAE Formula Hybrid MP6.

## Notes for team
If you're doing any specific project work, please work on your own branch and open a PR when everything's ready to go. 

```sh
$ git branch <name>
$ git checkout <name>
```
1. Install Rust 
	- Windows: install rust-init.exe from website
	- WSL: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Update toolchain: `rustup update`
2.1 Install `rust-analyzer`
3. Install cargo-flash
	- `cargo install cargo-flash `
4. Install compile target
	- `rustup target add thumbv7em-none-eabihf`
5. Flash board
	- Plug in board
	- `cargo-flash --chip STM32F411RETx --release`
	- Alternatively, `./build_unix` for Linux/WSL or `./build` for Windows

## Little Primer on Git
`git` is a version control system that concurrent modification on files and directories by tracking changes to a filesystem. 
A folder that we use with `git` is called a *repository*. 
We're using what's called a *remote repository*, where a copy of the repository is treated as the master copy and is stored away from your machine. 
We're using GitHub as a host for our remote repository. 

After installing `git` on your machine, you can *clone* a remote repository on to your machine:
```
$ git clone https://github.com/Will-Dale-19/FormulaHybrid_MP6
```

This will create a local copy of the repository as a folder. Changes made on your local copy are *not* automatically propogated to the remote repository,
and changes to the remote repository are *not* automatically propogated to your local copy. 

Instead, when we have some changes we want to *push* to the remote repository, we need to perform the following steps. 
1. Stage the desired changes. This is performed on a per-file basis. Staging the changes tells `git` which things we are done with and what we want to keep on our copy. 
2. Commit the staged changes. This is confirming to `git` that the changes we staged are *permanent*, and we want to keep them. 
`git` stores every commit with an ID corresponding to the state of the repository at the time of commit. This allows us to track when changes are made, as well as view and revert to previous states. 
3. Push changes to the remote repository: This will propogate our commit to the remote repository, matching the current state of the remote with the current state of your local copy. 

You can perform all three steps with the following commands
```sh
$ git add .
$ git commit -m"<some message"
$ git push
```
The `add <files>` command stages changes for a commit. The `.` is shorthand for `all`, which will stage all changes made in the entire working directory. 

The `commit` command commits the staged changes to a unique ID. If we don't pass the `-m` flag to the command, we'll get brought to a `vi` text editor to make a message. If you don't know how to use `vi`, it's easier to pass the message in with the command rather than make the message in an additional step. 

Finally, the `push` command pushes the current commit to the remote repository. 

If we want to integrate changes from the remote repo to our local copy, we need to *pull* down and merge the changes. 
This is performed by executing `$ git pull`. 

If we're working on the main branch, it's a good idea to always `pull` before you start working, and at the very least `commit` when you're done. 

In a production environment, it's even safer to work on `branches`. 
By default, every `git` repository has a `main` branch. If we're working on a feature or large change, best practice is to `checkout` a branch dedicated to that change. Working on a branch makes `git` track changes separately from the main branch, meaning you can write and change the repository however you want without impacting `main`. 

When the feature or whatever is complete, we don't ever want to manually `merge` to the main branch. Instead, we open a *Pull Request* (PR) in GitHub.
This will notify other contributors to the repository, and allows the changes to be reviewed next to the main copy before being merged in. 

You can create and checkout a branch using the following:
```sh
git branch <name>
git checkout <name>
```
We can shorthand this by executing:
```sh
git checkout -b <name>
```
We can also see which branch we are on by calling `git branch` with no arguments, and available branches by calling `git checkout` without any arguments.

If the command line is a bit scary to use, most modern editors have `git` support built in. VSCode has `git` support under the Version Control tab, and the GitLens extension is very good for adding extra functionality. 

## Relavant Documentation
- [cortex_m crate](https://docs.rs/cortex-m/)
- [cortex_m runtime crate](https://docs.rs/cortex-m-rt/latest/cortex_m_rt/)
- [stm32f4 hal](https://docs.rs/stm32f4xx-hal)
- [stm32f7 hal](https://docs.rs/stm32f7xx-hal)
- [panic_halt](https://docs.rs/panic-halt/)
- [svd2rust](https://docs.rs/svd2rust/0.24.1/svd2rust/#peripheral-api)
