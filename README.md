# :trollface: Ethereum Vanity Address Generator

A simple CLI for creating vanity ethereum addresses (I.E. Those that begin with a desired hex string). You can also create random addresses too.

&nbsp;

***

&nbsp;

## __:page_with_curl: Usage:

**1)** Clone the repo:

_**`❍ git clone https://github.com/gskapka/ethereum-vanity-address-generator.git`**_

**2)** Enter app dir and build it:

_**`❍ cd  ethereum-vanity-address-generator && cargo b --release`**_

**3)** You'll find the __`ethereum-vanity-address-generator`__binary in:

_**`❍ cd ./target/release`**_

**4)** To use the binary itself see the following:

```
❍ Ethereum Vanity Address Generator ❍

    Copyright: Greg Kapka 2020
    Questions: greg@kapka.co.uk

❍ Info ❍

A simple CLI for creating vanity ethereum addresses (I.E. Those that begin with a desired hex string). You can also create random addresses too.

❍ Usage ❍

Usage:  ethereum-vanity-address-generator [--help]
        ethereum-vanity-address-generator version
        ethereum-vanity-address-generator generateAddress [--logLevel=<string>]
        ethereum-vanity-address-generator generateVanityAddress <prefix> [--logLevel=<string>]

Commands:

    version                ❍ Show version info.
    generateAddress        ❍ Generate a random ethereum address.
    generateVanityAddress  ❍ Generate a vanity etheruem address that starts with the provided hex prefix.
    <prefix>               ❍ Valid hex you want as a prefix for your vanity ethereum address.

Options:

    --help                 ❍ Show this message.
    --logLevel=<level>     ❍ Define the level of logging in the tool's output as one of: `none`, `info`, `debug`, `trace`
                             or `error`. [default: none]

```

&nbsp;

## __:black_nib: Notes:__

__❍__


## __:clipboard: To Do List:__

[ ] Make as CLI w/ Docopt
[ ] All custom message passing in for signing
