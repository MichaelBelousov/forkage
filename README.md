# Forkage

This is both documentation and a design document (the latter at this stage anyway).
Many things here aren't implemented.

Forkage is an unbiased package manager that encourages forking packages for tweaking, and
searching others' forks.
With the current goal of supporting git and common git hosts.

## Commands

### installing and uninstalling

```sh
fkg install gh:MichaelBelousov/forkage
fkg install github.com:MichaelBelousov/forkage
fkg install github.com:MichaelBelousov/forkage/branch
fkg install MichaelBelousov/forkage # if you choose a default provider
fkg i MichaelBelousov/forkage
# when installing a specific revision, if you already have it installed unambiguously you can drop the
# leading qualifier
fkg i forkage@v1.0
fkg i forkage@v1.0 # use git tag
fkg i forkage@0adf18 # use git commit
fkg uninstall forkage # will work if unambiguously installed
fkg remove forkage
```

### updating

```sh
fkg update gh:MichaelBelousov/forkage
fkg upgrade forkage # will work if unambigous
fkg merge forkage --ref origin/branch
fkg merge forkage --ref gh:Alice/forkage/branch
fkg rebase forkage
fkg merge forkage --vcs-args rebase --interactive
```

### config

```sh
fkg config set default-provider github.com
fkg config get default-provider
fkg config set search-providers github.com,gitlab.com,src.ht
fkg config list
fkg provider github login # opens a github integration page for authorizing a token
```

### forking

```sh
fkg fork MichaelBelousov/forkage # will create a fork for your user using the github API
fkg fork --as myorg MichaelBelousov/forkage # will create a fork in an organization that you have authorized
```

### search

```sh
# at install time you choose default searched providers
fkg search forkage
# but you can specify
fkg search gh:forkage
# list all forks for each result
fkg search --forks gh:forkage
```

### inspecting

```sh
# look at installed packages if they exist
fkg info forkage
# can also show extended information of remote package
fkg info gh:MichaelBelousov/forkage
fkg list # list all installed packages
```

### creating

```sh
# this is basically like creating a new repo on a provider and git cloning it
fkg init Name # create a package (repo) for your user on your default provider
fkg init gl:Org/Name # create a package under a specific provider and organization
```

## Layout

Packages are installed to the nearest `forkage` directory, ascending from the current directory's path.
If there are none, a new `forkage` directory is created in the current directory
When executing commands, if there is non.

A config file will be searched for in order:
- `.forkageconfig` next to the nearest `forkage` directory ascending the current directory's path.
- `$HOME/.forkageconfig`
- `/etc/forkageconfig`

An example git provider URL of github.com/MichaelBelousov/forkage would be installed as:

```sh
./forkage/github.com/MichaelBelousov/forkage
```

The URL's path delimiters are translated to file system path delimiters.

## Dependencies

Forkage does not do dependencies. Instead, authors are encouraged to mark dependencies in ways particular
to the content they are hosting.

For instance, a package containing JavaScript source of an npm package could configure a workspace manager
like [`pnpm`](https://pnpm.io) to include all `forkage/*` directories in the workspace so that they participate
in workspace dependency resolution and even build commands.

Some projects may choose to be encapsulate their dependencies by adding e.g. git submodules to their repository.

There may come a day when we see that it would be useful for a forkage package to say that installing it should
install others. We'll see. For now use a git submodule or subtree, etc.
