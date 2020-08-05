pub const USAGE_INFO: &str = "
❍ Ethereum Vanity Address Generator ❍

    Copyright: Greg Kapka 2020
    Questions: greg@kapka.co.uk

❍ Info ❍

A simple CLI for creating vanity ethereum addresses (I.E. Those that begin with a desired hex string). You can also create random addresses too.

❍ Usage ❍

Usage:  ethereum-vanity-address-generator [--help]
        ethereum-vanity-address-generator version
        ethereum-vanity-address-generator generateRandomAddress [--logLevel=<string>]
        ethereum-vanity-address-generator generateVanityAddress <prefix> [--logLevel=<string>]

Commands:

        version                ❍ Show version info.
        generateRandomAddress  ❍ Generate a random ethereum address.
        generateVanityAddress  ❍ Generate a vanity etheruem address that starts with the provided hex prefix.
        <prefix>               ❍ Valid hex you want as a prefix for your vanity ethereum address.

Options:

        --help                 ❍ Show this message.
        --logLevel=<level>     ❍ Define the level of logging in the tool's output as one of: `none`, `info`, `debug`, `trace`
                                 or `error`. [default: none]

";
