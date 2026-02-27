# RTD


Random Things Downloader

A simple CLI to download random things from a repository.

RTD is powered by a federated network of static-hosted repos. Anyone can host one, anyone can link to others.


## Quickstart

Here is how to add a repository:
```bash
rtd repository add <URL>
```
And to remove a repository:
```bash
rtd repository remove <URL>
```
Here is how to list all available repositories in your configuration:
```bash
rtd repository list
```


Before you can pull, You need to synchronize the repository using:
```bash
rtd repository sync
```
Here is how to pull from a repository:
```bash
rtd pull
```
The pull subcommand have a flag called `max-depth` which will set the maximum depth for a nested repository.

## How it works

First, The user will add a repository and then synchronize their local repository and the server repository. When they pull, it will search for a set random repository and then will open and read it, After that, rtd will choose a random line which will be either a reward or a url to another repository.

If a repository contain a url to another repository, It will get the another repository and then it will search for a random line, And it will repeat.

The first repository in this project is: https://gist.githubusercontent.com/Bimasakti1024/c05d38ef8b93b8fd7dfb861977dd48e7/raw/d3f97fab766d1749e6651b23d5a03b6876aecdde/rtd-repo.txt



## Creating your own repository

Creating your own repository is really simple, You just need:

1. An internet connection.

2. A working http or https server.

3. A text editor.


What you need to do is to run the http/https server and create a text file that are accessible through the http/https server. Here is an example of the repository:

```
# This is a reward
https://pastebin.com/raw/sqg8Ay0d
# If you want to make a nested repository
# that links your repository with others,
# you can use the Nested tag, like this:
Nested https://gist.githubusercontent.com/Bimasakti1024/c05d38ef8b93b8fd7dfb861977dd48e7/raw/d3f97fab766d1749e6651b23d5a03b6876aecdde/rtd-repo.txt
```

## License

MIT

