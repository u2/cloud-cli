# 开发指南

参考`cloud-cli/src/cmd`下的命令实现。

需要异步的话可以从Context里拿到runtime，用这个runtime去block_on即可执行异步任务。

需要client的什么功能，就在Context相应的泛型里加上相应的trait约束。
如果觉得泛型太复杂，也可以把它去掉，把所有泛型都用具体的类型替换掉。