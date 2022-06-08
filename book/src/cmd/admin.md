# admin

admin相关命令，用于管理链的配置。

```plaintext
$ cldi help admin
cldi-admin
The admin commands for managing chain

USAGE:
    cldi admin <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    update-admin          Update admin of the chain
    update-validators     Update validators of the chain
    set-block-interval    Set block interval
    emergency-brake       Send emergency brake cmd to chain
    set-package-limit     Set package limit
    set-block-limit       Set block limit
    help                  Print this message or the help of the given subcommand(s)
```

这些命令必须以链的管理员账号发送，否则链上会返回错误。具体来说，当前账户的地址必须和链配置的管理员地址一致。

admin下的所有命令都是通过向链发送UTXO交易来完成的。

UTXO交易的数据格式是controller内部定义的。
