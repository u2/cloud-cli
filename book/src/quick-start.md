# 快速入门

## 配置
### 1. controller和executor的地址

想要与链交互，首先要知道如何访问链。

区块链两个rpc地址，分别是controller和executor微服务。

假设controller的地址为"localhost:50004", executor的地址为"localhost:50002"。

那么我们可以通过`-r`和`-e`来告诉cldi如何访问链：
```bash
# 注意-r和-e必须在子命令之前
$ cldi -r localhost:50004 -e localhost:50002 get block-number
```

### 2. 账户名称

发送交易的命令需要对交易进行签名，我们需要指定签名所使用的账户。
cldi在第一次使用的时候会创建一个名为`default`的默认账户，用户可以通过`-u`来指定账户：
```bash
# 同样地，-u必须在子命令之前
$ cldi -u Alice send --to <to> --value <value> --data <data>
```
创建和导入账户相关的命令请参见[account](cmd/account.md)。

## 使用Context管理配置

每次都指定微服务的访问地址和使用的账户名称不太方便，我们可以通过context命令来管理这些配置。

```bash
# 创建一个Context
$ cldi -r localhost:50004 -e localhost:50002 -u Alice context save Wonderland
# 将这个Context设为默认
$ cldi context default Wonderland
# 也可以使用-c来切换Context，-c也必须在子命令之前
$ cldi -c Wonderland get block-number
# 列出当前Context信息和已保存的Context
$ cldi context list
```

## 交互模式

cldi提供了命令行模式和交互模式，在未传入子命令的时候cldi会进入交互模式。

交互模式与命令行模式的命令是等价的，例如：
```bash
$ cldi get block-number
```
等价于
```bash
$ cldi
cldi> get block-number
```
在交互模式下，用户可以通过`-c`, `-r` `-e`来改变当前会话的Context配置。

```bash
# 修改当前会话的全局配置
cldi> -r localhost:50004
# 仅针对这条命令应用这个配置
cldi> -r localhost:50004 get block-number
```

Q: How to quit cldi?<br>
<del>A: :q</del><br>
A: CTRL-D

## 缩写

cldi提供了很多命令的缩写，这里列举一些：
```plaintext
cldi> get block-number
cldi> get bn

cldi> get system-config
cldi> get sc

cldi> context list
cldi> ctx ls
cldi> ctx l

cldi> account generate --name Alice
cldi> account gen --name Alice
cldi> account g --name Alice
cldi> a g --name Alice

cldi> bench send
cldi> b send

cldi> watch
cldi> w
```

这些缩写仅为方便用户操作，不作稳定性保证，不建议在脚本中使用。

## 命令行模式下的补全

`cldi completions <shell-name>`命令会输出补全脚本，需要添加到, 例如`.profile`, `.bashrc`里才能生效。目前支持的shell有：`bash`, `zsh`, `powershell`, `fish`, `elvish`。

以bash为例，将下列脚本添加到`.bashrc`里即可。
```bash
source <(cldi completions bash)
```


## 使用示例

### 1.生成账户
如果需要更好的安全性，请加上`-p <password>`为私钥进行加密。
有密码的账户在硬盘上会进行加密存储，并且不会在生成时显示明文私钥。
加密后的账户需要经过`-p <password>`解密才能使用。
```plaintext
cldi> account generate --name Alice
{
  "crypto_type": "SM",
  "address": "0xb7768b2f989eeb9a1c7315aa38fb5fbd68333b8a",
  "public_key": "0x325ef60c3d8a94dd363a83f8b9a1ecbe3583b41aa204709eb0d2a19e7e323571d6d4015e5a049bfd04d3ff661385c36fe2066f9aaf72c943ff4ad1fc15e03e73",
  "secret_key": "0x9d08b671a8f12141c45edbd59e81eaf282a2534505ad0545bb46bf64d642b071"
}
```

### 2.创建环境配置
```bash
cldi> -r localhost:50004 -e localhost:50002 -u Alice context save Wonderland
# 设为默认环境
cldi> context default Wonderland
```

### 3.查询块高
```plaintext
cldi> get block-number
406030
```

