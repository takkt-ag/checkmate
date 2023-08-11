# checkmate - declaratively configure Checkmk

Checkmate is an application that enables declarative configuration of Checkmk. With Checkmate, you can provide a YAML
file that describes the folders, hosts, and rulesets, and then invoke Checkmate on that configuration, which will take
care of pushing the necessary changes to the Checkmk site and activating them.

## Installation

Checkmate is a Rust application, and can be installed using `cargo`. Checkmate is not yet available on crates.io, so
you'll have to clone the repository and install it from there:

```shell
git clone https://github.com/takkt-ag/checkmate.git
cd checkmate
cargo install --path checkmate
```

## Usage

`checkmate`:

```text
Configure checkmk declaratively using checkmate by providing a configuration file

Usage: checkmate <COMMAND>

Commands:
  lint   Link the configuration file to the checkmk site
  apply  Apply the configuration to the checkmk site
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

`checkmate lint`:

```text
Validate the configuration file

Usage: checkmate lint [OPTIONS]

Options:
      --config-file <CONFIG_FILE>  The configuration file to use [env: CHECKMATE_CONFIG_FILE=] [default: checkmate.yaml]
      --print-config               Print the internal repesentation of the configuration file after loading it
  -h, --help                       Print help
```

`checkmate apply`:

```text
Apply the configuration to the checkmk site

Usage: checkmate apply [OPTIONS] --server-url <SERVER_URL> --site <SITE> --secret <SECRET>

Options:
      --server-url <SERVER_URL>
          URL to the checkmk server.

          If checkmk is not running at the root-path, please include the required prefix here.

          [env: CHECKMATE_CHECKMK_SERVER_URL=]

      --site <SITE>
          The checkmk site to configure

          [env: CHECKMATE_CHECKMK_SITE=]

      --username <USERNAME>
          The username to use for authentication

          [env: CHECKMATE_CHECKMK_USERNAME=]
          [default: automation]

      --secret <SECRET>
          The secret to use for authentication.

          You should preferably provide this through the environment variable `CHECKMATE_CHECKMK_SECRET`.

          [env: CHECKMATE_CHECKMK_SECRET=]

      --config-file <CONFIG_FILE>
          The configuration file to use

          [env: CHECKMATE_CONFIG_FILE=]
          [default: checkmate.yaml]

  -h, --help
          Print help (see a summary with '-h')
```

## Missing features

* [ ] Hosts: hosts removed from configuration (i.e. orphaned hosts) are not automatically removed.
    * Workaround: manually remove the host through the Checkmk UI.
* [ ] Folders: folders removed from configuration (i.e. orphaned folders) are not automatically removed.
    * Workaround: manually remove the folder through the Checkmk UI.
* [ ] Rulesets: removal of an entire ruleset will not remove the rules from Checkmk.
    * Workaround: do not remove the entire ruleset from the YAML-file, but define an explicit empty list.
