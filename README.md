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
fkg i forkage@v1.0 # use git tag
fkg i forkage@0adf18 # use git commit
# if you somehow need two different versions of one package, use --unique installs
fkg i --unique MichaelBelousov@v2.0 # includes the tag in the package directory's name
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
# diff our version vs some branch
fkg diff forkage @master # can use unqualified branch/ref
```

### creating

```sh
# this is basically like creating a new repo on a provider and git cloning it
fkg init Name # create a package (repo) for your user on your default provider
fkg init gl:Org/Name # create a package under a specific provider and organization
```

### raw operations

```sh
# do something in the package's directory
fkg do gh:MichaelBelousov/forkage git ls-files
fkg do forkage git diff master
```

## Layout

Packages are installed to the nearest `forkage` directory, ascending from the current directory's path.
If there are none, a new `forkage` directory is created in the current directory.

A config file will be searched for in order:
- `.forkageconfig` next to the nearest `forkage` directory ascending the current directory's path.
- `$HOME/.forkageconfig`
- `/etc/forkageconfig`

An example git provider URL of `https://github.com/MichaelBelousov/forkage` would be installed as:

```sh
./forkage/github.com/MichaelBelousov/forkage
```

A unique install (`fkg i --unique gh:MichaelBelousov/forkage@v0.3`) would install to a unique directory:

```sh
./forkage/github.com/MichaelBelousov/forkage@v0.3
```

The URL's path delimiters are translated to file system path delimiters.

### `forkagelist`

Next to the `forkage` directory will be placed a `forkagelist` file capturing the packages in the project.
The format is .ini

<!-- FIXME, define https vs ssh addresses  -->
```ini
[forkage]
  address = https://github.com/MichaelBelousov/forkage
  locked = 0123456789abcdef
  reference = master
  unique = false
[forkage@v0.1]
  address = https://github.com/MichaelBelousov/forkage
  locked = 7654321089dcbaef
  reference = 0.1
  unique = true
```

## Dependencies

Haven't thought this one out yet. Technically if the package uses forkage, we can recurse.

Consider npm dependencies though: a package containing JavaScript source of an npm package could configure a workspace manager
like [`pnpm`](https://pnpm.io) to include all `forkage/*` directories in the workspace so that they participate
in workspace dependency resolution and even build commands.