### 4.查询系统配置
```plaintext
cldi> get system-config
{
  "admin": "0x753dd50f878f08647a0105dd44aa3f4b7cf3408d",
  "admin_pre_hash": "0x000000000000000000000000000000000000000000000000000000000000000000",
  "block_interval": 3,
  "block_interval_pre_hash": "0x000000000000000000000000000000000000000000000000000000000000000000",
  "block_limit": 100,
  "block_limit_pre_hash": "0x000000000000000000000000000000000000000000000000000000000000000000",
  "chain_id": "0x63586a3c0255f337c77a777ff54f0040b8c388da04f23ecee6bfd4953a6512b4",
  "chain_id_pre_hash": "0x000000000000000000000000000000000000000000000000000000000000000000",
  "emergency_brake": false,
  "emergency_brake_pre_hash": "0x000000000000000000000000000000000000000000000000000000000000000000",
  "package_limit": 30000,
  "package_limit_pre_hash": "0x000000000000000000000000000000000000000000000000000000000000000000",
  "validators": [
    "0x74f1bf7351bf97d7217a9232aa0074e303018f7d",
    "0xcee098ece4e372a2f0b4f6bebad04f2111fd1c26",
    "0xfef8db7854b88c6e914a7e21af61b0f485fc3f0d",
    "0xc3bebda27a8f2ae3c8f4d79b656a5cee2db9421c"
  ],
  "validators_pre_hash": "0x000000000000000000000000000000000000000000000000000000000000000000",
  "version": 0,
  "version_pre_hash": "0x000000000000000000000000000000000000000000000000000000000000000000"
}

```

### 5.创建合约

我们通过`cldi create <data>`发送创建合约交易。其中`<data>`是合约的数据，这里以一个计数器合约为例。返回结果为这个创建合约交易的哈希。
```plaintext
cldi> create 0x608060405234801561001057600080fd5b5060f58061001f6000396000f3006080604052600436106053576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806306661abd1460585780634f2be91f146080578063d826f88f146094575b600080fd5b348015606357600080fd5b50606a60a8565b6040518082815260200191505060405180910390f35b348015608b57600080fd5b50609260ae565b005b348015609f57600080fd5b5060a660c0565b005b60005481565b60016000808282540192505081905550565b600080819055505600a165627a7a72305820faa1d1f51d7b5ca2b200e0f6cdef4f2d7e44ee686209e300beb1146f40d32dee0029
0xbdeabf94a31c503deb4400fc63aee2a89e8f43d6570ed7ad5cd4f6f2898be0a2
```

等待交易上链后，通过`cldi get receipt <tx-hash>`获取交易回执，在交易回执中的`contract_addr`项查看创建出来的合约的地址。这里为`0xf064e32407b6cc412fe33f6ba55f578ac413ecdc`


```plaintext
cldi> get receipt 0xbdeabf94a31c503deb4400fc63aee2a89e8f43d6570ed7ad5cd4f6f2898be0a2
{
  "block_number": 406069,
  "contract_addr": "0xf064e32407b6cc412fe33f6ba55f578ac413ecdc",
  "cumulative_quota_used": "0x0000000000000000000000000000000000000000000000000000000000018ed3",
  "error_msg": "",
  "legacy_block_hash": "0x265386a6afc6072f0acb5d32e0fe079e101129041dbfed2bee8872a849e8f7a3",
  "logs": [],
  "logs_bloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
  "quota_used": "0x0000000000000000000000000000000000000000000000000000000000018ed3",
  "state_root": "0x96899ff87f9bed55fd880750c2c51661ee24e14f672a1c9d7ac9573536b8c3f0",
  "tx_hash": "0xbdeabf94a31c503deb4400fc63aee2a89e8f43d6570ed7ad5cd4f6f2898be0a2",
  "tx_index": 0
}
```

### 6.调用合约

查询合约数据。当前计数器的值为0。

```plaintext
cldi> call 0xf064e32407b6cc412fe33f6ba55f578ac413ecdc 0x06661abd
0x0000000000000000000000000000000000000000000000000000000000000000
```

发送交易，使得计数器加一。
```plaintext
cldi> send 0xf064e32407b6cc412fe33f6ba55f578ac413ecdc 0x4f2be91f
0x99e57fdfed555059fa143ad0bc4d8ddc8764f8024fb3b28e880a84667414dec5
```

等待交易上链后，再次查询，可以看到结果为一，符合预期。
```plaintext
cldi> call 0xf064e32407b6cc412fe33f6ba55f578ac413ecdc 0x06661abd
0x0000000000000000000000000000000000000000000000000000000000000001
```
